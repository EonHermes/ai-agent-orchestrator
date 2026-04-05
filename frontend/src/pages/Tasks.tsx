import React, { useState } from 'react';
import { Link } from 'react-router-dom';
import { Plus, Search, Filter, ArrowRight, Clock, CheckCircle, XCircle, AlertCircle } from 'lucide-react';
import { useStore } from '../store/useStore';
import { formatDistanceToNow } from '../utils/format';

const Tasks: React.FC = () => {
  const { tasks, loadingTasks, fetchTasks, submitTask } = useStore();
  const [query, setQuery] = useState('');
  const [statusFilter, setStatusFilter] = useState<string>('');
  const [showSubmitForm, setShowSubmitForm] = useState(false);
  const [newQuery, setNewQuery] = useState('');

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newQuery.trim()) return;

    try {
      await submitTask(newQuery);
      setNewQuery('');
      setShowSubmitForm(false);
      fetchTasks({ status: statusFilter || undefined });
    } catch (error) {
      console.error('Failed to submit task:', error);
      alert('Failed to submit task');
    }
  };

  const filteredTasks = tasks.filter((task) => {
    if (statusFilter && task.status !== statusFilter) return false;
    return true;
  });

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'completed':
        return <CheckCircle size={16} className="text-eon-success" />;
      case 'failed':
        return <XCircle size={16} className="text-eon-error" />;
      case 'dispatched':
        return <Clock size={16} className="text-eon-warning" />;
      case 'cancelled':
        return <AlertCircle size={16} className="text-eon-textSecondary" />;
      default:
        return null;
    }
  };

  const getStatusClass = (status: string) => {
    switch (status) {
      case 'completed':
        return 'bg-eon-success/20 text-eon-success';
      case 'failed':
        return 'bg-eon-error/20 text-eon-error';
      case 'dispatched':
        return 'bg-eon-warning/20 text-eon-warning';
      case 'cancelled':
        return 'bg-eon-surface text-eon-textSecondary';
      default:
        return 'bg-eon-surfaceLight text-eon-textSecondary';
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-3xl font-bold text-eon-text">Tasks</h1>
          <p className="text-eon-textSecondary mt-2">
            Submit and monitor AI agent tasks
          </p>
        </div>
        <button
          onClick={() => setShowSubmitForm(true)}
          className="btn-primary flex items-center gap-2"
        >
          <Plus size={20} />
          <span>New Task</span>
        </button>
      </div>

      {/* Submit Form Modal */}
      {showSubmitForm && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="card max-w-2xl w-full">
            <h2 className="text-xl font-bold text-eon-text mb-4">Submit New Task</h2>
            <form onSubmit={handleSubmit} className="space-y-4">
              <div>
                <label className="label">Describe your task in natural language</label>
                <textarea
                  value={newQuery}
                  onChange={(e) => setNewQuery(e.target.value)}
                  className="input w-full h-32"
                  placeholder="e.g., Analyze last week's model performance and generate a report..."
                  required
                />
                <p className="text-xs text-eon-textSecondary mt-2">
                  The AI will automatically parse your request, decompose it into sub-tasks,
                  and dispatch to the most appropriate agents.
                </p>
              </div>

              <div className="flex gap-3">
                <button type="submit" className="btn-primary">
                  Submit Task
                </button>
                <button
                  type="button"
                  onClick={() => setShowSubmitForm(false)}
                  className="btn-secondary"
                >
                  Cancel
                </button>
              </div>
            </form>
          </div>
        </div>
      )}

      {/* Filters */}
      <div className="card">
        <div className="flex gap-4">
          <div className="relative flex-1">
            <Search size={18} className="absolute left-3 top-1/2 transform -translate-y-1/2 text-eon-textSecondary" />
            <input
              type="text"
              placeholder="Search tasks..."
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              className="input w-full pl-10"
            />
          </div>
          <div className="flex items-center gap-2">
            <Filter size={18} className="text-eon-textSecondary" />
            <select
              value={statusFilter}
              onChange={(e) => setStatusFilter(e.target.value)}
              className="input w-40"
            >
              <option value="">All Status</option>
              <option value="pending">Pending</option>
              <option value="dispatched">Dispatched</option>
              <option value="completed">Completed</option>
              <option value="failed">Failed</option>
              <option value="cancelled">Cancelled</option>
            </select>
          </div>
        </div>
      </div>

      {/* Task List */}
      {loadingTasks ? (
        <div className="text-center py-12 text-eon-textSecondary">Loading tasks...</div>
      ) : filteredTasks.length === 0 ? (
        <div className="card text-center py-12">
          <Clock size={64} className="mx-auto mb-4 text-eon-textSecondary opacity-50" />
          <h3 className="text-lg font-semibold text-eon-text mb-2">No Tasks Yet</h3>
          <p className="text-eon-textSecondary mb-6">
            Submit a task to see the orchestration in action
          </p>
          <button onClick={() => setShowSubmitForm(true)} className="btn-primary">
            Submit Your First Task
          </button>
        </div>
      ) : (
        <div className="space-y-4">
          {filteredTasks.map((task) => (
            <Link
              key={task.id}
              to={`/tasks/${task.id}`}
              className="card block hover:bg-eon-surfaceLight transition-colors"
            >
              <div className="flex items-start justify-between">
                <div className="flex-1">
                  <div className="flex items-center gap-3 mb-2">
                    {getStatusIcon(task.status)}
                    <span className={`px-2 py-1 rounded text-xs font-medium ${getStatusClass(task.status)}`}>
                      {task.status.toUpperCase()}
                    </span>
                    <span className="text-xs text-eon-textSecondary">
                      {formatDistanceToNow(task.created_at)} ago
                    </span>
                  </div>
                  <p className="text-eon-text font-medium mb-2">{task.user_query}</p>
                  {task.parsed_plan && task.parsed_plan.steps && (
                    <div className="flex items-center gap-2 text-sm text-eon-textSecondary">
                      <span>{task.parsed_plan.steps.length} steps</span>
                      <span>•</span>
                      <span>
                        {task.parsed_plan.steps.filter((s: any) => s.agent_id).length} agents
                      </span>
                    </div>
                  )}
                </div>
                <ArrowRight className="text-eon-textSecondary ml-4" size={20} />
              </div>
            </Link>
          ))}
        </div>
      )}
    </div>
  );
};

export default Tasks;
