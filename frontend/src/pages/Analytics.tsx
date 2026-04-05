import React, { useEffect } from 'react';
import { BarChart, Bar, XAxis, YAxis, CartesianGrid, Tooltip, Legend, ResponsiveContainer, PieChart, Pie, Cell } from 'recharts';
import { useStore } from '../store/useStore';
import { formatPercentage, formatLatency } from '../utils/format';

const Analytics: React.FC = () => {
  const { stats, fetchStats } = useStore();

  useEffect(() => {
    fetchStats();
  }, [fetchStats]);

  if (!stats) {
    return <div className="text-center py-12 text-eon-textSecondary">Loading analytics...</div>;
  }

  // Prepare data for charts
  const agentPerformanceData = stats.by_agent.map((agent) => ({
    name: agent.agent_name.length > 20 ? agent.agent_name.substring(0, 20) + '...' : agent.agent_name,
    successRate: agent.success_rate,
    avgLatency: agent.avg_latency_ms,
    tasks: agent.total_tasks,
  }));

  const statusData = [
    { name: 'Success', value: stats.success_rate, color: '#10b981' },
    { name: 'Failure', value: 100 - stats.success_rate, color: '#ef4444' },
  ];

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold text-eon-text">Analytics</h1>
        <p className="text-eon-textSecondary mt-2">
          Monitor system performance and agent effectiveness
        </p>
      </div>

      {/* Key Metrics */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
        <div className="card">
          <p className="text-sm text-eon-textSecondary mb-1">Total Executions</p>
          <p className="text-3xl font-bold text-eon-text">{stats.total_executions.toLocaleString()}</p>
        </div>
        <div className="card">
          <p className="text-sm text-eon-textSecondary mb-1">Success Rate</p>
          <p className={`text-3xl font-bold ${stats.success_rate >= 90 ? 'text-eon-success' : stats.success_rate >= 70 ? 'text-eon-warning' : 'text-eon-error'}`}>
            {formatPercentage(stats.success_rate)}
          </p>
        </div>
        <div className="card">
          <p className="text-sm text-eon-textSecondary mb-1">Avg Latency</p>
          <p className="text-3xl font-bold text-eon-text">{formatLatency(stats.avg_latency_ms)}</p>
        </div>
        <div className="card">
          <p className="text-sm text-eon-textSecondary mb-1">Active Agents</p>
          <p className="text-3xl font-bold text-eon-text">{stats.by_agent.length}</p>
        </div>
      </div>

      {/* Charts Grid */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Success Rate by Agent */}
        <div className="card">
          <h2 className="text-lg font-semibold text-eon-text mb-4">Success Rate by Agent</h2>
          <ResponsiveContainer width="100%" height={300}>
            <BarChart data={agentPerformanceData} layout="vertical">
              <CartesianGrid strokeDasharray="3 3" stroke="#374151" />
              <XAxis type="number" domain={[0, 100]} tick={{ fill: '#9ca3af' }} />
              <YAxis dataKey="name" type="category" width={150} tick={{ fill: '#9ca3af', fontSize: 12 }} />
              <Tooltip
                contentStyle={{ backgroundColor: '#1a1a1a', border: '1px solid #374151', borderRadius: '8px' }}
                labelStyle={{ color: '#f3f4f6' }}
                formatter={(value: number) => [`${value.toFixed(1)}%`, 'Success Rate']}
              />
              <Bar dataKey="successRate" fill="#3B82F6" radius={[0, 4, 4, 0]} />
            </BarChart>
          </ResponsiveContainer>
        </div>

        {/* Overall Success Rate Pie */}
        <div className="card">
          <h2 className="text-lg font-semibold text-eon-text mb-4">Overall Success Rate</h2>
          <ResponsiveContainer width="100%" height={300}>
            <PieChart>
              <Pie
                data={statusData}
                cx="50%"
                cy="50%"
                labelLine={false}
                label={({ name, percent }) => `${name}: ${(percent * 100).toFixed(0)}%`}
                outerRadius={80}
                fill="#8884d8"
                dataKey="value"
              >
                {statusData.map((entry, index) => (
                  <Cell key={`cell-${index}`} fill={entry.color} />
                ))}
              </Pie>
              <Tooltip
                contentStyle={{ backgroundColor: '#1a1a1a', border: '1px solid #374151', borderRadius: '8px' }}
                formatter={(value: number) => [`${value.toFixed(1)}%`, '']}
              />
              <Legend />
            </PieChart>
          </ResponsiveContainer>
        </div>

        {/* Avg Latency by Agent */}
        <div className="card lg:col-span-2">
          <h2 className="text-lg font-semibold text-eon-text mb-4">Average Latency by Agent</h2>
          <ResponsiveContainer width="100%" height={300}>
            <BarChart data={agentPerformanceData}>
              <CartesianGrid strokeDasharray="3 3" stroke="#374151" />
              <XAxis dataKey="name" tick={{ fill: '#9ca3af', fontSize: 12 }} angle={-45} textAnchor="end" height={100} />
              <YAxis tick={{ fill: '#9ca3af' }} />
              <Tooltip
                contentStyle={{ backgroundColor: '#1a1a1a', border: '1px solid #374151', borderRadius: '8px' }}
                formatter={(value: number) => [formatLatency(value), 'Avg Latency']}
              />
              <Bar dataKey="avgLatency" fill="#8B5CF6" radius={[4, 4, 0, 0]} />
            </BarChart>
          </ResponsiveContainer>
        </div>

        {/* Agent Performance Table */}
        <div className="card lg:col-span-2">
          <h2 className="text-lg font-semibold text-eon-text mb-4">Agent Performance Details</h2>
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead>
                <tr className="border-b border-eon-border">
                  <th className="text-left py-3 px-4 text-eon-textSecondary font-medium">Agent</th>
                  <th className="text-right py-3 px-4 text-eon-textSecondary font-medium">Total Tasks</th>
                  <th className="text-right py-3 px-4 text-eon-textSecondary font-medium">Success</th>
                  <th className="text-right py-3 px-4 text-eon-textSecondary font-medium">Failed</th>
                  <th className="text-right py-3 px-4 text-eon-textSecondary font-medium">Success Rate</th>
                  <th className="text-right py-3 px-4 text-eon-textSecondary font-medium">Avg Latency</th>
                </tr>
              </thead>
              <tbody>
                {stats.by_agent.map((agent) => {
                  const failed = agent.total_tasks - Math.round((agent.success_rate / 100) * agent.total_tasks);
                  return (
                    <tr key={agent.agent_id} className="border-b border-eon-border/50 hover:bg-eon-surfaceLight transition-colors">
                      <td className="py-3 px-4 text-eon-text font-medium">{agent.agent_name}</td>
                      <td className="py-3 px-4 text-right text-eon-text">{agent.total_tasks}</td>
                      <td className="py-3 px-4 text-right text-eon-success">
                        {Math.round((agent.success_rate / 100) * agent.total_tasks)}
                      </td>
                      <td className="py-3 px-4 text-right text-eon-error">{failed}</td>
                      <td className="py-3 px-4 text-right">
                        <span
                          className={`font-medium ${
                            agent.success_rate >= 90
                              ? 'text-eon-success'
                              : agent.success_rate >= 70
                              ? 'text-eon-warning'
                              : 'text-eon-error'
                          }`}
                        >
                          {formatPercentage(agent.success_rate)}
                        </span>
                      </td>
                      <td className="py-3 px-4 text-right text-eon-text">{formatLatency(agent.avg_latency_ms)}</td>
                    </tr>
                  );
                })}
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Analytics;
