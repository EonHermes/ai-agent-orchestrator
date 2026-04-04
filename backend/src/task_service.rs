use sqlx::{SqlitePool, Row};
use anyhow::{Result, Context};
use tokio::time::{timeout, Duration};
use tracing::{info, warn, error};

use crate::db;
use crate::models::{Task, TaskStatus, SubTask, SubTaskStatus, Agent, PlanStep};
use crate::llm_service::LlmService;
use crate::agent_service::AgentService;
use reqwest::Client;

pub struct TaskService {
    pool: SqlitePool,
    llm_service: LlmService,
    agent_service: AgentService,
    http_client: Client,
    max_concurrent_tasks: usize,
}

impl TaskService {
    pub fn new(
        pool: SqlitePool,
        llm_service: LlmService,
        agent_service: AgentService,
        max_concurrent_tasks: usize,
    ) -> Self {
        Self {
            pool,
            llm_service,
            agent_service,
            http_client: Client::new(),
            max_concurrent_tasks,
        }
    }

    pub async fn create_and_execute_task(&self, user_query: &str) -> Result<String> {
        let task_id = uuid::Uuid::new_v4().to_string();

        // Create task record
        db::create_task(&self.pool, &task_id, user_query).await?;

        info!("Created task {} with query: {}", task_id, user_query);

        // Execute in background (don't await to avoid blocking)
        let task_service = self.clone();
        tokio::spawn(async move {
            if let Err(e) = task_service.execute_task(&task_id).await {
                error!("Task {} failed: {}", task_id, e);
                let _ = db::update_task_status(&task_service.pool, &task_id, "failed", Some(&e.to_string())).await;
            }
        });

        Ok(task_id)
    }

