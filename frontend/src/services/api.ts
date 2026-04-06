import axios from 'axios';
import { Task, CreateTask, UpdateTask, TaskFilter, ApiResponse } from '@/types';

const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080';

const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

export const taskApi = {
  list: async (filter?: TaskFilter): Promise<Task[]> => {
    const params = new URLSearchParams();
    if (filter?.status) params.append('status', filter.status);
    if (filter?.priority_min !== undefined) params.append('priority_min', filter.priority_min.toString());
    if (filter?.priority_max !== undefined) params.append('priority_max', filter.priority_max.toString());
    filter?.tags.forEach(tag => params.append('tags', tag));
    if (filter?.search) params.append('search', filter.search);

    const response = await api.get<ApiResponse<Task[]>>(`/api/tasks?${params.toString()}`);
    return response.data.data;
  },

  get: async (id: string): Promise<Task> => {
    const response = await api.get<ApiResponse<Task>>(`/api/tasks/${id}`);
    return response.data.data;
  },

  create: async (task: CreateTask): Promise<Task> => {
    const response = await api.post<ApiResponse<Task>>('/api/tasks', task);
    return response.data.data;
  },

  update: async (id: string, task: UpdateTask): Promise<Task> => {
    const response = await api.put<ApiResponse<Task>>(`/api/tasks/${id}`, task);
    return response.data.data;
  },

  delete: async (id: string): Promise<void> => {
    await api.delete(`/api/tasks/${id}`);
  },

  health: async (): Promise<string> => {
    const response = await api.get<ApiResponse<string>>('/api/health');
    return response.data.data;
  },
};
