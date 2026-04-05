import React, { useEffect } from 'react';
import { useParams, Link } from 'react-router-dom';
import { ArrowLeft, Play, CheckCircle, XCircle, Clock, Server } from 'lucide-react';
import { useStore } from '../store/useStore';
import { formatDate } from '../utils/format';

const TaskDetail: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const { fetchTasks, selectTask, selectedTask, tasks } = useStore();

  useEffect(() => {
    if (id) {
      fetchTasks({ /* Could add specific filter */ });
      // For now, we'll search in the already fetched tasks or could call getTask endpoint
      const task = tasks.find((t) => t.id === id);
      if (task) selectTask(task);
    }
  }, [id, fetchTasks, tasks, selectTask]);

  if (!selectedTask) {
    return (
      <div className="text-center py-12">
        <p className="text-eon-textSecondary">Task not found or loading...</p>
      </div>
    );
  }

  const task = selectedTask;
  const subTasks = task.sub_tasks || [];

  return (
    <div className="space-y-6">
      <div className="flex items-center gap-4">
        <Link
          to="/tasks"
          className="p-2 hover:bg-eon-surface rounded-lg"
        >
          <ArrowLeft size={20} className="text-eon-textSecondary" />
        </Link>
        <div className="flex-1">
          <h1 className="text-2xl font-bold text-eon-text">Task Details</h1>
          <p className="text-eon-textSecondary text-sm">{task.id}</p>
        </div>
      </div>

      {/* Task Info */}
      <div className="card">
        <div className="flex items-start justify-between mb-4">
          <div>
            <h2 className="text-xl font-semibold text-eon-text mb-2">{task.user_query}</h2>
            <div className="flex items-center gap-4 text-sm text-eon-textSecondary">
              <span>Created: {formatDate(task.created_at)}</span>
              {task.started_at && <span>Started: {formatDate(task.started_at)}</span>}
              {task.completed_at && <span>Completed: {formatDate(task.completed_at)}</span>}
            </div>
          </div>
          <div
            className={`px-3 py-1 rounded-full text-sm font-medium ${
              task.status === 'completed'
                ? 'bg-eon-success/20 text-eon-success'
                : task.status === 'failed'
                ? 'bg-eon-error/20 text-eon-error'
                : task.status === 'dispatched'
                ? 'bg-eon-warning/20 text-eon-warning'
                : 'bg-eon-surface text-eon-textSecondary'
            }`}
          >
            {task.status}
          </div>
        </div>

        {task.parsed_plan && task.parsed_plan.steps && (
          <div className="mt-6">
            <h3 className="text-sm font-semibold text-eon-textSecondary mb-3">Execution Plan</h3>
            <div className="space-y-2">
              {task.parsed_plan.steps.map((step: any, idx: number) => (
                <div
                  key={idx}
                  className="flex items-start gap-3 p-3 bg-eon-surface rounded-lg"
                >
                  <div className="flex-shrink-0 w-6 h-6 rounded-full bg-eon-surfaceLight flex items-center justify-center text-xs font-medium">
                    {idx + 1}
                  </div>
                  <div className="flex-1">
                    <div className="flex items-center gap-2">
                      <span className="font-medium text-eon-text">{step.capability}</span>
                      {step.agent_id ? (
                        <span className="text-xs bg-eon-primary/20 text-eon-primary px-2 py-0.5 rounded">
                          Agent assigned
                        </span>
                      ) : (
                        <span className="text-xs bg-eon-error/20 text-eon-error px-2 py-0.5 rounded">
                          No agent
                        </span>
                      )}
                    </div>
                    {step.input.description && (
                      <p className="text-sm text-eon-textSecondary mt-1">{step.input.description}</p>
                    )}
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}
      </div>

      {/* Sub-Tasks */}
      <div className="card">
        <h3 className="text-lg font-semibold text-eon-text mb-4">Sub-Task Execution</h3>
        {subTasks.length === 0 ? (
          <p className="text-eon-textSecondary">No sub-tasks have been created yet.</p>
        ) : (
          <div className="space-y-4">
            {subTasks.map((subTask) => (
              <div
                key={subTask.id}
                className="flex items-start gap-4 p-4 bg-eon-surface rounded-lg"
              >
                <div className="flex-shrink-0">
                  {subTask.status === 'completed' ? (
                    <div className="p-2 bg-eon-success/20 rounded-lg">
                      <CheckCircle size={20} className="text-eon-success" />
                    </div>
                  ) : subTask.status === 'failed' ? (
                    <div className="p-2 bg-eon-error/20 rounded-lg">
                      <XCircle size={20} className="text-eon-error" />
                    </div>
                  ) : subTask.status === 'running' ? (
                    <div className="p-2 bg-eon-warning/20 rounded-lg">
                      <Clock size={20} className="text-eon-warning animate-pulse" />
                    </div>
                  ) : (
                    <div className="p-2 bg-eon-surfaceLight rounded-lg">
                      <Clock size={20} className="text-eon-textSecondary" />
                    </div>
                  )}
                </div>

                <div className="flex-1">
                  <div className="flex items-center gap-2 mb-1">
                    <span className="font-medium text-eon-text">{subTask.capability}</span>
                    <span
                      className={`text-xs px-2 py-0.5 rounded ${
                        subTask.status === 'completed'
                          ? 'bg-eon-success/20 text-eon-success'
                          : subTask.status === 'failed'
                          ? 'bg-eon-error/20 text-eon-error'
                          : subTask.status === 'running'
                          ? 'bg-eon-warning/20 text-eon-warning'
                          : 'bg-eon-surfaceLight text-eon-textSecondary'
                      }`}
                    >
                      {subTask.status}
                    </span>
                  </div>

                  {subTask.error && (
                    <p className="text-sm text-eon-error mt-1">{subTask.error}</p>
                  )}

                  {subTask.output && (
                    <details className="mt-2">
                      <summary className="text-sm text-eon-primary cursor-pointer">Show output</summary>
                      <pre className="mt-2 p-3 bg-eon-background rounded text-xs overflow-auto text-eon-textSecondary">
                        {JSON.stringify(subTask.output, null, 2)}
                      </pre>
                    </details>
                  )}

                  {subTask.latency_ms && (
                    <p className="text-xs text-eon-textSecondary mt-2">
                      Latency: {subTask.latency_ms}ms
                    </p>
                  )}
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      {/* Error display */}
      {task.error_message && (
        <div className="card border-eon-error">
          <div className="flex items-center gap-2 mb-2">
            <XCircle size={20} className="text-eon-error" />
            <h3 className="font-semibold text-eon-error">Task Failed</h3>
          </div>
          <p className="text-eon-text">{task.error_message}</p>
        </div>
      )}
    </div>
  );
};

export default TaskDetail;
