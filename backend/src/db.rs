use sqlx::{SqlitePool, Row};
use anyhow::Result;
use chrono::Utc;

pub async fn init_db(pool: &SqlitePool) -> Result<()> {
    // Enable WAL mode
    sqlx::query("PRAGMA journal_mode=WAL").execute(pool).await?;
    sqlx::query("PRAGMA foreign_keys=ON").execute(pool).await?;

    Ok(())
}

pub async fn run_migrations(pool: &SqlitePool) -> Result<()> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}

// Agent operations
pub async fn create_agent(
    pool: &SqlitePool,
    id: &str,
    name: &str,
    description: Option<&str>,
    endpoint_url: &str,
    capabilities: &[String],
    status: &str,
    metadata: Option<&serde_json::Value>,
) -> Result<()> {
    let capabilities_json = serde_json::to_string(capabilities)?;
    let metadata_json = metadata
        .as_ref()
        .map(|v| serde_json::to_string(v).transpose())
        .transpose()?;

    sqlx::query(
        r#"
        INSERT INTO agents (id, name, description, endpoint_url, capabilities, status, metadata, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(id)
    .bind(name)
    .bind(description)
    .bind(endpoint_url)
    .bind(&capabilities_json)
    .bind(status)
    .bind(metadata_json.as_deref())
    .bind(Utc::now())
    .bind(Utc::now())
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_agent(pool: &SqlitePool, id: &str) -> Result<Option<serde_json::Value>> {
    let row = sqlx::query("SELECT * FROM agents WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?;

    Ok(row.map(|r| {
        serde_json::json!({
            "id": r.get::<String, _>("id"),
            "name": r.get::<String, _>("name"),
            "description": r.get::<Option<String>, _>("description"),
            "endpoint_url": r.get::<String, _>("endpoint_url"),
            "capabilities": serde_json::from_str::<Vec<String>>(&r.get::<String, _>("capabilities")).unwrap_or_default(),
            "status": r.get::<String, _>("status"),
            "metadata": r.get::<Option<serde_json::Value>, _>("metadata"),
            "created_at": r.get::<String, _>("created_at"),
            "updated_at": r.get::<String, _>("updated_at"),
        })
    }))
}

pub async fn list_agents(pool: &SqlitePool, status: Option<&str>) -> Result<Vec<serde_json::Value>> {
    let mut query = "SELECT * FROM agents".to_string();
    if let Some(status) = status {
        query.push_str(&format!(" WHERE status = '{}'", status));
    }
    query.push_str(" ORDER BY created_at DESC");

    let rows = sqlx::query(&query).fetch_all(pool).await?;

    let mut agents = Vec::new();
    for row in rows {
        agents.push(serde_json::json!({
            "id": row.get::<String, _>("id"),
            "name": row.get::<String, _>("name"),
            "description": row.get::<Option<String>, _>("description"),
            "endpoint_url": row.get::<String, _>("endpoint_url"),
            "capabilities": serde_json::from_str::<Vec<String>>(&row.get::<String, _>("capabilities")).unwrap_or_default(),
            "status": row.get::<String, _>("status"),
            "metadata": row.get::<Option<serde_json::Value>, _>("metadata"),
            "created_at": row.get::<String, _>("created_at"),
            "updated_at": row.get::<String, _>("updated_at"),
        }));
    }

    Ok(agents)
}

pub async fn update_agent(
    pool: &SqlitePool,
    id: &str,
    description: Option<&str>,
    endpoint_url: Option<&str>,
    capabilities: Option<&Vec<String>>,
    status: Option<&str>,
    metadata: Option<&serde_json::Value>,
) -> Result<()> {
    let mut sets = Vec::new();
    let mut query = sqlx::QueryBuilder::new("UPDATE agents SET ");

    if let Some(description) = description {
        query.push("description = ");
        query.push_bind(description);
        sets.push(true);
    }

    if let Some(endpoint_url) = endpoint_url {
        if sets.last().map(|b| *b).unwrap_or(false) {
            query.push(", ");
        }
        query.push("endpoint_url = ");
        query.push_bind(endpoint_url);
        sets.push(true);
    }

    if let Some(capabilities) = capabilities {
        if sets.last().map(|b| *b).unwrap_or(false) {
            query.push(", ");
        }
        query.push("capabilities = ");
        query.push_bind(&serde_json::to_string(capabilities)?);
        sets.push(true);
    }

    if let Some(status) = status {
        if sets.last().map(|b| *b).unwrap_or(false) {
            query.push(", ");
        }
        query.push("status = ");
        query.push_bind(status);
        sets.push(true);
    }

    if let Some(metadata) = metadata {
        if sets.last().map(|b| *b).unwrap_or(false) {
            query.push(", ");
        }
        query.push("metadata = ");
        query.push_bind(&serde_json::to_string(metadata)?);
        sets.push(true);
    }

    if !sets.is_empty() {
        query.push(", updated_at = ");
        query.push_bind(Utc::now());
        query.push(" WHERE id = ");
        query.push_bind(id);

        query.build().execute(pool).await?;
    }

    Ok(())
}

pub async fn delete_agent(pool: &SqlitePool, id: &str) -> Result<()> {
    sqlx::query("DELETE FROM agents WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn list_capabilities(pool: &SqlitePool) -> Result<Vec<String>> {
    let rows = sqlx::query("SELECT capabilities FROM agents WHERE status != 'inactive'")
        .fetch_all(pool)
        .await?;

    let mut capabilities = Vec::new();
    for row in rows {
        if let Ok(caps_json) = row.get::<String, _>("capabilities") {
            if let Ok(caps) = serde_json::from_str::<Vec<String>>(&caps_json) {
                capabilities.extend(caps);
            }
        }
    }

    capabilities.sort();
    capabilities.dedup();

    Ok(capabilities)
}

// Task operations
pub async fn create_task(
    pool: &SqlitePool,
    id: &str,
    user_query: &str,
) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO tasks (id, user_query, status, created_at)
        VALUES (?, ?, ?, ?)
        "#,
    )
    .bind(id)
    .bind(user_query)
    .bind("pending")
    .bind(Utc::now())
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn update_task_status(
    pool: &SqlitePool,
    id: &str,
    status: &str,
    error_message: Option<&str>,
) -> Result<()> {
    let now = Utc::now();
    let mut query = sqlx::QueryBuilder::new("UPDATE tasks SET status = ");

    query.push_bind(status);

    if matches!(status, "dispatched" | "completed" | "failed") {
        query.push(", started_at = ");
        query.push_bind(now);
    }
    if matches!(status, "completed" | "failed" | "cancelled") {
        query.push(", completed_at = ");
        query.push_bind(now);
    }

    if let Some(error) = error_message {
        query.push(", error_message = ");
        query.push_bind(error);
    }

    query.push(" WHERE id = ");
    query.push_bind(id);

    query.build().execute(pool).await?;
    Ok(())
}

pub async fn get_task(pool: &SqlitePool, id: &str) -> Result<Option<serde_json::Value>> {
    let row = sqlx::query("SELECT * FROM tasks WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?;

    Ok(row.map(|r| {
        serde_json::json!({
            "id": r.get::<String, _>("id"),
            "user_query": r.get::<String, _>("user_query"),
            "parsed_plan": r.get::<Option<serde_json::Value>, _>("parsed_plan"),
            "status": r.get::<String, _>("status"),
            "error_message": r.get::<Option<String>, _>("error_message"),
            "created_at": r.get::<String, _>("created_at"),
            "started_at": r.get::<Option<String>, _>("started_at"),
            "completed_at": r.get::<Option<String>, _>("completed_at"),
        })
    }))
}

pub async fn list_tasks(
    pool: &SqlitePool,
    status: Option<&str>,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<serde_json::Value>> {
    let mut query = "SELECT * FROM tasks".to_string();

    if let Some(status) = status {
        query.push_str(&format!(" WHERE status = '{}'", status));
    }

    query.push_str(" ORDER BY created_at DESC");

    if let Some(limit) = limit {
        query.push_str(&format!(" LIMIT {}", limit));
        if let Some(offset) = offset {
            query.push_str(&format!(" OFFSET {}", offset));
        }
    }

    let rows = sqlx::query(&query).fetch_all(pool).await?;

    let mut tasks = Vec::new();
    for row in rows {
        tasks.push(serde_json::json!({
            "id": row.get::<String, _>("id"),
            "user_query": row.get::<String, _>("user_query"),
            "status": row.get::<String, _>("status"),
            "created_at": row.get::<String, _>("created_at"),
            "started_at": row.get::<Option<String>, _>("started_at"),
            "completed_at": row.get::<Option<String>, _>("completed_at"),
        }));
    }

    Ok(tasks)
}

// SubTask operations
pub async fn create_sub_task(
    pool: &SqlitePool,
    id: &str,
    task_id: &str,
    agent_id: &str,
    capability: &str,
    input: Option<&serde_json::Value>,
) -> Result<()> {
    let input_json = input.map(|v| serde_json::to_string(v)).transpose()?;

    sqlx::query(
        r#"
        INSERT INTO sub_tasks (id, task_id, agent_id, capability, input, status)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(id)
    .bind(task_id)
    .bind(agent_id)
    .bind(capability)
    .bind(input_json.as_deref())
    .bind("pending")
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn update_sub_task(
    pool: &SqlitePool,
    id: &str,
    status: &str,
    output: Option<&serde_json::Value>,
    error: Option<&str>,
    latency_ms: Option<i32>,
) -> Result<()> {
    let now = Utc::now();

    let output_json = output.map(|v| serde_json::to_string(v)).transpose()?;

    let mut query = sqlx::QueryBuilder::new("UPDATE sub_tasks SET status = ");
    query.push_bind(status);

    if matches!(status, "completed" | "failed") {
        query.push(", completed_at = ");
        query.push_bind(now);
    }
    if output_json.is_some() {
        query.push(", output = ");
        query.push_bind(output_json.as_deref());
    }
    if let Some(error) = error {
        query.push(", error = ");
        query.push_bind(error);
    }
    if let Some(latency) = latency_ms {
        query.push(", latency_ms = ");
        query.push_bind(latency);
    }

    query.push(" WHERE id = ");
    query.push_bind(id);

    query.build().execute(pool).await?;
    Ok(())
}

pub async fn get_sub_tasks_for_task(pool: &SqlitePool, task_id: &str) -> Result<Vec<serde_json::Value>> {
    let rows = sqlx::query(
        "SELECT st.*, a.name as agent_name FROM sub_tasks st LEFT JOIN agents a ON st.agent_id = a.id WHERE st.task_id = ? ORDER BY st.id"
    )
    .bind(task_id)
    .fetch_all(pool)
    .await?;

    let mut sub_tasks = Vec::new();
    for row in rows {
        sub_tasks.push(serde_json::json!({
            "id": row.get::<String, _>("id"),
            "task_id": row.get::<String, _>("task_id"),
            "agent_id": row.get::<String, _>("agent_id"),
            "agent_name": row.get::<Option<String>, _>("agent_name"),
            "capability": row.get::<String, _>("capability"),
            "input": row.get::<Option<serde_json::Value>, _>("input"),
            "output": row.get::<Option<serde_json::Value>, _>("output"),
            "error": row.get::<Option<String>, _>("error"),
            "status": row.get::<String, _>("status"),
            "started_at": row.get::<Option<String>, _>("started_at"),
            "completed_at": row.get::<Option<String>, _>("completed_at"),
            "latency_ms": row.get::<Option<i32>, _>("latency_ms"),
        }));
    }

    Ok(sub_tasks)
}

// Execution logging
pub async fn log_execution(
    pool: &SqlitePool,
    task_id: &str,
    agent_id: Option<&str>,
    step: i32,
    action: &str,
    input_snapshot: Option<&serde_json::Value>,
    output_snapshot: Option<&serde_json::Value>,
    latency_ms: Option<i32>,
    success: bool,
) -> Result<()> {
    let input_json = input_snapshot.map(|v| serde_json::to_string(v)).transpose()?;
    let output_json = output_snapshot.map(|v| serde_json::to_string(v)).transpose()?;

    sqlx::query(
        r#"
        INSERT INTO executions (id, task_id, agent_id, step, action, input_snapshot, output_snapshot, latency_ms, success, timestamp)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(uuid::Uuid::new_v4().to_string())
    .bind(task_id)
    .bind(agent_id)
    .bind(step)
    .bind(action)
    .bind(input_json.as_deref())
    .bind(output_json.as_deref())
    .bind(latency_ms)
    .bind(success)
    .bind(Utc::now())
    .execute(pool)
    .await?;

    Ok(())
}

// Performance tracking
pub async fn update_agent_performance(
    pool: &SqlitePool,
    agent_id: &str,
    capability: &str,
    success: bool,
    latency_ms: Option<i32>,
) -> Result<()> {
    // Get or create performance record
    let row = sqlx::query(
        "SELECT * FROM agent_performance WHERE agent_id = ? AND capability = ?"
    )
    .bind(agent_id)
    .bind(capability)
    .fetch_optional(pool)
    .await?;

    if let Some(row) = row {
        let success_count: i64 = row.get("success_count");
        let failure_count: i64 = row.get("failure_count");
        let avg_latency: Option<f64> = row.get("avg_latency_ms");

        let (new_success, new_failure) = if success {
            (success_count + 1, failure_count)
        } else {
            (success_count, failure_count + 1)
        };

        let new_avg_latency = match (avg_latency, latency_ms) {
            (Some(old_avg), Some(latency)) => {
                let total = (new_success + new_failure - 1) as f64 * old_avg + latency as f64;
                total / (new_success + new_failure) as f64
            }
            _ => latency_ms.map(|l| l as f64),
        };

        sqlx::query(
            "UPDATE agent_performance SET success_count = ?, failure_count = ?, avg_latency_ms = ?, last_updated = ? WHERE agent_id = ? AND capability = ?"
        )
        .bind(new_success)
        .bind(new_failure)
        .bind(new_avg_latency)
        .bind(Utc::now())
        .bind(agent_id)
        .bind(capability)
        .execute(pool)
        .await?;
    } else {
        let (success_count, failure_count) = if success {
            (1, 0)
        } else {
            (0, 1)
        };

        sqlx::query(
            "INSERT INTO agent_performance (id, agent_id, capability, success_count, failure_count, avg_latency_ms, last_updated) VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(uuid::Uuid::new_v4().to_string())
        .bind(agent_id)
        .bind(capability)
        .bind(success_count)
        .bind(failure_count)
        .bind(latency_ms.map(|l| l as f64))
        .bind(Utc::now())
        .execute(pool)
        .await?;
    }

    Ok(())
}

pub async fn get_agent_performance(pool: &SqlitePool, agent_id: &str) -> Result<Vec<AgentPerformance>> {
    let rows = sqlx::query(
        "SELECT * FROM agent_performance WHERE agent_id = ? ORDER BY last_updated DESC"
    )
    .bind(agent_id)
    .fetch_all(pool)
    .await?;

    let mut performance = Vec::new();
    for row in rows {
        performance.push(AgentPerformance {
            id: row.get("id"),
            agent_id: row.get("agent_id"),
            capability: row.get("capability"),
            success_count: row.get("success_count"),
            failure_count: row.get("failure_count"),
            avg_latency_ms: row.get("avg_latency_ms"),
            last_updated: row.get("last_updated"),
        });
    }

    Ok(performance)
}