import axios from 'axios';
import type {
  Agent,
  NewAgent,
  UserTask,
  NewTask,
  TaskResponse,
  Execution,
  ExecutionStats,
  StatusResponse,
  HealthResponse,
  ParseOnlyRequest,
  ParseOnlyResponse,
} from '../types';

const api = axios.create({
  baseURL: '/api/v1',
  headers: {
    'Content-Type': 'application/json',
  },
});

export const agentApi = {
  list: (status?: string): Promise<{ agents: Agent[]; total: number }> =>
    api.get('/agents', { params: { status } }).then((res) => res.data),

  get: (id: string): Promise<Agent> =>
    api.get(`/agents/${id}`).then((res) => res.data),

  create: (agent: NewAgent): Promise<Agent> =>
    api.post('/agents', agent).then((res) => res.data),

  update: (id: string, data: Partial<NewAgent & { status?: string }>): Promise<Agent> =>
    api.put(`/agents/${id}`, data).then((res) => res.data),

  delete: (id: string): Promise<void> =>
    api.delete(`/agents/${id}`).then(() => undefined),

  capabilities: (): Promise<string[]> =>
    api.get('/agents/capabilities').then((res) => res.data),
};

export const taskApi = {
  list: (params?: {
    status?: string;
    agent_id?: string;
    limit?: number;
    offset?: number;
  }): Promise<UserTask[]> =>
    api.get('/tasks', { params }).then((res) => res.data),

  get: (id: string): Promise<TaskResponse> =>
    api.get(`/tasks/${id}`).then((res) => res.data),

  create: (task: NewTask): Promise<UserTask> =>
    api.post('/tasks', task).then((res) => res.data),

  submit: (submission: { user_query: string }): Promise<TaskResponse> =>
    api.post('/tasks/submit', submission).then((res) => res.data),

  cancel: (id: string): Promise<void> =>
    api.post(`/tasks/${id}/cancel`).then(() => undefined),
};

export const executionApi = {
  list: (task_id: string, limit?: number): Promise<Execution[]> =>
    api.get('/executions', { params: { task_id, limit } }).then((res) => res.data),

  stats: (): Promise<ExecutionStats> =>
    api.get('/executions/stats').then((res) => res.data),
};

export const systemApi = {
  health: (): Promise<HealthResponse> =>
    api.get('/health').then((res) => res.data),

  status: (): Promise<StatusResponse> =>
    api.get('/status').then((res) => res.data),

  parse: (data: ParseOnlyRequest): Promise<ParseOnlyResponse> =>
    api.post('/parse', data).then((res) => res.data),
};

export default api;
