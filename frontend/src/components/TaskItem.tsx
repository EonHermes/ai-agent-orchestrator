import React from 'react';
import { Trash2, Edit2, MoreVertical } from 'lucide-react';
import { Task } from '@/types';

interface TaskItemProps {
  task: Task;
  onEdit: (task: Task) => void;
  onDelete: (id: string) => void;
  onStatusChange: (id: string, status: string) => void;
}

export const TaskItem: React.FC<TaskItemProps> = ({
  task,
  onEdit,
  onDelete,
  onStatusChange,
}) => {
  const statusColors = {
    todo: 'badge-todo',
    in_progress: 'badge-in_progress',
    done: 'badge-done',
  };

  const statusLabels = {
    todo: 'To Do',
    in_progress: 'In Progress',
    done: 'Done',
  };

  return (
    <div className="card hover:bg-gray-750 transition-colors group">
      <div className="flex items-start justify-between gap-4">
        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-2 mb-2">
            <span className={`badge ${statusColors[task.status]}`}>
              {statusLabels[task.status]}
            </span>
            <span className="text-sm text-gray-400">
              Priority: {task.priority}
            </span>
          </div>

          <h3 className="text-lg font-medium text-white mb-2 break-words">
            {task.title}
          </h3>

          {task.description && (
            <p className="text-gray-400 text-sm mb-3 line-clamp-2">
              {task.description}
            </p>
          )}

          {task.tags.length > 0 && (
            <div className="flex flex-wrap gap-1 mb-3">
              {task.tags.map((tag) => (
                <span key={tag} className="badge badge-todo text-xs">
                  {tag}
                </span>
              ))}
            </div>
          )}

          <div className="text-xs text-gray-500">
            Updated: {new Date(task.updated_at).toLocaleDateString()}
          </div>
        </div>

        <div className="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
          <select
            value={task.status}
            onChange={(e) => onStatusChange(task.id, e.target.value)}
            className="text-xs bg-gray-700 border border-gray-600 rounded px-2 py-1 text-white"
            title="Change status"
          >
            <option value="todo">To Do</option>
            <option value="in_progress">In Progress</option>
            <option value="done">Done</option>
          </select>

          <button
            onClick={() => onEdit(task)}
            className="p-2 text-gray-400 hover:text-white hover:bg-gray-700 rounded"
            title="Edit task"
          >
            <Edit2 size={16} />
          </button>

          <button
            onClick={() => onDelete(task.id)}
            className="p-2 text-gray-400 hover:text-red-400 hover:bg-gray-700 rounded"
            title="Delete task"
          >
            <Trash2 size={16} />
          </button>
        </div>
      </div>
    </div>
  );
};
