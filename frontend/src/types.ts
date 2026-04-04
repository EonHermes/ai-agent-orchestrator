export interface Agent {
  id: string;
  name: string;
  description?: string;
  endpoint_url: string;
  capabilities: string[];
  status: 'active' | 'inactive' | 'error';
  metadata?: Record<string, unknown>;
  created_at: string;
  updated_at: string;
}

export interface Task {
  id: string;
  user_query: string;
  parsed_plan?: unknown;
  status: 'pending' | 'dispatched' | 'completed' | 'failed' | 'cancelled';
  error_message?: string;
  created_at: string;
  started_at?: string;
  completed_at?: string;
}

export interface SubTask {
  id: string;
  task_id: string;
  agent_id: string;
  agent_name?: string;
  capability: string;
  input?: unknown;
  output?: unknown;
  error?: string;
  status: 'pending' | 'running' | 'completed' | 'failed';
  started_at?: string;
  completed_at?: string;
  latency_ms?: number;
}

export interface TaskWithDetails extends Task {
  sub_tasks: SubTask[];
}

export interface Execution {
  id: string;
  task_id: string;
  agent_id?: string;
  agent_name?: string;
  step: number;
  action: string;
  input_snapshot?: unknown;
  output_snapshot?: unknown;
  latency_ms?: number;
  success: boolean;
  timestamp: string;
}

export interface AgentStats {
  agent: Agent;
  total_executions: number;
  success_rate: number;
  avg_latency_ms?: number;
  top_capabilities: [string, number][];
}

export interface SystemStatus {
  total_agents: number;
  active_agents: number;
  total_tasks: number;
  active_tasks: number;
  completed_tasks: number;
  failed_tasks: number;
  avg_task_latency_ms?: number;
}

export interface CreateAgentRequest {
  name: string;
  description?: string;
  endpoint_url: string;
  capabilities: string[];
  metadata?: Record<string, unknown>;
}

export interface UpdateAgentRequest {
  description?: string;
  endpoint_url?: string;
  capabilities?: string[];
  status?: 'active' | 'inactive' | 'error';
  metadata?: Record<string, unknown>;
}

export interface CreateTaskRequest {
  user_query: string;
}

export interface ParseRequest {
  user_query: string;
}

export interface ParseResponse {
  plan: PlanStep[];
  reasoning: string;
}

export interface PlanStep {
  capability: string;
  description: string;
  input_schema: Record<string, unknown>;
}