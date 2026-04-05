use crate::{
    db::Repository,
    models::*,
    services::{agent_client::AgentClient, llm::LlmService},
};
use tracing::{error, info, warn};
use std::sync::Arc;

pub struct Executor {
    repo: Arc<Repository>,
    llm: Arc<LlmService>,
    agent_client: Arc<AgentClient>,
    max_concurrent_tasks: usize,
}

impl Executor {
    pub fn new(
        repo: Arc<Repository>,
        llm: Arc<LlmService>,
        agent_client: Arc<AgentClient>,
        max_concurrent_tasks: usize,
    ) -> Self {
        Self {
            repo,
            llm,
            agent_client,
            max_concurrent_tasks,
        }
    }

    pub async fn execute_task(&self, task_id: &str) -> Result<(), anyhow::Error> {
        // 1. Load task and agents
        let task = self.repo.get_task(task_id).await?.ok_or_else(|| anyhow::anyhow!("Task not found: {}", task_id))?;
        let agents = self.repo.list_agents(Some("active")).await?;

        if agents.is_empty() {
            anyhow::bail!("No active agents available for task execution");
        }

        // 2. Update task status to dispatched
        self.repo.update_task_status(task_id, TaskStatus::Dispatched, None).await?;

        // 3. Parse the user query into action items
        info!("Parsing task: {}", task.user_query);
        let parsed_results = self.llm.parse_task(&task.user_query).await?;
        info!("Parsed {} action items", parsed_results.len());

        // 4. Create execution plan by matching actions to agents
        info!("Creating execution plan with {} agents", agents.len());
        let mut plan_steps = Vec::new();
        let mut step_order = 0;

        for parsed in parsed_results {
            let action = parsed.action;
            let mut matched = false;

            // Find an agent with matching capability
            for agent in &agents {
                if agent.capabilities.contains(&action) {
                    plan_steps.push(PlanStep {
                        agent_id: agent.id.clone(),
                        capability: action.clone(),
                        input: serde_json::json!({
                            "description": parsed.description,
                            "inputs": parsed.inputs,
                            "outputs": parsed.outputs,
                            "complexity": parsed.complexity,
                        }),
                        order: step_order,
                    });
                    matched = true;
                    break;
                }
            }

            if !matched {
                warn!("No agent found for action: {}", action);
                // We can still continue - some steps may be unmatched
                plan_steps.push(PlanStep {
                    agent_id: "".to_string(),
                    capability: action,
                    input: serde_json::json!({
                        "description": parsed.description,
                        "error": "No agent available for this capability"
                    }),
                    order: step_order,
                });
            }

            step_order += 1;
        }

        let plan = ParsedPlan { steps: plan_steps.clone() };
        self.repo.update_task_plan(task_id, &serde_json::to_value(&plan)?).await?;

        // 5. Create sub-tasks and execute them sequentially
        for step in plan_steps.iter() {
            if step.agent_id.is_empty() {
                warn!("Skipping step {}: no agent available", step.order);
                continue;
            }

            let sub_task = self.repo.create_sub_task(&NewSubTask {
                task_id: task_id.to_string(),
                agent_id: step.agent_id.clone(),
                capability: step.capability.clone(),
                input: Some(step.input.clone()),
            }).await?;

            info!("Executing sub-task {} with agent {}", sub_task.id, step.agent_id);

            // Get agent details
            let agent = self.repo.get_agent(&step.agent_id).await?.ok_or_else(|| anyhow::anyhow!("Agent not found: {}", step.agent_id))?;

            // Execute
            let start = std::time::Instant::now();
            match self.agent_client.call_agent(
                &agent.endpoint_url,
                &step.capability,
                step.input.clone(),
                None,
            ).await {
                Ok(response) => {
                    let latency = start.elapsed().as_millis() as i64;
                    
                    if response.success {
                        info!("Sub-task {} completed successfully", sub_task.id);
                        self.repo.update_sub_task_status(
                            &sub_task.id,
                            SubTaskStatus::Completed,
                            response.output.as_ref(),
                            None,
                            Some(latency),
                        ).await?;
                    } else {
                        let error_msg = response.error.unwrap_or_else(|| "Unknown agent error".to_string());
                        error!("Sub-task {} failed: {}", sub_task.id, error_msg);
                        self.repo.update_sub_task_status(
                            &sub_task.id,
                            SubTaskStatus::Failed,
                            None,
                            Some(&error_msg),
                            Some(latency),
                        ).await?;
                        // Continue with next steps (failures don't stop the plan)
                    }
                }
                Err(e) => {
                    error!("Agent call failed for sub-task {}: {}", sub_task.id, e);
                    self.repo.update_sub_task_status(
                        &sub_task.id,
                        SubTaskStatus::Failed,
                        None,
                        Some(&e.to_string()),
                        None,
                    ).await?;
                    // Continue
                }
            }

            // Log execution for audit
            let execution = Execution::new(
                task_id.to_string(),
                Some(step.agent_id.clone()),
                step.order as i64,
                "execute_subtask".to_string(),
                Some(step.input.clone()),
                None,
                None,
                true, // We'll update with actual success based on sub-task
            );
            // Don't block on logging failure
            let _ = self.repo.create_execution(&execution, None).await;
        }

        // 6. Check if all sub-tasks completed successfully
        let sub_tasks = self.repo.get_sub_tasks_for_task(task_id).await?;
        let all_succeeded = sub_tasks.iter().all(|st| matches!(st.status, SubTaskStatus::Completed));
        let any_failed = sub_tasks.iter().any(|st| matches!(st.status, SubTaskStatus::Failed));

        let final_status = if any_failed {
            TaskStatus::Failed
        } else if all_succeeded {
            TaskStatus::Completed
        } else {
            TaskStatus::Failed // Some pending but shouldn't happen with sequential execution
        };

        self.repo.update_task_status(task_id, final_status, None).await?;

        info!("Task {} completed with status: {:?}", task_id, final_status);
        Ok(())
    }
}
