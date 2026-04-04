import axios from 'axios';
import type {
  Agent,
  Task,
  SubTask,
  Execution,
  SystemStatus,
  AgentStats,
  CreateAgentRequest,
  UpdateAgentRequest,
  CreateTaskRequest,
  TaskWithDetails,
  ParseResponse,
} from './types';

const API_BASE = '/api/v1';

const api = axios.create({
  baseURL: API_BASE,
  headers: {
    'Content-Type': 'application/json',
  },
});

export const health = async () => {
  const res = await api.get('/../health');
  return res.data;
};

export const getStatus = async () => {
  const res = await api.get<SystemStatus>('/status');
  return res.data;
};

// Agents
export const listAgents = async (status?: string) => {
  const params = status ? { status } : undefined;
  const res = await api.get<Agent[]>('/agents', { params });
  return res.data;
};

export const createAgent = async (data: CreateAgentRequest) => {
  const res = await api.post<{ id: string }>('/agents', data);
  return res.data;
};

export const getAgent = async (id: string) => {
  const res = await api.get<Agent>(`/agents/${id}`);
  return res.data;
};

export const updateAgent = async (id: string, data: UpdateAgentRequest) => {
  await api.put(`/agents/${id}`, data);
};

export const deleteAgent = async (id: string) => {
  await api.delete(`/agents/${id}`);
};

export const listCapabilities = async () => {
  const res = await api.get<string[]>('/agents/capabilities');
  return res.data;
};

export const getAgentStats = async (id: string) => {
  const res = await api.get<AgentStats>(`/agents/${id}/stats`);
  return res.data;
};

// Tasks
export const createTask = async (data: CreateTaskRequest) => {
  const res = await api.post<{ task_id: string }>('/tasks', data);
  return res.data;
};

export const listTasks = async (status?: string, limit?: number, offset?: number) => {
  const params: Record<string, number | string> = {};
  if (status) params.status = status;
  if (limit) params.limit = limit;
  if (offset) params.offset = offset;
  const res = await api.get<Task[]>('/tasks', { params });
  return res.data;
};

export const getTask = async (id: string) => {
  const res = await api.get<TaskWithDetails>(`/tasks/${id}`);
  return res.data;
};

export const cancelTask = async (id: string) => {
  await api.post(`/tasks/${id}/cancel`);
};

export const getTaskPlan = async (id: string) => {
  const res = await api.get<{ plan: unknown }>(`/tasks/${id}/plan`);
  return res.data;
};

export const parseTask = async (user_query: string) => {
  const res = await api.post<ParseResponse>('/parse', { user_query });
  return res.data;
};

// Executions
export const listExecutions = async (limit: number = 100) => {
  const res = await api.get<Execution[]>('/executions', { params: { limit } });
  return res.data;
};

export const getExecution = async (id: string) => {
  const res = await api.get<Execution>(`/executions/${id}`);
  return res.data;
};

export const getExecutionStats = async () => {
  const res = await api.get<any[]>('/executions/stats');
  return res.data;
};