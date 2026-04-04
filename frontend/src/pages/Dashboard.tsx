import React, { useEffect } from 'react';
import { useStore } from '../store';
import {
  Activity, Cpu, CheckCircle, AlertCircle, Clock, Server, BarChart2
} from 'lucide-react';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer, PieChart, Pie, Cell } from 'recharts';

const Dashboard: React.FC = () => {
  const { status, agents, tasks, executions, activeTaskDetails, refreshStatus, fetchAgents, fetchTasks, fetchExecutions } = useStore();

  useEffect(() => {
    refreshStatus();
    fetchAgents();
    fetchTasks();
    fetchExecutions();
    // Refresh every 30 seconds
    const interval = setInterval(() => {
      refreshStatus();
      fetchAgents();
      fetchTasks();
    }, 30000);
    return () => clearInterval(interval);
  }, [refreshStatus, fetchAgents, fetchTasks, fetchExecutions]);

  const COLORS = ['#10b981', '#ef4444', '#f59e0b', '#6b7280'];

  const statusData = [
    { name: 'Active', value: status?.active_agents || 0, color: '#10b981' },
    { name: 'Inactive', value: (status?.total_agents || 0) - (status?.active_agents || 0), color: '#6b7280' },
  ];

  const recentExecutions = executions.slice(0, 10);

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-3xl font-bold">Dashboard</h1>
        <div className="flex items-center gap-2 text-sm text-gray-400">
          <Clock className="w-4 h-4" />
          {new Date().toLocaleTimeString()}
        </div>
      </div>

      {/* Stats Cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <StatCard
          icon={<Cpu className="w-6 h-6" />}
          title="Total Agents"
          value={String(status?.total_agents || 0)}
          subtitle={`${status?.active_agents || 0} active`}
        />
        <StatCard
          icon={<Activity className="w-6 h-6" />}
          title="Active Tasks"
          value={String(status?.active_tasks || 0)}
          subtitle={`${status?.completed_tasks || 0} completed`}
        />
        <StatCard
          icon={<CheckCircle className="w-6 h-6" />}
          title="Success Rate"
          value={`${((status?.completed_tasks || 0) / Math.max(status?.total_tasks || 1, 1) * 100).toFixed(1)}%`}
          subtitle={`${status?.failed_tasks || 0} failed`}
        />
        <StatCard
          icon={<Clock className="w-6 h-6" />}
          title="Avg Latency"
          value={status?.avg_task_latency_ms ? `${(status.avg_task_latency_ms / 1000).toFixed(1)}s` : 'N/A'}
          subtitle="per task"
        />
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Agent Status */}
        <div className="bg-gray-800 rounded-lg p-6 shadow">
          <h2 className="text-xl font-semibold mb-4 flex items-center gap-2">
            <Server className="w-5 h-5" />
            Agent Status
          </h2>
          <div className="h-64 flex items-center justify-center">
            <ResponsiveContainer width="100%" height="100%">
              <PieChart>
                <Pie
                  data={statusData}
                  dataKey="value"
                  nameKey="name"
                  cx="50%"
                  cy="50%"
                  outerRadius={80}
                  label={({ name, percent }) => `${name}: ${(percent * 100).toFixed(0)}%`}
                >
                  {statusData.map((entry, index) => (
                    <Cell key={`cell-${index}`} fill={entry.color} />
                  ))}
                </Pie>
                <Tooltip />
              </PieChart>
            </ResponsiveContainer>
          </div>
          <div className="mt-4 text-center text-sm text-gray-400">
            {agents.length} agent{agents.length !== 1 ? 's' : ''} registered
          </div>
        </div>

        {/* Recent Task Success */}
        <div className="bg-gray-800 rounded-lg p-6 shadow">
          <h2 className="text-xl font-semibold mb-4 flex items-center gap-2">
            <BarChart2 className="w-5 h-5" />
            Recent Executions
          </h2>
          <div className="h-64 overflow-y-auto">
            {recentExecutions.length === 0 ? (
              <p className="text-gray-500 text-center mt-20">No executions yet</p>
            ) : (
              <table className="w-full text-sm">
                <thead>
                  <tr className="border-b border-gray-700">
                    <th className="text-left py-2">Agent</th>
                    <th className="text-left py-2">Action</th>
                    <th className="text-right py-2">Latency</th>
                    <th className="text-right py-2">Status</th>
                  </tr>
                </thead>
                <tbody>
                  {recentExecutions.map((exec) => (
                    <tr key={exec.id} className="border-b border-gray-800">
                      <td className="py-2">{exec.agent_name || 'Unknown'}</td>
                      <td className="py-2">{exec.action}</td>
                      <td className="text-right py-2">{exec.latency_ms ? `${exec.latency_ms}ms` : '-'}</td>
                      <td className="text-right py-2">
                        {exec.success ? (
                          <CheckCircle className="w-5 h-5 text-green-500 inline" />
                        ) : (
                          <AlertCircle className="w-5 h-5 text-red-500 inline" />
                        )}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            )}
          </div>
        </div>
      </div>

      {/* Active Task Details */}
      {activeTaskDetails && (
        <div className="bg-gray-800 rounded-lg p-6 shadow">
          <h2 className="text-xl font-semibold mb-4 flex items-center gap-2">
            <Activity className="w-5 h-5" />
            Active Task: {activeTaskDetails.task.user_query.substring(0, 60)}...
          </h2>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            {activeTaskDetails.sub_tasks.map((sub) => (
              <div key={sub.id} className="bg-gray-700 rounded p-4">
                <div className="flex justify-between items-start">
                  <div>
                    <p className="font-medium">{sub.capability}</p>
                    <p className="text-sm text-gray-400">Agent: {sub.agent_name || 'Unassigned'}</p>
                  </div>
                  <StatusBadge status={sub.status} />
                </div>
                {sub.error && (
                  <p className="text-red-400 text-sm mt-2">{sub.error}</p>
                )}
                {sub.output && (
                  <pre className="mt-2 text-xs bg-gray-900 rounded p-2 overflow-auto max-h-40">
                    {JSON.stringify(sub.output, null, 2)}
                  </pre>
                )}
              </div>
            ))}
          </div>
        </div>
      )}

      {/* System Health */}
      <div className="bg-gray-800 rounded-lg p-6 shadow">
        <h2 className="text-xl font-semibold mb-4 flex items-center gap-2">
          <Activity className="w-5 h-5" />
          System Health
        </h2>
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
          <div className="bg-gray-700 rounded p-4">
            <p className="text-sm text-gray-400">Database</p>
            <p className="font-medium text-green-400">Connected</p>
          </div>
          <div className="bg-gray-700 rounded p-4">
            <p className="text-sm text-gray-400">API Latency</p>
            <p className="font-medium">{status?.avg_task_latency_ms ? `${(status.avg_task_latency_ms / 1000).toFixed(2)}s` : 'N/A'}</p>
          </div>
          <div className="bg-gray-700 rounded p-4">
            <p className="text-sm text-gray-400">Queue Depth</p>
            <p className="font-medium">{status?.active_tasks || 0} / {status?.total_tasks || 0}</p>
          </div>
          <div className="bg-gray-700 rounded p-4">
            <p className="text-sm text-gray-400">Success Rate</p>
            <p className="font-medium">
              {status?.total_tasks ? `${((status.completed_tasks || 0) / status.total_tasks * 100).toFixed(1)}%` : 'N/A'}
            </p>
          </div>
        </div>
      </div>
    </div>
  );
};

interface StatCardProps {
  icon: React.ReactNode;
  title: string;
  value: string;
  subtitle: string;
}

const StatCard: React.FC<StatCardProps> = ({ icon, title, value, subtitle }) => (
  <div className="bg-gray-800 rounded-lg p-6 shadow">
    <div className="flex items-center justify-between">
      <div className="text-blue-400">{icon}</div>
    </div>
    <p className="text-sm text-gray-400 mt-2">{title}</p>
    <p className="text-3xl font-bold mt-1">{value}</p>
    <p className="text-sm text-gray-500">{subtitle}</p>
  </div>
);

const StatusBadge: React.FC<{ status: string }> = ({ status }) => {
  const styles = {
    pending: 'bg-yellow-500/20 text-yellow-400',
    running: 'bg-blue-500/20 text-blue-400',
    completed: 'bg-green-500/20 text-green-400',
    failed: 'bg-red-500/20 text-red-400',
  };
  return (
    <span className={`px-2 py-1 rounded text-xs font-medium ${styles[status as keyof typeof styles] || 'bg-gray-500/20 text-gray-400'}`}>
      {status}
    </span>
  );
};

export default Dashboard;