use reqwest::Client;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanStep {
    pub capability: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseResponse {
    pub plan: Vec<PlanStep>,
    pub reasoning: String,
}

pub struct LlmService {
    client: Client,
    api_key: String,
    model: String,
    timeout_seconds: u64,
    max_tokens: u32,
}

#[derive(Debug, Clone, Serialize)]
struct LlmRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: u32,
    temperature: f32,
}

#[derive(Debug, Clone, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Clone, Deserialize)]
struct LlmResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Clone, Deserialize)]
struct Choice {
    message: Message,
}

impl LlmService {
    pub fn new(api_key: String, model: String, timeout_seconds: u64, max_tokens: u32) -> Self {
        Self {
            client: Client::new(),
            api_key,
            model,
            timeout_seconds,
            max_tokens,
        }
    }

    pub async fn parse_task(&self, user_query: &str, available_capabilities: &[String]) -> Result<ParseResponse> {
        let system_prompt = format!(
            r#"You are an intelligent task planner for an AI Agent Orchestrator system.
Your job is to decompose user requests into a sequence of sub-tasks that can be executed by specialized AI agents.

AVAILABLE CAPABILITIES:
{}

RULES:
1. Break complex requests into 2-5 logical steps
2. Each step must map to exactly ONE capability from the available list
3. Be specific about what each step needs as input
4. Provide clear reasoning for your decomposition
5. If the request cannot be fulfilled with available capabilities, set plan to [] and explain why

OUTPUT FORMAT (strict JSON):
{{
  "reasoning": "Explain your thought process",
  "plan": [
    {{
      "capability": "exact_capability_name",
      "description": "What this step does",
      "input_schema": {{}} // JSON schema or example input object
    }}
  ]
}}

IMPORTANT: capability MUST match exactly one from the available capabilities list. Do not invent capabilities."#,
            available_capabilities.iter().map(|c| format!("- {}", c)).collect::<Vec<_>>().join("\n"),
            serde_json::to_string(&serde_json::json!({
                "type": "object",
                "properties": {
                    "reasoning": {"type": "string"},
                    "plan": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "capability": {"type": "string"},
                                "description": {"type": "string"},
                                "input_schema": {"type": "object"}
                            },
                            "required": ["capability", "description", "input_schema"]
                        }
                    }
                },
                "required": ["reasoning", "plan"]
            })).unwrap_or_default()
        );

        let user_message = format!("User request: \"{}\"", user_query);

        let request = LlmRequest {
            model: self.model.clone(),
            messages: vec![
                Message { role: "system".to_string(), content: system_prompt },
                Message { role: "user".to_string(), content: user_message },
            ],
            max_tokens: self.max_tokens,
            temperature: 0.3,  // Low temperature for consistent parsing
        };

        info!("Sending parsing request to OpenRouter for query: {}", user_query);

        let response = self.client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .timeout(std::time::Duration::from_secs(self.timeout_seconds))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            anyhow::bail!("OpenRouter API error {}: {}", status, text);
        }

        let llm_response: LlmResponse = response.json().await?;

        if let Some(choice) = llm_response.choices.first() {
            let content = &choice.message.content;
            info!("LLM parsing completed, response length: {}", content.len());

            // Extract JSON from response (handle markdown code blocks)
            let json_str = if let Some(start) = content.find('{') {
                if let Some(end) = content.rfind('}') {
                    &content[start..=end]
                } else {
                    content
                }
            } else {
                content
            };

            let parsed: ParseResponse = serde_json::from_str(json_str)?;
            Ok(parsed)
        } else {
            anyhow::bail!("No response from LLM");
        }
    }

    pub async fn aggregate_results(&self, sub_task_results: &[(String, String, serde_json::Value)]) -> Result<String> {
        // sub_task_results: (agent_name, capability, output)
        let results_summary = sub_task_results.iter()
            .map(|(agent, cap, output)| {
                format!("- {} [{}]: {}", agent, cap, serde_json::to_string_pretty(output).unwrap_or_default())
            })
            .collect::<Vec<_>>()
            .join("\n\n");

        let prompt = format!(
            r#"You are an intelligent result aggregator for multi-agent workflows.

Given the following sub-task results, synthesize them into a coherent, comprehensive response.

RESULT SUMMARY:
{}

INSTRUCTIONS:
1. Combine the results into a single natural language response
2. Preserve important data and insights from each sub-task
3. Resolve any contradictions if present (prefer more specific/recent data)
4. Format the response clearly with appropriate structure (markdown is fine)
5. Do not mention that you are an aggregator or that these were separate tasks
6. Make the response sound like it came from a single, unified intelligence
7. Include relevant data points, metrics, and actionable insights

OUTPUT:
A single, comprehensive response that answers the original user query."#,
            results_summary
        );

        let request = LlmRequest {
            model: self.model.clone(),
            messages: vec![
                Message { role: "system".to_string(), content: "You are a result aggregator that synthesizes multi-agent outputs into coherent responses.".to_string() },
                Message { role: "user".to_string(), content: prompt },
            ],
            max_tokens: self.max_tokens,
            temperature: 0.5,
        };

        info!("Aggregating {} sub-task results", sub_task_results.len());

        let response = self.client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .timeout(std::time::Duration::from_secs(self.timeout_seconds))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("OpenRouter API error: {}", response.status());
        }

        let llm_response: LlmResponse = response.json().await?;

        if let Some(choice) = llm_response.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            anyhow::bail!("No aggregation response from LLM");
        }
    }

    pub async fn suggest_agent_capability(&self, agent_description: &str) -> Result<Vec<String>> {
        let prompt = format!(
            r#"Based on this agent description, suggest 3-5 relevant capability strings that this agent might provide.

AGENT DESCRIPTION:
{}

Rules:
- Capabilities should be snake_case, descriptive verbs + nouns (e.g., "performance_analysis", "code_review", "natural_language_workflow")
- Focus on what the agent can DO, not its name
- Be specific and actionable
- Return ONLY a JSON array of strings

Output format: ["capability1", "capability2", ...]"#,
            agent_description
        );

        let request = LlmRequest {
            model: self.model.clone(),
            messages: vec![
                Message { role: "system".to_string(), content: "You are a capability extraction engine.".to_string() },
                Message { role: "user".to_string(), content: prompt },
            ],
            max_tokens: 500,
            temperature: 0.3,
        };

        let response = self.client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;

        let llm_response: LlmResponse = response.json().await?;

        if let Some(choice) = llm_response.choices.first() {
            let content = &choice.message.content;
            let json_str = if let Some(start) = content.find('[') {
                &content[start..]
            } else {
                content
            };

            let capabilities: Vec<String> = serde_json::from_str(json_str)?;
            Ok(capabilities)
        } else {
            anyhow::bail!("No capability suggestion from LLM");
        }
    }
}