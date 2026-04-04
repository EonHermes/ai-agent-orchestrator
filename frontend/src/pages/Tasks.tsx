import React, { useEffect, useState } from 'react';
import { useStore } from '../store';
import { Send, Eye, Play, X, Clock, CheckCircle, AlertCircle } from 'lucide-react';
import type { ParseResponse } from '../types';

const Tasks: React.FC = () => {
  const {
    tasks,
    activeTask,
    activeTaskDetails,
    loadingTasks,
    fetchTasks,
    createTask,
    cancelTask,
    getTask,
    parseTask,
  } = useStore();

  const [query, setQuery] = useState('');
  const [parseResult, setParseResult] = useState<ParseResponse | null>(null);
  const [parsing, setParsing] = useState(false);

  useEffect(() => {
    fetchTasks();
  }, [fetchTasks]);

  const handleParse = async () => {
    if (!query.trim()) return;
    setParsing(true);
    try {
      const result = await parseTask(query);
      setParseResult(result);
    } catch (e) {
      console.error('Parse failed:', e);
    } finally {
      setParsing(false);
    }
  };

  const handleSubmit = async () => {
    if (!query.trim()) return;
    await createTask(query);
    setQuery('');
    setParseResult(null);
  };

  const handleViewTask = async (id: string) => {
    await getTask(id);
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'pending': return <Clock className="w-4 h-4 text-yellow-500" />;
      case 'dispatched': return <Play className="w-4 h-4 text-blue-500 animate-pulse" />;
      case 'completed': return <CheckCircle className="w-4 h-4 text-green-500" />;
      case 'failed': return <AlertCircle className="w-4 h-4 text-red-500" />;
      case 'cancelled': return <X className="w-4 h-4 text-gray-500" />;
      default: return <Clock className="w-4 h-4" />;
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-3xl font-bold">Tasks</h1>
      </div>

      {/* Task Input */}
      <div className="bg-gray-800 rounded-lg p-6 shadow">
        <h2 className="text-xl font-semibold mb-4">Submit New Task</h2>
        <div className="space-y-4">
          <textarea
            className="w-full bg-gray-700 border border-gray-600 rounded px-3 py-3 min-h-[100px]"
            placeholder="Describe your task in natural language... (e.g., 'Analyze workflow performance and identify bottlenecks')"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
          />
          <div className="flex gap-2">
            <button
              onClick={handleParse}
              disabled={parsing || !query.trim()}
              className="bg-gray-600 hover:bg-gray-700 disabled:bg-gray-700 px-4 py-2 rounded flex items-center gap-2"
            >
              <Eye className="w-4 h-4" />
              {parsing ? 'Parsing...' : 'Preview Plan'}
            </button>
            <button
              onClick={handleSubmit}
              disabled={!query.trim()}
              className="bg-blue-600 hover:bg-blue-700 disabled:bg-blue-800 px-4 py-2 rounded flex items-center gap-2 flex-1"
            >
              <Send className="w-4 h-4" />
              Execute
            </button>
          </div>
        </div>

        {/* Parse Result */}
        {parseResult && (
          <div className="mt-4 p-4 bg-gray-700 rounded">
            <h3 className="font-semibold mb-2">Execution Plan</h3>
            <p className="text-sm text-gray-400 mb-3">{parseResult.reasoning}</p>
            <ul className="space-y-2">
              {parseResult.plan.map((step, idx) => (
                <li key={idx} className="flex items-start gap-2 text-sm">
                  <span className="text-blue-400 font-mono">{idx + 1}.</span>
                  <div>
                    <span className="font-medium">{step.capability}</span>
                    <p className="text-gray-400 text-xs">{step.description}</p>
                  </div>
                </li>
              ))}
            </ul>
          </div>
        )}
      </div>

      {/* Active Task Details */}
      {activeTaskDetails && (
        <div className="bg-gray-800 rounded-lg p-6 shadow">
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-xl font-semibold">Task: {activeTaskDetails.task.user_query}</h2>
            {activeTaskDetails.task.status !== 'completed' && activeTaskDetails.task.status !== 'cancelled' && (
              <button
                onClick={() => cancelTask(activeTaskDetails.task.id)}
                className="bg-red-500/20 hover:bg-red-500/30 text-red-400 px-3 py-2 rounded flex items-center gap-2 text-sm"
              >
                <X className="w-4 h-4" />
                Cancel
              </button>
            )}
          </div>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {activeTaskDetails.sub_tasks.map((sub) => (
              <div key={sub.id} className="bg-gray-700 rounded p-4">
                <div className="flex justify-between items-start">
                  <div>
                    <div className="flex items-center gap-2 mb-1">
                      <span className="font-medium">{sub.capability}</span>
                      {getStatusIcon(sub.status)}
                    </div>
                    <p className="text-sm text-gray-400">Agent: {sub.agent_name || 'Unassigned'}</p>
                    {sub.latency_ms && (
                      <p className="text-xs text-gray-500 mt-1">{sub.latency_ms}ms</p>
                    )}
                  </div>
                </div>
                {sub.error && (
                  <p className="text-red-400 text-sm mt-2 bg-red-500/10 p-2 rounded">{sub.error}</p>
                )}
                {sub.output && (
                  <details className="mt-2">
                    <summary className="text-sm text-gray-400 cursor-pointer">View Output</summary>
                    <pre className="mt-2 text-xs bg-gray-900 rounded p-2 overflow-auto max-h-48">
                      {JSON.stringify(sub.output, null, 2)}
                    </pre>
                  </details>
                )}
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Task History */}
      <div className="bg-gray-800 rounded-lg p-6 shadow">
        <h2 className="text-xl font-semibold mb-4">Task History</h2>
        {loadingTasks ? (
          <p className="text-gray-400">Loading...</p>
        ) : tasks.length === 0 ? (
          <p className="text-gray-500">No tasks yet.</p>
        ) : (
          <div className="overflow-x-auto">
            <table className="w-full text-sm">
              <thead>
                <tr className="border-b border-gray-700">
                  <th className="text-left py-3">Query</th>
                  <th className="text-left py-3">Status</th>
                  <th className="text-left py-3">Created</th>
                  <th className="text-left py-3">Duration</th>
                  <th className="text-right py-3">Actions</th>
                </tr>
              </thead>
              <tbody>
                {tasks.map((task) => {
                  const details = activeTaskDetails?.task.id === task.id ? activeTaskDetails : null;
                  const duration = task.completed_at && task.started_at
                    ? `${Math.round((new Date(task.completed_at).getTime() - new Date(task.started_at).getTime()) / 1000)}s`
                    : task.started_at ? 'Running...' : '-';

                  return (
                    <tr key={task.id} className="border-b border-gray-800 hover:bg-gray-750">
                      <td className="py-3 max-w-md truncate" title={task.user_query}>
                        {task.user_query}
                      </td>
                      <td className="py-3">
                        <span className={`px-2 py-1 rounded text-xs font-medium flex items-center w-fit gap-1 ${
                          task.status === 'completed' ? 'bg-green-500/20 text-green-400' :
                          task.status === 'failed' ? 'bg-red-500/20 text-red-400' :
                          task.status === 'cancelled' ? 'bg-gray-500/20 text-gray-400' :
                          task.status === 'dispatched' ? 'bg-blue-500/20 text-blue-400' :
                          'bg-yellow-500/20 text-yellow-400'
                        }`}>
                          {getStatusIcon(task.status)}
                          {task.status}
                        </span>
                      </td>
                      <td className="py-3 text-gray-400">
                        {new Date(task.created_at).toLocaleString()}
                      </td>
                      <td className="py-3 text-gray-400">{duration}</td>
                      <td className="py-3 text-right">
                        <button
                          onClick={() => handleViewTask(task.id)}
                          className="text-blue-400 hover:text-blue-300"
                          title="View details"
                        >
                          <Eye className="w-4 h-4 inline" />
                        </button>
                      </td>
                    </tr>
                  );
                })}
              </tbody>
            </table>
          </div>
        )}
      </div>
    </div>
  );
};

export default Tasks;