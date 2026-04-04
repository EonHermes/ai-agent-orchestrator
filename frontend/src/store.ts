import { create } from 'zustand';
import type { Agent, Task, SystemStatus, AgentStats, ParseResponse } from './types';
import * as api from './api';

interface AppState {
  // System
  status: SystemStatus | null;
  refreshStatus: () => Promise<void>;

  // Agents
  agents: Agent[];
  agentStats: Record<string, AgentStats>;
  loadingAgents: boolean;
  fetchAgents: () => Promise<void>;
  createAgent: (data: api.CreateAgentRequest) => Promise<string>;
  updateAgent: (id: string, data: api.UpdateAgentRequest) => Promise<void>;
  deleteAgent: (id: string) => Promise<void>;
  fetchAgentStats: (id: string) => Promise<void>;

  // Tasks
  tasks: Task[];
  activeTask: Task | null;
  activeTaskDetails: api.TaskWithDetails | null;
  loadingTasks: boolean;
  fetchTasks: (status?: string) => Promise<void>;
  createTask: (query: string) => Promise<string>;
  getTask: (id: string) => Promise<void>;
  cancelTask: (id: string) => Promise<void>;
  parseTask: (query: string) => Promise<ParseResponse>;

  // Executions
  executions: api.Execution[];
  executionStats: any[];
  fetchExecutions: () => Promise<void>;
  fetchExecutionStats: () => Promise<void>;

  // UI
  activeTab: 'agents' | 'tasks' | 'executions' | 'dashboard';
  setActiveTab: (tab: AppState['activeTab']) => void;
  sidebarOpen: boolean;
  toggleSidebar: () => void;
}

export const useStore = create<AppState>((set) => ({
  // System
  status: null,
  refreshStatus: async () => {
    try {
      const status = await api.getStatus();
      set({ status });
    } catch (e) {
      console.error('Failed to fetch status:', e);
    }
  },

  // Agents
  agents: [],
  agentStats: {},
  loadingAgents: false,
  fetchAgents: async () => {
    set({ loadingAgents: true });
    try {
      const agents = await api.listAgents();
      set({ agents, loadingAgents: false });
    } catch (e) {
      console.error('Failed to fetch agents:', e);
      set({ loadingAgents: false });
    }
  },
  createAgent: async (data) => {
    const res = await api.createAgent(data);
    set((state) => ({ agents: [...state.agents, { ...data, id: res.id, status: 'active', created_at: new Date().toISOString(), updated_at: new Date().toISOString(), capabilities: data.capabilities } as Agent] }));
    return res.id;
  },
  updateAgent: async (id, data) => {
    await api.updateAgent(id, data);
    set((state) => ({
      agents: state.agents.map((a) => (a.id === id ? { ...a, ...data } : a)),
    }));
  },
  deleteAgent: async (id) => {
    await api.deleteAgent(id);
    set((state) => ({ agents: state.agents.filter((a) => a.id !== id) }));
  },
  fetchAgentStats: async (id) => {
    try {
      const stats = await api.getAgentStats(id);
      set((state) => ({ agentStats: { ...state.agentStats, [id]: stats } }));
    } catch (e) {
      console.error('Failed to fetch agent stats:', e);
    }
  },

  // Tasks
  tasks: [],
  activeTask: null,
  activeTaskDetails: null,
  loadingTasks: false,
  fetchTasks: async (status?: string) => {
    set({ loadingTasks: true });
    try {
      const tasks = await api.listTasks(status);
      set({ tasks, loadingTasks: false });
    } catch (e) {
      console.error('Failed to fetch tasks:', e);
      set({ loadingTasks: false });
    }
  },
  createTask: async (query) => {
    const res = await api.createTask({ user_query: query });
    set((state) => ({ tasks: [{
      id: res.task_id,
      user_query: query,
      status: 'pending',
      created_at: new Date().toISOString(),
    }, ...state.tasks]));
    // Refresh tasks and status
    await Promise.all([
      useStore.getState().fetchTasks(),
      useStore.getState().refreshStatus(),
    ]);
    return res.task_id;
  },
  getTask: async (id) => {
    const details = await api.getTask(id);
    set({ activeTask: details.task, activeTaskDetails: details });
  },
  cancelTask: async (id) => {
    await api.cancelTask(id);
    set((state) => ({
      tasks: state.tasks.map((t) => (t.id === id ? { ...t, status: 'cancelled' as const } : t)),
      activeTask: state.activeTask?.id === id ? { ...state.activeTask, status: 'cancelled' as const } : state.activeTask,
    }));
  },
  parseTask: async (query) => {
    return api.parseTask(query);
  },

  // Executions
  executions: [],
  executionStats: [],
  fetchExecutions: async () => {
    try {
      const executions = await api.listExecutions(100);
      set({ executions });
    } catch (e) {
      console.error('Failed to fetch executions:', e);
    }
  },
  fetchExecutionStats: async () => {
    try {
      const stats = await api.getExecutionStats();
      set({ executionStats: stats });
    } catch (e) {
      console.error('Failed to fetch execution stats:', e);
    }
  },

  // UI
  activeTab: 'dashboard',
  setActiveTab: (tab) => set({ activeTab: tab }),
  sidebarOpen: true,
  toggleSidebar: () => set((state) => ({ sidebarOpen: !state.sidebarOpen })),
}));