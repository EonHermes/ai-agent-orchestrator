import { create } from 'zustand';
import type { Agent, UserTask, ExecutionStats, StatusResponse, HealthResponse } from '../types';
import { agentApi, taskApi, executionApi, systemApi } from '../services/api';

interface AppState {
  // Agents
  agents: Agent[];
  loadingAgents: boolean;
  fetchAgents: () => Promise<void>;
  createAgent: (agent: any) => Promise<Agent>;
  deleteAgent: (id: string) => Promise<void>;

  // Tasks
  tasks: UserTask[];
  loadingTasks: boolean;
  fetchTasks: (params?: any) => Promise<void>;
  submitTask: (query: string) => Promise<UserTask>;

  // Stats
  stats: ExecutionStats | null;
  systemStatus: StatusResponse | null;
  health: HealthResponse | null;
  fetchStats: () => Promise<void>;
  fetchSystemStatus: () => Promise<void>;
  fetchHealth: () => Promise<void>;

  // Selected
  selectedTask: UserTask | null;
  selectTask: (task: UserTask | null) => void;
}

export const useStore = create<AppState>((set) => ({
  // Agents
  agents: [],
  loadingAgents: false,
  fetchAgents: async () => {
    set({ loadingAgents: true });
    try {
      const response = await agentApi.list();
      set({ agents: response.agents, loadingAgents: false });
    } catch (error) {
      console.error('Failed to fetch agents:', error);
      set({ loadingAgents: false });
    }
  },
  createAgent: async (agent) => {
    const created = await agentApi.create(agent);
    set((state) => ({ agents: [...state.agents, created] }));
    return created;
  },
  deleteAgent: async (id) => {
    await agentApi.delete(id);
    set((state) => ({ agents: state.agents.filter((a) => a.id !== id) }));
  },

  // Tasks
  tasks: [],
  loadingTasks: false,
  fetchTasks: async (params) => {
    set({ loadingTasks: true });
    try {
      const tasks = await taskApi.list(params);
      set({ tasks, loadingTasks: false });
    } catch (error) {
      console.error('Failed to fetch tasks:', error);
      set({ loadingTasks: false });
    }
  },
  submitTask: async (query) => {
    const response = await taskApi.submit({ user_query: query });
    set((state) => ({ tasks: [response.task, ...state.tasks] }));
    return response.task;
  },

  // Stats
  stats: null,
  systemStatus: null,
  health: null,
  fetchStats: async () => {
    try {
      const [stats, status, health] = await Promise.all([
        executionApi.stats(),
        systemApi.status(),
        systemApi.health(),
      ]);
      set({ stats, systemStatus: status, health });
    } catch (error) {
      console.error('Failed to fetch stats:', error);
    }
  },
  fetchSystemStatus: async () => {
    try {
      const status = await systemApi.status();
      set({ systemStatus: status });
    } catch (error) {
      console.error('Failed to fetch system status:', error);
    }
  },
  fetchHealth: async () => {
    try {
      const health = await systemApi.health();
      set({ health });
    } catch (error) {
      console.error('Failed to fetch health:', error);
    }
  },

  // Selected task
  selectedTask: null,
  selectTask: (task) => set({ selectedTask: task }),
}));
