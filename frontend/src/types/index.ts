export interface Agent {
  id: string;
  name: string;
  description?: string;
  endpoint_url: string;
  capabilities: string[];
  status: AgentStatus;
  metadata?: Record<string, any>;
  created_at: string;
  updated_at: string;
}

export type AgentStatus = 'active' | 'inactive' | 'error';

export interface NewAgent {
  name: string;
  description?: string;
  endpoint_url: string;
  capabilities: string[];
  metadata?: Record<string, any>;
}

export interface UserTask {
  id: string;
  user_query: string;
  parsed_plan?: ParsedPlan;
  status: TaskStatus;
  error_message?: string;
  created_at: string;
  started_at?: string;
  completed_at?: string;
}

export type TaskStatus = 'pending' | 'dispatched' | 'completed' | 'failed' | 'cancelled';

export interface NewTask {
  user_query: string;
}

export interface SubTask {
  id: string;
  task_id: string;
  agent_id: string;
  capability: string;
  input?: Record<string, any>;
  output?: Record<string, any>;
  error?: string;
  status: SubTaskStatus;
  started_at?: string;
  completed_at?: string;
  latency_ms?: number;
}

export type SubTaskStatus = 'pending' | 'running' | 'completed' | 'failed';

export interface Execution {
  id: string;
  task_id: string;
  agent_id?: string;
  step: number;
  action: string;
  input_snapshot?: Record<string, any>;
  output_snapshot?: Record<string, any>;
  latency_ms?: number;
  success: boolean;
  timestamp: string;
}

export interface ParsedPlan {
  steps: PlanStep[];
}

export interface PlanStep {
  agent_id: string;
  capability: string;
  input: Record<string, any>;
  order: number;
}

export interface AgentListResponse {
  agents: Agent[];
  total: number;
}

export interface TaskResponse {
  task: UserTask;
  sub_tasks: SubTask[];
}

export interface ExecutionStats {
  total_executions: number;
  success_rate: number;
  avg_latency_ms: number;
  by_agent: AgentStats[];
}

export interface AgentStats {
  agent_id: string;
  agent_name: string;
  total_tasks: number;
  success_rate: number;
  avg_latency_ms: number;
}

export interface TaskSubmission {
  user_query: string;
}

export interface ParseOnlyRequest {
  user_query: string;
}

export interface ParseOnlyResponse {
  success: boolean;
  parsed?: ParseResult[];
  error?: string;
}

export interface ParseResult {
  action: string;
  description: string;
  inputs: string[];
  outputs: string[];
  complexity: string;
}

export interface HealthResponse {
  status: string;
  database: string;
  timestamp: string;
}

export interface StatusResponse {
  agent_count: number;
  task_stats: TaskStats;
  uptime_seconds: number;
  start_time: string;
}

export interface TaskStats {
  pending: number;
  dispatched: number;
  completed: number;
  failed: number;
  total: number;
}
