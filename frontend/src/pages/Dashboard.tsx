import React from 'react';
import { Link } from 'react-router-dom';
import { Activity, Bot, ListTodo, Clock, ArrowRight, Plus } from 'lucide-react';
import { useStore } from '../store/useStore';
import { formatDistanceToNow } from '../utils/format';

const Dashboard: React.FC = () => {
  const { agents, tasks, stats, systemStatus, health, fetchAgents, fetchTasks } = useStore();

  React.useEffect(() => {
    fetchAgents();
    fetchTasks();
  }, [fetchAgents, fetchTasks]);

  const activeAgents = agents.filter((a) => a.status === 'active');
  const recentTasks = tasks.slice(0, 5);

  return (
    <div className="space-y-8">
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-3xl font-bold text-eon-text">Dashboard</h1>
          <p className="text-eon-textSecondary mt-2">
            Monitor your AI agent ecosystem at a glance
          </p>
        </div>
        <Link
          to="/tasks"
          className="btn-primary flex items-center gap-2"
        >
          <Plus size={20} />
          <span>New Task</span>
        </Link>
      </div>

      {/* System Health */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div className="card">
          <div className="flex items-center gap-4">
            <div className={`p-3 rounded-lg ${health?.status === 'healthy' ? 'bg-eon-success/20' : 'bg-eon-error/20'}`}>
              <Activity size={24} className={health?.status === 'healthy' ? 'text-eon-success' : 'text-eon-error'} />
            </div>
            <div>
              <p className="text-sm text-eon-textSecondary">System Status</p>
              <p className="text-xl font-semibold text-eon-text">
                {health?.status === 'healthy' ? 'Healthy' : 'Degraded'}
              </p>
            </div>
          </div>
        </div>

        <div className="card">
          <div className="flex items-center gap-4">
            <div className="p-3 rounded-lg bg-eon-primary/20">
              <Bot size={24} className="text-eon-primary" />
            </div>
            <div>
              <p className="text-sm text-eon-textSecondary">Active Agents</p>
              <p className="text-xl font-semibold text-eon-text">
                {activeAgents.length} / {agents.length}
              </p>
            </div>
          </div>
        </div>

        <div className="card">
          <div className="flex items-center gap-4">
            <div className="p-3 rounded-lg bg-eon-secondary/20">
              <ListTodo size={24} className="text-eon-secondary" />
            </div>
            <div>
              <p className="text-sm text-eon-textSecondary">Tasks</p>
              <p className="text-xl font-semibold text-eon-text">
                {tasks.length} total
              </p>
            </div>
          </div>
        </div>
      </div>

      {/* Quick Stats */}
      {systemStatus && (
        <div className="card">
          <h2 className="text-lg font-semibold text-eon-text mb-4">Task Statistics</h2>
          <div className="grid grid-cols-2 md:grid-cols-4 gap-6">
            <div>
              <p className="text-sm text-eon-textSecondary">Pending</p>
              <p className="text-2xl font-bold text-eon-text">{systemStatus.task_stats.pending}</p>
            </div>
            <div>
              <p className="text-sm text-eon-textSecondary">Dispatched</p>
              <p className="text-2xl font-bold text-eon-warning">{systemStatus.task_stats.dispatched}</p>
            </div>
            <div>
              <p className="text-sm text-eon-textSecondary">Completed</p>
              <p className="text-2xl font-bold text-eon-success">{systemStatus.task_stats.completed}</p>
            </div>
            <div>
              <p className="text-sm text-eon-textSecondary">Failed</p>
              <p className="text-2xl font-bold text-eon-error">{systemStatus.task_stats.failed}</p>
            </div>
          </div>
        </div>
      )}

      {/* Recent Tasks */}
      <div className="card">
        <div className="flex justify-between items-center mb-4">
          <h2 className="text-lg font-semibold text-eon-text">Recent Tasks</h2>
          <Link
            to="/tasks"
            className="text-eon-primary hover:text-blue-400 flex items-center gap-1 text-sm"
          >
            View all <ArrowRight size={16} />
          </Link>
        </div>

        {recentTasks.length === 0 ? (
          <div className="text-center py-12 text-eon-textSecondary">
            <ListTodo size={48} className="mx-auto mb-4 opacity-50" />
            <p>No tasks yet</p>
            <p className="text-sm">Submit your first task to get started</p>
          </div>
        ) : (
          <div className="space-y-3">
            {recentTasks.map((task) => (
              <div
                key={task.id}
                className="flex items-center justify-between p-4 bg-eon-surfaceLight rounded-lg"
              >
                <div className="flex-1">
                  <p className="text-eon-text font-medium line-clamp-1">{task.user_query}</p>
                  <div className="flex items-center gap-4 mt-2">
                    <span
                      className={`text-xs px-2 py-1 rounded ${
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
                    </span>
                    <span className="text-xs text-eon-textSecondary">
                      {formatDistanceToNow(task.created_at)} ago
                    </span>
                  </div>
                </div>
                <Link
                  to={`/tasks/${task.id}`}
                  className="ml-4 p-2 hover:bg-eon-surface rounded-lg"
                >
                  <ArrowRight size={20} className="text-eon-textSecondary" />
                </Link>
              </div>
            ))}
          </div>
        )}
      </div>

      {/* Performance Stats */}
      {stats && stats.by_agent.length > 0 && (
        <div className="card">
          <h2 className="text-lg font-semibold text-eon-text mb-4">Agent Performance</h2>
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead>
                <tr className="border-b border-eon-border">
                  <th className="text-left py-3 px-4 text-eon-textSecondary">Agent</th>
                  <th className="text-right py-3 px-4 text-eon-textSecondary">Tasks</th>
                  <th className="text-right py-3 px-4 text-eon-textSecondary">Success Rate</th>
                  <th className="text-right py-3 px-4 text-eon-textSecondary">Avg Latency</th>
                </tr>
              </thead>
              <tbody>
                {stats.by_agent.slice(0, 5).map((agent) => (
                  <tr key={agent.agent_id} className="border-b border-eon-border/50">
                    <td className="py-3 px-4 text-eon-text">{agent.agent_name}</td>
                    <td className="py-3 px-4 text-right text-eon-text">{agent.total_tasks}</td>
                    <td className="py-3 px-4 text-right">
                      <span
                        className={`${
                          agent.success_rate >= 90
                            ? 'text-eon-success'
                            : agent.success_rate >= 70
                            ? 'text-eon-warning'
                            : 'text-eon-error'
                        }`}
                      >
                        {agent.success_rate.toFixed(1)}%
                      </span>
                    </td>
                    <td className="py-3 px-4 text-right text-eon-text">{agent.avg_latency_ms.toFixed(0)}ms</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      )}
    </div>
  );
};

export default Dashboard;
