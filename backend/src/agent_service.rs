use sqlx::SqlitePool;
use anyhow::Result;
use tracing::{info, warn};

use crate::db;
use crate::models::{Agent, AgentStatus};

pub struct AgentService {
    pool: SqlitePool,
    llm_service: crate::llm_service::LlmService,
}

impl AgentService {
    pub fn new(pool: SqlitePool, llm_service: crate::llm_service::LlmService) -> Self {
        Self { pool, llm_service }
    }

    pub async fn find_agents_for_capability(&self, capability: &str) -> Result<Vec<Agent>> {
        let agents = db::list_agents(&self.pool, Some("active")).await?;
        let mut result = Vec::new();

        for agent_json in agents {
            let caps = match serde_json::from_value::<Vec<String>>(agent_json["capabilities"].clone()) {
                Ok(caps) => caps,
                Err(e) => {
                    warn!("Failed to parse capabilities for agent: {}", e);
                    continue;
                }
            };

            if caps.iter().any(|c| c == capability) {
                let agent = Agent {
                    id: agent_json["id"].as_str().unwrap_or_default().to_string(),
                    name: agent_json["name"].as_str().unwrap_or_default().to_string(),
                    description: agent_json["description"].as_str().map(String::from),
                    endpoint_url: agent_json["endpoint_url"].as_str().unwrap_or_default().to_string(),
                    capabilities: caps,
                    status: match agent_json["status"].as_str().unwrap_or_default() {
                        "active" => AgentStatus::Active,
                        "inactive" => AgentStatus::Inactive,
                        "error" => AgentStatus::Error,
                        _ => AgentStatus::Inactive,
                    },
                    metadata: agent_json["metadata"].clone(),
                    created_at: chrono::DateTime::parse_from_rfc3339(
                        agent_json["created_at"].as_str().unwrap_or_default()
                    ).unwrap_or_else(|_| chrono::Utc::now()),
                    updated_at: chrono::DateTime::parse_from_rfc3339(
                        agent_json["updated_at"].as_str().unwrap_or_default()
                    ).unwrap_or_else(|_| chrono::Utc::now()),
                };
                result.push(agent);
            }
        }

        info!("Found {} agents for capability '{}'", result.len(), capability);
        Ok(result)
    }

    pub async fn select_best_agent_for_capability(&self, capability: &str) -> Result<Option<Agent>> {
        let candidates = self.find_agents_for_capability(capability).await?;

        if candidates.is_empty() {
            return Ok(None);
        }

        // If only one candidate, return it
        if candidates.len() == 1 {
            return Ok(Some(candidates[0].clone()));
        }

        // TODO: Use performance data to select best agent
        // For now, return first active agent
        Ok(candidates.into_iter().next())
    }

    pub async fn register_agent_with_suggestions(
        &self,
        name: &str,
        description: &str,
        endpoint_url: &str,
        provided_capabilities: Vec<String>,
        metadata: Option<serde_json::Value>,
    ) -> Result<String> {
        let agent_id = uuid::Uuid::new_v4().to_string();

        // If no capabilities provided, use LLM to suggest them
        let capabilities = if provided_capabilities.is_empty() {
            info!("No capabilities provided, using LLM to suggest for agent: {}", name);
            self.llm_service.suggest_agent_capability(description).await?
        } else {
            provided_capabilities
        };

        db::create_agent(
            &self.pool,
            &agent_id,
            name,
            Some(description),
            endpoint_url,
            &capabilities,
            "active",
            metadata.as_ref(),
        ).await?;

        info!("Registered agent '{}' with {} capabilities", name, capabilities.len());
        Ok(agent_id)
    }

    pub async fn get_all_agent_stats(&self) -> Result<Vec<serde_json::Value>> {
        let agents = db::list_agents(&self.pool, None).await?;
        let mut stats = Vec::new();

        for agent_json in agents {
            let id = agent_json["id"].as_str().unwrap_or_default();
            if id.is_empty() {
                continue;
            }

            // Get performance data
            let performance = db::get_agent_performance(&self.pool, id).await?;

            let total_executions = performance.iter()
                .map(|p| p.success_count + p.failure_count)
                .sum::<i64>();

            let total_successes = performance.iter()
                .map(|p| p.success_count)
                .sum::<i64>();

            let success_rate = if total_executions > 0 {
                total_successes as f64 / total_executions as f64
            } else {
                0.0
            };

            let avg_latency = if total_executions > 0 {
                let weighted_sum = performance.iter()
                    .filter_map(|p| {
                        p.avg_latency_ms.map(|lat| {
                            let count = p.success_count + p.failure_count;
                            (lat * count as f64, count as f64)
                        })
                    })
                    .collect::<Vec<_>>();

                if !weighted_sum.is_empty() {
                    let (sum, counts): (f64, f64) = weighted_sum.iter().fold((0.0, 0.0), |(s, c), (&val, &cnt)| (s + val, c + cnt));
                    Some(sum / counts)
                } else {
                    None
                }
            } else {
                None
            };

            let top_capabilities = performance.iter()
                .map(|p| (p.capability.clone(), p.success_count + p.failure_count))
                .collect::<Vec<_>>();

            stats.push(serde_json::json!({
                "agent": agent_json,
                "total_executions": total_executions,
                "success_rate": success_rate,
                "avg_latency_ms": avg_latency,
                "top_capabilities": top_capabilities,
            }));
        }

        Ok(stats)
    }
}