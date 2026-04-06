import React, { createContext, useContext, useReducer, useEffect, ReactNode } from 'react';
import { taskApi } from '@/services/api';
import { wsService, WebSocketMessage } from '@/services/websocket';
import { Task, TaskFilter, TaskStatus } from '@/types';

interface TaskState {
  tasks: Task[];
  loading: boolean;
  error: string | null;
  filter: TaskFilter;
}

type TaskAction =
  | { type: 'SET_TASKS'; payload: Task[] }
  | { type: 'ADD_TASK'; payload: Task }
  | { type: 'UPDATE_TASK'; payload: Task }
  | { type: 'DELETE_TASK'; payload: string }
  | { type: 'SET_LOADING'; payload: boolean }
  | { type: 'SET_ERROR'; payload: string | null }
  | { type: 'SET_FILTER'; payload: Partial<TaskFilter> };

const initialState: TaskState = {
  tasks: [],
  loading: false,
  error: null,
  filter: {
    status: undefined,
    priority_min: undefined,
    priority_max: undefined,
    tags: [],
    search: '',
  },
};

function taskReducer(state: TaskState, action: TaskAction): TaskState {
  switch (action.type) {
    case 'SET_TASKS':
      return { ...state, tasks: action.payload };
    case 'ADD_TASK':
      return { ...state, tasks: [action.payload, ...state.tasks] };
    case 'UPDATE_TASK':
      return {
        ...state,
        tasks: state.tasks.map(t =>
          t.id === action.payload.id ? action.payload : t
        ),
      };
    case 'DELETE_TASK':
      return {
        ...state,
        tasks: state.tasks.filter(t => t.id !== action.payload),
      };
    case 'SET_LOADING':
      return { ...state, loading: action.payload };
    case 'SET_ERROR':
      return { ...state, error: action.payload };
    case 'SET_FILTER':
      return { ...state, filter: { ...state.filter, ...action.payload } };
    default:
      return state;
  }
}

interface TaskContextValue extends TaskState {
  fetchTasks: () => Promise<void>;
  createTask: (task: any) => Promise<void>;
  updateTask: (id: string, task: any) => Promise<void>;
  deleteTask: (id: string) => Promise<void>;
  setFilter: (filter: Partial<TaskFilter>) => void;
  updateTaskStatus: (id: string, status: TaskStatus) => Promise<void>;
}

const TaskContext = createContext<TaskContextValue | null>(null);

export const TaskProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const [state, dispatch] = useReducer(taskReducer, initialState);

  const fetchTasks = async () => {
    dispatch({ type: 'SET_LOADING', payload: true });
    dispatch({ type: 'SET_ERROR', payload: null });

    try {
      const tasks = await taskApi.list(state.filter);
      dispatch({ type: 'SET_TASKS', payload: tasks });
    } catch (error: any) {
      dispatch({ type: 'SET_ERROR', payload: error.message });
    } finally {
      dispatch({ type: 'SET_LOADING', payload: false });
    }
  };

  const createTask = async (task: any) => {
    const created = await taskApi.create(task);
    dispatch({ type: 'ADD_TASK', payload: created });
  };

  const updateTask = async (id: string, task: any) => {
    const updated = await taskApi.update(id, task);
    dispatch({ type: 'UPDATE_TASK', payload: updated });
  };

  const updateTaskStatus = async (id: string, status: TaskStatus) => {
    await updateTask(id, { status });
  };

  const deleteTask = async (id: string) => {
    await taskApi.delete(id);
    dispatch({ type: 'DELETE_TASK', payload: id });
  };

  const setFilter = (filter: Partial<TaskFilter>) => {
    dispatch({ type: 'SET_FILTER', payload: filter });
  };

  useEffect(() => {
    fetchTasks();
  }, [state.filter.status, state.filter.search]); // Re-fetch when filter changes

  useEffect(() => {
    // Connect WebSocket
    const setupWs = async () => {
      try {
        await wsService.connect();

        const unsubscribe = wsService.subscribe((message: WebSocketMessage) => {
          switch (message.type) {
            case 'task_created':
              dispatch({ type: 'ADD_TASK', payload: message.task });
              break;
            case 'task_updated':
              dispatch({ type: 'UPDATE_TASK', payload: message.task });
              break;
            case 'task_deleted':
              dispatch({ type: 'DELETE_TASK', payload: message.task.id });
              break;
          }
        });
      } catch (error) {
        console.error('Failed to connect WebSocket:', error);
      }
    };

    setupWs();

    return () => {
      wsService.disconnect();
    };
  }, []);

  const value: TaskContextValue = {
    ...state,
    fetchTasks,
    createTask,
    updateTask,
    deleteTask,
    setFilter,
    updateTaskStatus,
  };

  return (
    <TaskContext.Provider value={value}>
      {children}
    </TaskContext.Provider>
  );
};

export const useTasks = () => {
  const context = useContext(TaskContext);
  if (!context) {
    throw new Error('useTasks must be used within TaskProvider');
  }
  return context;
};
