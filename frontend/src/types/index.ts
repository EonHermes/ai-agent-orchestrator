export type TaskStatus = 'todo' | 'in_progress' | 'done';

export interface Task {
  id: string;
  title: string;
  description?: string;
  status: TaskStatus;
  priority: number;
  tags: string[];
  created_at: string;
  updated_at: string;
}

export interface CreateTask {
  title: string;
  description?: string;
  status: TaskStatus;
  priority: number;
  tags: string[];
}

export interface UpdateTask {
  title?: string;
  description?: string;
  status?: TaskStatus;
  priority?: number;
  tags?: string[];
}

export interface TaskFilter {
  status?: TaskStatus;
  priority_min?: number;
  priority_max?: number;
  tags: string[];
  search?: string;
}

export interface ApiResponse<T> {
  data: T;
  timestamp: string;
}

export interface WebSocketMessage {
  type: string;
  payload: any;
}

export interface TaskBroadcast extends WebSocketMessage {
  task: Task;
  timestamp: string;
}
