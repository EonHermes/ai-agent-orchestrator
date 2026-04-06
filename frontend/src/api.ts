import axios from 'axios';
import { ApiResponse, CreateSnippet, UpdateSnippet, Snippet } from '../types';

const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080/api';

const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Add token to requests if available
api.interceptors.request.use(config => {
  const token = localStorage.getItem('token');
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

// Handle auth errors
api.interceptors.response.use(
  response => response,
  error => {
    if (error.response?.status === 401) {
      localStorage.removeItem('token');
      localStorage.removeItem('user');
      window.location.href = '/login';
    }
    return Promise.reject(error);
  }
);

export const authAPI = {
  register: (username: string, password: string) =>
    api.post<ApiResponse<User & { token: string }>>('/auth/register', { username, password }),
  login: (username: string, password: string) =>
    api.post<ApiResponse<{ token: string; user: User }>>('/auth/login', { username, password }),
};

export const snippetsAPI = {
  create: (snippet: CreateSnippet) =>
    api.post<ApiResponse<Snippet>>('/snippets', snippet),
  getAll: (params?: { user_id?: string; language?: string; tag?: string }) =>
    api.get<ApiResponse<Snippet[]>>('/snippets', { params }),
  getById: (id: string) =>
    api.get<ApiResponse<Snippet>>(`/snippets/${id}`),
  update: (id: string, snippet: UpdateSnippet) =>
    api.put<ApiResponse<Snippet>>(`/snippets/${id}`, snippet),
  delete: (id: string) =>
    api.delete<ApiResponse<null>>(`/snippets/${id}`),
  search: (query: string, limit?: number) =>
    api.get<ApiResponse<Snippet[]>>('/snippets/search', { params: { q: query, limit } }),
};

export default api;