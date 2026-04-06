import React, { useState, useEffect } from 'react';
import { Plus, Search, Filter } from 'lucide-react';
import { TaskForm } from '@/components/TaskForm';
import { TaskItem } from '@/components/TaskItem';
import { useTasks } from '@/context/TaskContext';
import { TaskStatus } from '@/types';

function App() {
  const { tasks, loading, error, filter, setFilter, createTask, updateTask, deleteTask, updateTaskStatus } = useTasks();
  const [showForm, setShowForm] = useState(false);
  const [editingTask, setEditingTask] = useState<any>(null);
  const [searchInput, setSearchInput] = useState(filter.search || '');

  useEffect(() => {
    const timer = setTimeout(() => {
      setFilter({ search: searchInput || undefined });
    }, 300);
    return () => clearTimeout(timer);
  }, [searchInput]);

  const handleSubmit = async (taskData: any) => {
    if (editingTask) {
      await updateTask(editingTask.id, taskData);
      setEditingTask(null);
    } else {
      await createTask(taskData);
    }
    setShowForm(false);
  };

  const handleEdit = (task: any) => {
    setEditingTask(task);
    setShowForm(true);
  };

  const handleCancel = () => {
    setShowForm(false);
    setEditingTask(null);
  };

  const handleDelete = async (id: string) => {
    if (window.confirm('Are you sure you want to delete this task?')) {
      await deleteTask(id);
    }
  };

  const handleStatusChange = async (id: string, status: string) => {
    await updateTaskStatus(id, status as TaskStatus);
  };

  const priorityLevels = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

  return (
    <div className="min-h-screen bg-gray-900 text-gray-100">
      <div className="max-w-6xl mx-auto px-4 py-8">
        <header className="mb-8">
          <h1 className="text-4xl font-bold mb-2">Task Dashboard</h1>
          <p className="text-gray-400">Real-time collaborative task management</p>

          <div className="mt-6 flex flex-col sm:flex-row gap-4">
            <div className="flex-1 relative">
              <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400" size={20} />
              <input
                type="text"
                value={searchInput}
                onChange={(e) => setSearchInput(e.target.value)}
                placeholder="Search tasks..."
                className="input pl-10"
              />
            </div>

            <div className="flex gap-2 items-center">
              <Filter size={20} className="text-gray-400" />
              <select
                value={filter.status || ''}
                onChange={(e) => setFilter({ status: e.target.value as TaskStatus || undefined })}
                className="input w-auto"
              >
                <option value="">All Status</option>
                <option value="todo">To Do</option>
                <option value="in_progress">In Progress</option>
                <option value="done">Done</option>
              </select>

              <select
                value={filter.priority_min?.toString() || ''}
                onChange={(e) => setFilter({ priority_min: e.target.value ? parseInt(e.target.value) : undefined })}
                className="input w-auto"
              >
                <option value="">Min Priority</option>
                {priorityLevels.map(p => (
                  <option key={`min-${p}`} value={p}>{p}+</option>
                ))}
              </select>

              <button
                onClick={() => setShowForm(true)}
                className="btn btn-primary flex items-center gap-2"
              >
                <Plus size={20} />
                New Task
              </button>
            </div>
          </div>
        </header>

        <main>
          {error && (
            <div className="mb-6 p-4 bg-red-900/50 border border-red-500 text-red-200 rounded-lg">
              {error}
            </div>
          )}

          {showForm && (
            <div className="mb-6">
              <TaskForm
                onSubmit={handleSubmit}
                initialValues={editingTask}
                onCancel={handleCancel}
                isEditing={!!editingTask}
              />
            </div>
          )}

          {loading ? (
            <div className="text-center py-12">
              <div className="inline-block animate-spin rounded-full h-8 w-8 border-t-2 border-b-2 border-blue-500 mb-4"></div>
              <p className="text-gray-400">Loading tasks...</p>
            </div>
          ) : tasks.length === 0 ? (
            <div className="card text-center py-12">
              <p className="text-gray-400 mb-4">No tasks found.</p>
              <button
                onClick={() => setShowForm(true)}
                className="btn btn-primary"
              >
                Create your first task
              </button>
            </div>
          ) : (
            <div className="space-y-4">
              {tasks.map(task => (
                <TaskItem
                  key={task.id}
                  task={task}
                  onEdit={handleEdit}
                  onDelete={handleDelete}
                  onStatusChange={handleStatusChange}
                />
              ))}
            </div>
          )}
        </main>

        <footer className="mt-12 pt-8 border-t border-gray-800 text-center text-gray-500 text-sm">
          <p>Real-time Task Dashboard • Built with Rust + React</p>
        </footer>
      </div>
    </div>
  );
}

export default App;
