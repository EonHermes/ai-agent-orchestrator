export interface User {
  id: string;
  username: string;
  created_at: string;
}

export interface Snippet {
  id: string;
  user_id: string;
  title: string;
  code: string;
  language: string;
  description?: string;
  tags?: string;
  created_at: string;
  updated_at: string;
}

export interface CreateSnippet {
  title: string;
  code: string;
  language: string;
  description?: string;
  tags?: string;
}

export interface UpdateSnippet {
  title?: string;
  code?: string;
  language?: string;
  description?: string;
  tags?: string;
}

export interface LoginCredentials {
  username: string;
  password: string;
}

export interface AuthResponse {
  token: string;
  user: User;
}

export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
}