use sqlx::{Sqlite, SqlitePool, Transaction};
use crate::models::*;

pub struct Repository {
    pool: SqlitePool,
}

impl Repository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn begin_transaction(&self) -> Result<Transaction<'_, Sqlite>, sqlx::Error> {
        self.pool.begin().await
    }

    // ===== AGENTS =====

    pub async fn create_agent(&self, agent: &NewAgent) -> Result<Agent, sqlx::Error> {
        let agent = Agent::new(
            agent.name.clone(),
            agent.description.clone(),
            agent.endpoint_url.clone(),
            agent.capabilities.clone(),
            agent.metadata.clone(),
        );

        sqlx::query!(
            r#"
            INSERT INTO agents (id, name, description, endpoint_url, capabilities, status, metadata, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            "#,
            agent.id,
            agent.name,
            agent.description,
            agent.endpoint_url,
            serde_json::to_string(&agent.capabilities).unwrap(),
            agent.status.to_string(),
            agent.metadata.as_ref().map(|m| serde_json::to_string(m).unwrap()),
            agent.created_at,
            agent.updated_at,
        )
        .execute(&self.pool)
        .await?;

        Ok(agent)
    }

    pub async fn get_agent(&self, id: &str) -> Result<Option<Agent>, sqlx::Error> {
        let row = sqlx::query!(
            "SELECT * FROM agents WHERE id = ?",
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| Agent {
            id: r.id,
            name: r.name,
            description: r.description,
            endpoint_url: r.endpoint_url,
            capabilities: serde_json::from_str(&r.capabilities).unwrap_or_default(),
            status: match r.status.as_str() {
                "active" => AgentStatus::Active,
                "inactive" => AgentStatus::Inactive,
                "error" => AgentStatus::Error,
                _ => AgentStatus::Inactive,
            },
            metadata: r.metadata.and_then(|m| serde_json::from_str(&m).ok()),
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    }

    pub async fn list_agents(&self, status: Option<&str>) -> Result<Vec<Agent>, sqlx::Error> {
        let rows = if let Some(status) = status {
            sqlx::query!("SELECT * FROM agents WHERE status = ?", status)
        } else {
            sqlx::query!("SELECT * FROM agents")
        }
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| Agent {
                id: r.id,
                name: r.name,
                description: r.description,
                endpoint_url: r.endpoint_url,
                capabilities: serde_json::from_str(&r.capabilities).unwrap_or_default(),
                status: match r.status.as_str() {
                    "active" => AgentStatus::Active,
                    "inactive" => AgentStatus::Inactive,
                    "error" => AgentStatus::Error,
                    _ => AgentStatus::Inactive,
                },
                metadata: r.metadata.and_then(|m| serde_json::from_str(&m).ok()),
                created_at: r.created_at,
                updated_at: r.updated_at,
            })
            .collect())
    }

    pub async fn update_agent_status(&self, id: &str, status: AgentStatus) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE agents SET status = ?, updated_at = ? WHERE id = ?",
            status.to_string(),
            Utc::now(),
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn delete_agent(&self, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM agents WHERE id = ?", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_all_capabilities(&self) -> Result<Vec<String>, sqlx::Error> {
        let rows = sqlx::query!("SELECT capabilities FROM agents")
            .fetch_all(&self.pool)
            .await?;

        let mut capabilities = Vec::new();
        for row in rows {
            if let Ok(caps) = serde_json::from_str::<Vec<String>>(&row.capabilities) {
                capabilities.extend(caps);
            }
        }
        capabilities.sort();
        capabilities.dedup();
        Ok(capabilities)
    }

    // ===== TASKS =====

    pub async fn create_task(&self, task: &NewTask) -> Result<UserTask, sqlx::Error> {
        let user_task = UserTask::new(task.user_query.clone());

        sqlx::query!(
            r#"
            INSERT INTO tasks (id, user_query, parsed_plan, status, error_message, created_at, started_at, completed_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            "#,
            user_task.id,
            user_task.user_query,
            serde_json::to_string(&user_task.parsed_plan).unwrap(),
            user_task.status.to_string(),
            user_task.error_message,
            user_task.created_at,
            user_task.started_at,
            user_task.completed_at,
        )
        .execute(&self.pool)
        .await?;

        Ok(user_task)
    }

    pub async fn get_task(&self, id: &str) -> Result<Option<UserTask>, sqlx::Error> {
        let row = sqlx::query!("SELECT * FROM tasks WHERE id = ?", id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|r| UserTask {
            id: r.id,
            user_query: r.user_query,
            parsed_plan: r.parsed_plan.and_then(|p| serde_json::from_str(&p).ok()),
            status: match r.status.as_str() {
                "pending" => TaskStatus::Pending,
                "dispatched" => TaskStatus::Dispatched,
                "completed" => TaskStatus::Completed,
                "failed" => TaskStatus::Failed,
                "cancelled" => TaskStatus::Cancelled,
                _ => TaskStatus::Pending,
            },
            error_message: r.error_message,
            created_at: r.created_at,
            started_at: r.started_at,
            completed_at: r.completed_at,
        }))
    }

    pub async fn list_tasks(
        &self,
        status: Option<&str>,
        agent_id: Option<&str>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<UserTask>, sqlx::Error> {
        let mut query = String::from("SELECT * FROM tasks");
        let mut params = Vec::new();

        if let Some(status) = status {
            query.push_str(" WHERE status = ?");
            params.push(status.to_string());
        }

        if let Some(agent_id) = agent_id {
            if params.is_empty() {
                query.push_str(" WHERE");
            } else {
                query.push_str(" AND");
            }
            query.push_str(" id IN (SELECT task_id FROM sub_tasks WHERE agent_id = ?)");
            params.push(agent_id.to_string());
        }

        query.push_str(" ORDER BY created_at DESC");

        if let Some(limit) = limit {
            query.push_str(" LIMIT ?");
            params.push(limit.to_string());
        }

        if let Some(offset) = offset {
            query.push_str(" OFFSET ?");
            params.push(offset.to_string());
        }

        let mut query_builder = sqlx::query(&query);
        for param in params {
            query_builder = query_builder.bind(param);
        }

        let rows = query_builder.fetch_all(&self.pool).await?;

        Ok(rows
            .into_iter()
            .map(|r| UserTask {
                id: r.id,
                user_query: r.user_query,
                parsed_plan: r.parsed_plan.and_then(|p| serde_json::from_str(&p).ok()),
                status: match r.status.as_str() {
                    "pending" => TaskStatus::Pending,
                    "dispatched" => TaskStatus::Dispatched,
                    "completed" => TaskStatus::Completed,
                    "failed" => TaskStatus::Failed,
                    "cancelled" => TaskStatus::Cancelled,
                    _ => TaskStatus::Pending,
                },
                error_message: r.error_message,
                created_at: r.created_at,
                started_at: r.started_at,
                completed_at: r.completed_at,
            })
            .collect())
    }

    pub async fn update_task_status(
        &self,
        id: &str,
        status: TaskStatus,
        error_message: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        let now = Utc::now();
        let started_at = if status == TaskStatus::Dispatched && !matches!(status, TaskStatus::Pending) {
            Some(now)
        } else {
            None
        };
        let completed_at = if matches!(status, TaskStatus::Completed | TaskStatus::Failed | TaskStatus::Cancelled) {
            Some(now)
        } else {
            None
        };

        sqlx::query!(
            r#"
            UPDATE tasks 
            SET status = ?, error_message = ?, started_at = ?, completed_at = ?
            WHERE id = ?
            "#,
            status.to_string(),
            error_message,
            started_at,
            completed_at,
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn update_task_plan(&self, id: &str, plan: &serde_json::Value) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE tasks SET parsed_plan = ? WHERE id = ?",
            serde_json::to_string(plan).unwrap(),
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    // ===== SUB TASKS =====

    pub async fn create_sub_task(&self, sub_task: &NewSubTask) -> Result<SubTask, sqlx::Error> {
        let sub_task = SubTask::new(
            sub_task.task_id.clone(),
            sub_task.agent_id.clone(),
            sub_task.capability.clone(),
            sub_task.input.clone(),
        );

        sqlx::query!(
            r#"
            INSERT INTO sub_tasks (id, task_id, agent_id, capability, input, output, error, status, started_at, completed_at, latency_ms)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
            "#,
            sub_task.id,
            sub_task.task_id,
            sub_task.agent_id,
            sub_task.capability,
            sub_task.input.as_ref().map(|i| serde_json::to_string(i).unwrap()),
            sub_task.output.as_ref().map(|o| serde_json::to_string(o).unwrap()),
            sub_task.error,
            sub_task.status.to_string(),
            sub_task.started_at,
            sub_task.completed_at,
            sub_task.latency_ms,
        )
        .execute(&self.pool)
        .await?;

        Ok(sub_task)
    }

    pub async fn get_sub_tasks_for_task(&self, task_id: &str) -> Result<Vec<SubTask>, sqlx::Error> {
        let rows = sqlx::query!(
            "SELECT * FROM sub_tasks WHERE task_id = ? ORDER BY created_at ASC",
            task_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| SubTask {
                id: r.id,
                task_id: r.task_id,
                agent_id: r.agent_id,
                capability: r.capability,
                input: r.input.and_then(|i| serde_json::from_str(&i).ok()),
                output: r.output.and_then(|o| serde_json::from_str(&o).ok()),
                error: r.error,
                status: match r.status.as_str() {
                    "pending" => SubTaskStatus::Pending,
                    "running" => SubTaskStatus::Running,
                    "completed" => SubTaskStatus::Completed,
                    "failed" => SubTaskStatus::Failed,
                    _ => SubTaskStatus::Pending,
                },
                started_at: r.started_at,
                completed_at: r.completed_at,
                latency_ms: r.latency_ms,
            })
            .collect())
    }

    pub async fn update_sub_task_status(
        &self,
        id: &str,
        status: SubTaskStatus,
        output: Option<&serde_json::Value>,
        error: Option<&str>,
        latency_ms: Option<i64>,
    ) -> Result<(), sqlx::Error> {
        let now = Utc::now();
        let started_at = if status == SubTaskStatus::Running {
            Some(now)
        } else {
            None
        };
        let completed_at = if matches!(status, SubTaskStatus::Completed | SubTaskStatus::Failed) {
            Some(now)
        } else {
            None
        };

        sqlx::query!(
            r#"
            UPDATE sub_tasks 
            SET status = ?, output = ?, error = ?, started_at = ?, completed_at = ?, latency_ms = ?
            WHERE id = ?
            "#,
            status.to_string(),
            output.as_ref().map(|o| serde_json::to_string(o).unwrap()),
            error,
            started_at,
            completed_at,
            latency_ms,
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    // ===== EXECUTIONS =====

    pub async fn create_execution(
        &self,
        execution: &Execution,
        tx: Option<&mut Transaction<'_, Sqlite>>,
    ) -> Result<(), sqlx::Error> {
        let tx = if let Some(tx) = tx {
            tx
        } else {
            &mut self.pool.begin().await?
        };

        sqlx::query!(
            r#"
            INSERT INTO executions (id, task_id, agent_id, step, action, input_snapshot, output_snapshot, latency_ms, success, timestamp)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
            "#,
            execution.id,
            execution.task_id,
            execution.agent_id,
            execution.step,
            execution.action,
            execution.input_snapshot.as_ref().map(|s| serde_json::to_string(s).unwrap()),
            execution.output_snapshot.as_ref().map(|s| serde_json::to_string(s).unwrap()),
            execution.latency_ms,
            execution.success,
            execution.timestamp,
        )
        .execute(tx)
        .await?;

        Ok(())
    }

    pub async fn get_executions_for_task(
        &self,
        task_id: &str,
        limit: Option<i64>,
    ) -> Result<Vec<Execution>, sqlx::Error> {
        let mut query = String::from(
            "SELECT * FROM executions WHERE task_id = ? ORDER BY step ASC, timestamp ASC"
        );
        let mut params = vec![task_id.to_string()];

        if let Some(limit) = limit {
            query.push_str(" LIMIT ?");
            params.push(limit.to_string());
        }

        let mut query_builder = sqlx::query(&query);
        for param in params {
            query_builder = query_builder.bind(param);
        }

        let rows = query_builder.fetch_all(&self.pool).await?;

        Ok(rows
            .into_iter()
            .map(|r| Execution {
                id: r.id,
                task_id: r.task_id,
                agent_id: r.agent_id,
                step: r.step,
                action: r.action,
                input_snapshot: r.input_snapshot.and_then(|s| serde_json::from_str(&s).ok()),
                output_snapshot: r.output_snapshot.and_then(|s| serde_json::from_str(&s).ok()),
                latency_ms: r.latency_ms,
                success: r.success,
                timestamp: r.timestamp,
            })
            .collect())
    }

    pub async fn get_execution_stats(&self) -> Result<ExecutionStats, sqlx::Error> {
        let total = sqlx::query!("SELECT COUNT(*) as count FROM executions")
            .fetch_one(&self.pool)
            .await?;

        let success_count = sqlx::query!("SELECT COUNT(*) as count FROM executions WHERE success = 1")
            .fetch_one(&self.pool)
            .await?;

        let total_executions = total.count.unwrap_or(0);
        let success_count = success_count.count.unwrap_or(0);
        let success_rate = if total_executions > 0 {
            success_count as f64 / total_executions as f64 * 100.0
        } else {
            0.0
        };

        let avg_latency = sqlx::query!(
            "SELECT AVG(latency_ms) as avg FROM executions WHERE latency_ms IS NOT NULL"
        )
        .fetch_one(&self.pool)
        .await?;

        let by_agent = sqlx::query!(
            r#"
            SELECT 
                e.agent_id,
                a.name as agent_name,
                COUNT(*) as total_tasks,
                AVG(e.latency_ms) as avg_latency,
                SUM(CASE WHEN e.success = 1 THEN 1 ELSE 0 END) as success_count
            FROM executions e
            LEFT JOIN agents a ON e.agent_id = a.id
            WHERE e.agent_id IS NOT NULL
            GROUP BY e.agent_id, a.name
            ORDER BY total_tasks DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(ExecutionStats {
            total_executions,
            success_rate,
            avg_latency_ms: avg_latency.avg.unwrap_or(0.0),
            by_agent: by_agent
                .into_iter()
                .map(|r| AgentStats {
                    agent_id: r.agent_id,
                    agent_name: r.agent_name,
                    total_tasks: r.total_tasks,
                    success_rate: if r.total_tasks > 0 {
                        r.success_count as f64 / r.total_tasks as f64 * 100.0
                    } else {
                        0.0
                    },
                    avg_latency_ms: r.avg_latency.unwrap_or(0.0),
                })
                .collect(),
        })
    }
}
