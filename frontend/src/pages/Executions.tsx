import React, { useEffect, useState } from 'react';
import { useStore } from '../store';
import { Activity, CheckCircle, AlertCircle } from 'lucide-react';

const Executions: React.FC = () => {
  const { executions, executionStats, fetchExecutions, fetchExecutionStats } = useStore();
  const [selectedExecution, setSelectedExecution] = useState<any>(null);

  useEffect(() => {
    fetchExecutions();
    fetchExecutionStats();
  }, [fetchExecutions, fetchExecutionStats]);

  return (
    <div className="space-y-6">
      <h1 className="text-3xl font-bold">Executions</h1>

      {/* Stats Summary */}
      {executionStats.length > 0 && (
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
          {executionStats.slice(0, 3).map((stat) => (
            <div key={stat.agent_id} className="bg-gray-800 rounded-lg p-6 shadow">
              <h3 className="text-lg font-semibold mb-2">{stat.agent_name}</h3>
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <p className="text-sm text-gray-400">Executions</p>
                  <p className="text-2xl font-bold">{stat.execution_count}</p>
                </div>
                <div>
                  <p className="text-sm text-gray-400">Success Rate</p>
                  <p className="text-2xl font-bold">{(stat.success_rate * 100).toFixed(1)}%</p>
                </div>
                <div className="col-span-2">
                  <p className="text-sm text-gray-400">Avg Latency</p>
                  <p className="text-xl font-bold">
                    {stat.avg_latency_ms ? `${stat.avg_latency_ms.toFixed(0)}ms` : 'N/A'}
                  </p>
                </div>
              </div>
            </div>
          ))}
        </div>
      )}

      {/* Executions List */}
      <div className="bg-gray-800 rounded-lg p-6 shadow">
        <h2 className="text-xl font-semibold mb-4 flex items-center gap-2">
          <Activity className="w-5 h-5" />
          Execution Log
        </h2>
        {executions.length === 0 ? (
          <p className="text-gray-500">No executions recorded yet.</p>
        ) : (
          <div className="overflow-x-auto">
            <table className="w-full text-sm">
              <thead>
                <tr className="border-b border-gray-700">
                  <th className="text-left py-3">Timestamp</th>
                  <th className="text-left py-3">Agent</th>
                  <th className="text-left py-3">Action</th>
                  <th className="text-right py-3">Latency</th>
                  <th className="text-center py-3">Status</th>
                </tr>
              </thead>
              <tbody>
                {executions.map((exec) => (
                  <tr
                    key={exec.id}
                    className="border-b border-gray-800 hover:bg-gray-700 cursor-pointer"
                    onClick={() => setSelectedExecution(selectedExecution?.id === exec.id ? null : exec)}
                  >
                    <td className="py-3 text-gray-400">{new Date(exec.timestamp).toLocaleString()}</td>
                    <td className="py-3">{exec.agent_name || 'System'}</td>
                    <td className="py-3">{exec.action}</td>
                    <td className="text-right py-3">{exec.latency_ms ? `${exec.latency_ms}ms` : '-'}</td>
                    <td className="text-center py-3">
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
          </div>
        )}
      </div>

      {/* Execution Detail Modal */}
      {selectedExecution && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
          <div className="bg-gray-800 rounded-lg p-6 max-w-3xl w-full max-h-96 overflow-y-auto">
            <div className="flex justify-between items-center mb-4">
              <h3 className="text-xl font-semibold">Execution Details</h3>
              <button onClick={() => setSelectedExecution(null)} className="text-gray-400 hover:text-white text-2xl">×</button>
            </div>
            <div className="space-y-4">
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <p className="text-sm text-gray-400">ID</p>
                  <p className="font-mono text-xs bg-gray-900 p-2 rounded">{selectedExecution.id}</p>
                </div>
                <div>
                  <p className="text-sm text-gray-400">Task ID</p>
                  <p className="font-mono text-xs bg-gray-900 p-2 rounded">{selectedExecution.task_id}</p>
                </div>
              </div>
              <div>
                <p className="text-sm text-gray-400">Agent</p>
                <p>{selectedExecution.agent_name || 'System'}</p>
              </div>
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <p className="text-sm text-gray-400">Action</p>
                  <p className="font-medium">{selectedExecution.action}</p>
                </div>
                <div>
                  <p className="text-sm text-gray-400">Status</p>
                  <p className={selectedExecution.success ? 'text-green-400' : 'text-red-400'}>
                    {selectedExecution.success ? 'Success' : 'Failed'}
                  </p>
                </div>
              </div>
              {selectedExecution.input_snapshot && (
                <div>
                  <p className="text-sm text-gray-400 mb-1">Input</p>
                  <pre className="bg-gray-900 p-3 rounded text-xs overflow-auto max-h-48">
                    {JSON.stringify(selectedExecution.input_snapshot, null, 2)}
                  </pre>
                </div>
              )}
              {selectedExecution.output_snapshot && (
                <div>
                  <p className="text-sm text-gray-400 mb-1">Output</p>
                  <pre className="bg-gray-900 p-3 rounded text-xs overflow-auto max-h-48">
                    {JSON.stringify(selectedExecution.output_snapshot, null, 2)}
                  </pre>
                </div>
              )}
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default Executions;