    async fn execute_task(&self, task_id: &str) -> Result<()> {
        // Mark as dispatched
        db::update_task_status(&self.pool, task_id, "dispatched", None).await?;

        // Get task details
        let task = db::get_task(&self.pool, task_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Task not found: {}", task_id))?;

        let user_query = task["user_query"].as_str().unwrap_or_default();

        // Step 1: Parse the natural language request
        info!("Parsing task {}: {}", task_id, user_query);

        // Get all active capabilities for parsing context
        let capabilities = db::list_capabilities(&self.pool).await?;
        let parse_result = self.llm_service.parse_task(user_query, &capabilities).await?;

        info!("Task {} parsed into {} steps", task_id, parse_result.plan.len());

        // Store parsed plan
        if let Ok(plan_json) = serde_json::to_value(parse_result.plan.clone()) {
            sqlx::query("UPDATE tasks SET parsed_plan = ? WHERE id = ?")
                .bind(plan_json.to_string())
                .bind(task_id)
                .execute(&self.pool)
                .await?;
        }

        // Step 2: For each step, find and assign an agent
        info!("Dispatching {} sub-tasks", parse_result.plan.len());

        let mut assignments = Vec::new();

        for (index, step) in parse_result.plan.iter().enumerate() {
            let sub_task_id = uuid::Uuid::new_v4().to_string();

            // Find agents for this capability
            let agents = self.agent_service.find_agents_for_capability(&step.capability).await?;

            let assigned_agent = if agents.is_empty() {
                warn!("No agents found for capability: {}", step.capability);
                // Create a placeholder sub-task with error
                db::create_sub_task(&self.pool, &sub_task_id, task_id, "", &step.capability, None).await?;
                db::update_sub_task(
                    &self.pool,
                    &sub_task_id,
                    "failed",
                    None,
                    Some(&format!("No agent available for capability: {}", step.capability)),
                    None,
                ).await?;
                None
            } else {
                // Select best agent (currently first active)
                let agent = &agents[0];

                info!("Assigning step {} to agent '{}' (capability: {})", index, agent.name, step.capability);

                // Store input data
                let input_json = serde_json::json!({
                    "query": user_query,
                    "step_description": step.description,
                    "capability": step.capability,
                    "input_schema": step.input_schema,
                    "step_index": index,
                    "total_steps": parse_result.plan.len(),
                });

                db::create_sub_task(
                    &self.pool,
                    &sub_task_id,
                    task_id,
                    &agent.id,
                    &step.capability,
                    Some(&input_json),
                ).await?;

                Some((sub_task_id, agent.clone()))
            };

            if let Some((ref sub_task_id, ref agent)) = assigned_agent {
                assignments.push((sub_task_id.clone(), agent.clone()));
            }
        }

        // Step 3: Execute sub-tasks in parallel (with concurrency limit)
        info!("Starting {} sub-task executions", assignments.len());

        let mut execution_handles = Vec::new();

        for (sub_task_id, agent) in assignments {
            let task_service = self.clone();
            let pool = self.pool.clone();
            let http_client = self.http_client.clone();

            let handle = tokio::spawn(async move {
                let start_time = std::time::Instant::now();

                // Update sub-task status to running
                if let Err(e) = db::update_sub_task(&pool, &sub_task_id, "running", None, None, None).await {
                    error!("Failed to update sub_task {} status: {}", sub_task_id, e);
                    return;
                }

                // Build request to agent
                let sub_task = db::get_sub_tasks_for_task(&pool, &task_id)
                    .await?
                    .into_iter()
                    .find(|st| st["id"] == sub_task_id)
                    .ok_or_else(|| anyhow::anyhow!("Sub-task not found: {}", sub_task_id))?;

                let input = sub_task["input"].clone();

                let request_body = serde_json::json!({
                    "task_id": task_id,
                    "sub_task_id": sub_task_id,
                    "capability": sub_task["capability"],
                    "input": input,
                });

                info!("Calling agent {} at {}", agent.name, agent.endpoint_url);

                // Call agent endpoint
                let response = timeout(
                    Duration::from_secs(120), // 2 minute timeout per agent
                    http_client
                        .post(&format!("{}/execute", agent.endpoint_url))
                        .json(&request_body)
                        .send()
                ).await;

                let latency_ms = start_time.elapsed().as_millis() as i32;
                let (output, success, error_msg) = match response {
                    Ok(Ok(resp)) => {
                        if resp.status().is_success() {
                            match resp.json::<serde_json::Value>().await {
                                Ok(json) => (Some(json), true, None),
                                Err(e) => (None, false, Some(format!("JSON parse error: {}", e))),
                            }
                        } else {
                            let status = resp.status();
                            let text = resp.text().await.unwrap_or_default();
                            (None, false, Some(format!("HTTP {}: {}", status, text.trim().to_string())))
                        }
                    }
                    Ok(Err(e)) => (None, false, Some(format!("Request failed: {}", e))),
                    Err(_) => (None, false, Some("Timeout after 120 seconds".to_string())),
                };

                // Update sub-task
                if let Err(e) = db::update_sub_task(
                    &pool,
                    &sub_task_id,
                    if success { "completed" } else { "failed" },
                    output.as_ref(),
                    error_msg.as_deref(),
                    Some(latency_ms),
                ).await {
                    error!("Failed to update sub_task {}: {}", sub_task_id, e);
                }

                // Log execution
                let _ = db::log_execution(
                    &pool,
                    task_id,
                    Some(&agent.id),
                    0, // TODO: step index
                    "agent_call",
                    Some(&request_body),
                    output.as_ref(),
                    Some(latency_ms),
                    success,
                ).await;

                // Update agent performance
                let capability = sub_task["capability"].as_str().unwrap_or_default();
                let _ = db::update_agent_performance(
                    &pool,
                    &agent.id,
                    capability,
                    success,
                    if success { Some(latency_ms) } else { None },
                ).await;

                Ok::<(String, serde_json::Value, bool), anyhow::Error>((sub_task_id, output.unwrap_or_default(), success))
            });

            execution_handles.push((sub_task_id, handle));
        }

        // Wait for all sub-tasks to complete
        let mut results = Vec::new();
        let mut all_success = true;

        for (sub_task_id, handle) in execution_handles {
            match handle.await {
                Ok(Ok((_id, output, success))) => {
                    results.push((sub_task_id.clone(), output));
                    if !success {
                        all_success = false;
                    }
                }
                Ok(Err(e)) => {
                    error!("Sub-task {} execution error: {}", sub_task_id, e);
                    all_success = false;
                }
                Err(e) => {
                    error!("Sub-task {} join error: {}", sub_task_id, e);
                    all_success = false;
                }
            }
        }

        // Step 4: Aggregate results
        info!("Aggregating {} results for task {}", results.len(), task_id);

        if results.is_empty() {
            db::update_task_status(&self.pool, task_id, "failed", Some("No sub-tasks completed successfully")).await?;
            return Ok(());
        }

        // Collect agent names and capabilities for each result
        let mut aggregator_inputs = Vec::new();
        for sub_task_id in &results.iter().map(|(id, _)| id).collect::<Vec<_>>() {
            let sub_tasks = db::get_sub_tasks_for_task(&self.pool, task_id).await?;
            if let Some(st) = sub_tasks.iter().find(|st| st["id"] == *sub_task_id) {
                let agent_name = st["agent_name"].as_str().unwrap_or("Unknown");
                let capability = st["capability"].as_str().unwrap_or_default();
                let output = &results.iter().find(|(id, _)| *id == *sub_task_id).map(|(_, out)| out).unwrap_or(&serde_json::json!({}));
                aggregator_inputs.push((agent_name.to_string(), capability.to_string(), output.clone()));
            }
        }

        let aggregated_result = self.llm_service.aggregate_results(&aggregator_inputs).await?;

        info!("Task {} completed successfully", task_id);

        // Mark task as completed
        db::update_task_status(&self.pool, task_id, "completed", None).await?;

        Ok(())
    }

    pub async fn get_task_status(&self, task_id: &str) -> Result<Option<serde_json::Value>> {
        let task = db::get_task(&self.pool, task_id).await?;
        let sub_tasks = db::get_sub_tasks_for_task(&self.pool, task_id).await?;

        if let Some(task_json) = task {
            Ok(Some(serde_json::json!({
                "task": task_json,
                "sub_tasks": sub_tasks,
            })))
        } else {
            Ok(None)
        }
    }
}

impl Clone for TaskService {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
            llm_service: LlmService {
                client: reqwest::Client::new(),
                api_key: self.llm_service.api_key.clone(),
                model: self.llm_service.model.clone(),
                timeout_seconds: self.llm_service.timeout_seconds,
                max_tokens: self.llm_service.max_tokens,
            },
            agent_service: AgentService {
                pool: self.pool.clone(),
                llm_service: LlmService {
                    client: reqwest::Client::new(),
                    api_key: self.agent_service.llm_service.api_key.clone(),
                    model: self.agent_service.llm_service.model.clone(),
                    timeout_seconds: self.agent_service.llm_service.timeout_seconds,
                    max_tokens: self.agent_service.llm_service.max_tokens,
                },
            },
            http_client: reqwest::Client::new(),
            max_concurrent_tasks: self.max_concurrent_tasks,
        }
    }
}