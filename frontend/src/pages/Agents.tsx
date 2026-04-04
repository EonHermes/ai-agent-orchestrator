import React, { useEffect, useState } from 'react';
import { useStore } from '../store';
import { Plus, Edit2, Trash2, Server, Activity } from 'lucide-react';
import type { CreateAgentRequest, AgentStats } from '../types';

const Agents: React.FC = () => {
  const { agents, agentStats, loadingAgents, fetchAgents, createAgent, deleteAgent, fetchAgentStats } = useStore();
  const [showCreate, setShowCreate] = useState(false);
  const [newAgent, setNewAgent] = useState<CreateAgentRequest>({
    name: '',
    description: '',
    endpoint_url: '',
    capabilities: [],
  });
  const [capabilityInput, setCapabilityInput] = useState('');
  const [selectedAgent, setSelectedAgent] = useState<AgentStats | null>(null);

  useEffect(() => {
    fetchAgents();
  }, [fetchAgents]);

  const handleCreate = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await createAgent(newAgent);
      setShowCreate(false);
      setNewAgent({ name: '', description: '', endpoint_url: '', capabilities: [] });
      setCapabilityInput('');
    } catch (error) {
      console.error('Failed to create agent:', error);
    }
  };

  const handleDelete = async (id: string) => {
    if (window.confirm('Are you sure you want to delete this agent?')) {
      await deleteAgent(id);
    }
  };

  const handleViewStats = async (id: string) => {
    await fetchAgentStats(id);
    const stats = useStore.getState().agentStats[id];
    if (stats) setSelectedAgent(stats);
  };

  const addCapability = () => {
    if (capabilityInput.trim() && !newAgent.capabilities.includes(capabilityInput.trim())) {
      setNewAgent({ ...newAgent, capabilities: [...newAgent.capabilities, capabilityInput.trim()] });
      setCapabilityInput('');
    }
  };

  const removeCapability = (cap: string) => {
    setNewAgent({
      ...newAgent,
      capabilities: newAgent.capabilities.filter(c => c !== cap)
    });
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-3xl font-bold">Agents</h1>
        <button
          onClick={() => setShowCreate(!showCreate)}
          className="bg-blue-600 hover:bg-blue-700 px-4 py-2 rounded-lg flex items-center gap-2"
        >
          <Plus className="w-5 h-5" />
          Register Agent
        </button>
      </div>

      {/* Create Form */}
      {showCreate && (
        <div className="bg-gray-800 rounded-lg p-6 shadow">
          <h2 className="text-xl font-semibold mb-4">Register New Agent</h2>
          <form onSubmit={handleCreate} className="space-y-4">
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <label className="block text-sm text-gray-400 mb-1">Name</label>
                <input
                  type="text"
                  required
                  className="w-full bg-gray-700 border border-gray-600 rounded px-3 py-2"
                  value={newAgent.name}
                  onChange={(e) => setNewAgent({ ...newAgent, name: e.target.value })}
                  placeholder="e.g., Workflow Assistant"
                />
              </div>
              <div>
                <label className="block text-sm text-gray-400 mb-1">Endpoint URL</label>
                <input
                  type="url"
                  required
                  className="w-full bg-gray-700 border border-gray-600 rounded px-3 py-2"
                  value={newAgent.endpoint_url}
                  onChange={(e) => setNewAgent({ ...newAgent, endpoint_url: e.target.value })}
                  placeholder="http://agent:8081"
                />
              </div>
            </div>
            <div>
              <label className="block text-sm text-gray-400 mb-1">Description</label>
              <textarea
                className="w-full bg-gray-700 border border-gray-600 rounded px-3 py-2"
                rows={2}
                value={newAgent.description || ''}
                onChange={(e) => setNewAgent({ ...newAgent, description: e.target.value })}
                placeholder="What does this agent do?"
              />
            </div>
            <div>
              <label className="block text-sm text-gray-400 mb-1">Capabilities</label>
              <div className="flex gap-2 mb-2">
                <input
                  type="text"
                  className="flex-1 bg-gray-700 border border-gray-600 rounded px-3 py-2"
                  value={capabilityInput}
                  onChange={(e) => setCapabilityInput(e.target.value)}
                  placeholder="e.g., workflow_optimization"
                  onKeyPress={(e) => e.key === 'Enter' && (e.preventDefault(), addCapability())}
                />
                <button type="button" onClick={addCapability} className="bg-gray-600 hover:bg-gray-700 px-4 py-2 rounded">
                  Add
                </button>
              </div>
              <div className="flex flex-wrap gap-2">
                {newAgent.capabilities.map((cap) => (
                  <span
                    key={cap}
                    className="bg-gray-700 px-3 py-1 rounded-full text-sm flex items-center gap-2"
                  >
                    {cap}
                    <button type="button" onClick={() => removeCapability(cap)} className="text-gray-400 hover:text-red-400">
                      ×
                    </button>
                  </span>
                ))}
              </div>
              <p className="text-xs text-gray-500 mt-1">
                Leave empty and provide a description for AI to suggest capabilities.
              </p>
            </div>
            <div className="flex gap-2">
              <button type="submit" className="bg-blue-600 hover:bg-blue-700 px-4 py-2 rounded">
                Create Agent
              </button>
              <button type="button" onClick={() => setShowCreate(false)} className="bg-gray-600 hover:bg-gray-700 px-4 py-2 rounded">
                Cancel
              </button>
            </div>
          </form>
        </div>
      )}

      {/* Agents List */}
      {loadingAgents ? (
        <p className="text-gray-400">Loading agents...</p>
      ) : agents.length === 0 ? (
        <div className="bg-gray-800 rounded-lg p-8 text-center">
          <Server className="w-12 h-12 mx-auto mb-4 text-gray-600" />
          <p className="text-gray-400">No agents registered yet.</p>
          <p className="text-sm text-gray-500 mt-2">Register your first agent to get started.</p>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {agents.map((agent) => (
            <div key={agent.id} className="bg-gray-800 rounded-lg p-6 shadow">
              <div className="flex items-start justify-between">
                <div>
                  <h3 className="text-lg font-semibold">{agent.name}</h3>
                  <p className="text-sm text-gray-400 mt-1 line-clamp-2">{agent.description || 'No description'}</p>
                </div>
                <span className={`px-2 py-1 rounded text-xs font-medium ${
                  agent.status === 'active' ? 'bg-green-500/20 text-green-400' :
                  agent.status === 'error' ? 'bg-red-500/20 text-red-400' :
                  'bg-gray-500/20 text-gray-400'
                }`}>
                  {agent.status}
                </span>
              </div>

              <div className="mt-4">
                <p className="text-sm text-gray-400 mb-2">Capabilities</p>
                <div className="flex flex-wrap gap-2">
                  {agent.capabilities.map((cap) => (
                    <span key={cap} className="bg-blue-500/20 text-blue-300 px-2 py-1 rounded text-xs">
                      {cap}
                    </span>
                  ))}
                </div>
              </div>

              <div className="mt-4 text-sm text-gray-500">
                <p>{agent.endpoint_url}</p>
              </div>

              <div className="mt-4 flex gap-2">
                <button
                  onClick={() => handleViewStats(agent.id)}
                  className="flex-1 bg-gray-700 hover:bg-gray-600 px-3 py-2 rounded text-sm flex items-center justify-center gap-2"
                >
                  <Activity className="w-4 h-4" />
                  Stats
                </button>
                <button
                  onClick={() => handleDelete(agent.id)}
                  className="bg-red-500/20 hover:bg-red-500/30 text-red-400 px-3 py-2 rounded text-sm"
                >
                  <Trash2 className="w-4 h-4" />
                </button>
              </div>

              {/* Stats Modal */}
              {selectedAgent?.agent.id === agent.id && (
                <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
                  <div className="bg-gray-800 rounded-lg p-6 max-w-2xl w-full max-h-96 overflow-y-auto">
                    <div className="flex justify-between items-center mb-4">
                      <h3 className="text-xl font-semibold">{agent.name} - Performance</h3>
                      <button onClick={() => setSelectedAgent(null)} className="text-gray-400 hover:text-white">×</button>
                    </div>
                    <div className="grid grid-cols-2 gap-4">
                      <div className="bg-gray-700 rounded p-4">
                        <p className="text-sm text-gray-400">Total Executions</p>
                        <p className="text-2xl font-bold">{selectedAgent.total_executions}</p>
                      </div>
                      <div className="bg-gray-700 rounded p-4">
                        <p className="text-sm text-gray-400">Success Rate</p>
                        <p className="text-2xl font-bold">{(selectedAgent.success_rate * 100).toFixed(1)}%</p>
                      </div>
                      <div className="bg-gray-700 rounded p-4 col-span-2">
                        <p className="text-sm text-gray-400 mb-2">Avg Latency</p>
                        <p className="text-2xl font-bold">
                          {selectedAgent.avg_latency_ms ? `${selectedAgent.avg_latency_ms.toFixed(0)}ms` : 'N/A'}
                        </p>
                      </div>
                    </div>
                    <div className="mt-4">
                      <p className="text-sm text-gray-400 mb-2">Top Capabilities</p>
                      <div className="space-y-2">
                        {selectedAgent.top_capabilities.map(([cap, count]) => (
                          <div key={cap} className="flex justify-between bg-gray-700 rounded px-3 py-2">
                            <span>{cap}</span>
                            <span className="text-blue-400">{count} runs</span>
                          </div>
                        ))}
                      </div>
                    </div>
                  </div>
                </div>
              )}
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

export default Agents;