import React, { useState } from 'react';
import { Plus, Trash2, Edit2, Server } from 'lucide-react';
import { useStore } from '../store/useStore';
import type { NewAgent, Agent } from '../types';

const Agents: React.FC = () => {
  const { agents, loadingAgents, fetchAgents, createAgent, deleteAgent } = useStore();
  const [showCreateForm, setShowCreateForm] = useState(false);
  const [newAgent, setNewAgent] = useState<NewAgent>({
    name: '',
    description: '',
    endpoint_url: '',
    capabilities: [],
  });
  const [capabilityInput, setCapabilityInput] = useState('');

  const handleCreate = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await createAgent(newAgent);
      setNewAgent({ name: '', description: '', endpoint_url: '', capabilities: [] });
      setShowCreateForm(false);
    } catch (error) {
      console.error('Failed to create agent:', error);
      alert('Failed to create agent');
    }
  };

  const handleAddCapability = () => {
    if (capabilityInput.trim() && !newAgent.capabilities.includes(capabilityInput.trim())) {
      setNewAgent({
        ...newAgent,
        capabilities: [...newAgent.capabilities, capabilityInput.trim()],
      });
      setCapabilityInput('');
    }
  };

  const handleRemoveCapability = (cap: string) => {
    setNewAgent({
      ...newAgent,
      capabilities: newAgent.capabilities.filter((c) => c !== cap),
    });
  };

  const handleDelete = async (id: string) => {
    if (window.confirm('Are you sure you want to delete this agent?')) {
      try {
        await deleteAgent(id);
      } catch (error) {
        console.error('Failed to delete agent:', error);
      }
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-3xl font-bold text-eon-text">Agents</h1>
          <p className="text-eon-textSecondary mt-2">
            Manage AI agents and their capabilities
          </p>
        </div>
        <button
          onClick={() => setShowCreateForm(true)}
          className="btn-primary flex items-center gap-2"
        >
          <Plus size={20} />
          <span>New Agent</span>
        </button>
      </div>

      {/* Create Agent Modal */}
      {showCreateForm && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="card max-w-2xl w-full max-h-[90vh] overflow-y-auto">
            <h2 className="text-xl font-bold text-eon-text mb-4">Create New Agent</h2>
            <form onSubmit={handleCreate} className="space-y-4">
              <div>
                <label className="label">Name</label>
                <input
                  type="text"
                  value={newAgent.name}
                  onChange={(e) => setNewAgent({ ...newAgent, name: e.target.value })}
                  className="input w-full"
                  required
                />
              </div>

              <div>
                <label className="label">Description</label>
                <textarea
                  value={newAgent.description || ''}
                  onChange={(e) => setNewAgent({ ...newAgent, description: e.target.value })}
                  className="input w-full h-24"
                />
              </div>

              <div>
                <label className="label">Endpoint URL</label>
                <input
                  type="url"
                  value={newAgent.endpoint_url}
                  onChange={(e) => setNewAgent({ ...newAgent, endpoint_url: e.target.value })}
                  className="input w-full"
                  required
                  placeholder="http://localhost:8080"
                />
              </div>

              <div>
                <label className="label">Capabilities</label>
                <div className="flex gap-2">
                  <input
                    type="text"
                    value={capabilityInput}
                    onChange={(e) => setCapabilityInput(e.target.value)}
                    className="input flex-1"
                    placeholder="e.g., analyze_data"
                    onKeyDown={(e) => e.key === 'Enter' && (e.preventDefault(), handleAddCapability())}
                  />
                  <button
                    type="button"
                    onClick={handleAddCapability}
                    className="btn-secondary"
                  >
                    Add
                  </button>
                </div>
                <div className="flex flex-wrap gap-2 mt-2">
                  {newAgent.capabilities.map((cap) => (
                    <span
                      key={cap}
                      className="flex items-center gap-1 px-3 py-1 bg-eon-surfaceLight rounded-full text-sm"
                    >
                      {cap}
                      <button
                        type="button"
                        onClick={() => handleRemoveCapability(cap)}
                        className="text-eon-textSecondary hover:text-eon-error"
                      >
                        ×
                      </button>
                    </span>
                  ))}
                </div>
              </div>

              <div className="flex gap-3 pt-4">
                <button type="submit" className="btn-primary">
                  Create Agent
                </button>
                <button
                  type="button"
                  onClick={() => setShowCreateForm(false)}
                  className="btn-secondary"
                >
                  Cancel
                </button>
              </div>
            </form>
          </div>
        </div>
      )}

      {/* Agents List */}
      {loadingAgents ? (
        <div className="text-center py-12 text-eon-textSecondary">Loading agents...</div>
      ) : agents.length === 0 ? (
        <div className="card text-center py-12">
          <Server size={64} className="mx-auto mb-4 text-eon-textSecondary opacity-50" />
          <h3 className="text-lg font-semibold text-eon-text mb-2">No Agents Yet</h3>
          <p className="text-eon-textSecondary mb-6">
            Register your first AI agent to participate in the orchestration network
          </p>
          <button
            onClick={() => setShowCreateForm(true)}
            className="btn-primary"
          >
            Register Agent
          </button>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {agents.map((agent) => (
            <div key={agent.id} className="card relative group">
              <div className="flex justify-between items-start mb-4">
                <div className="flex items-center gap-3">
                  <div
                    className={`p-2 rounded-lg ${
                      agent.status === 'active'
                        ? 'bg-eon-success/20'
                        : agent.status === 'error'
                        ? 'bg-eon-error/20'
                        : 'bg-eon-surfaceLight'
                    }`}
                  >
                    <Server
                      size={24}
                      className={
                        agent.status === 'active'
                          ? 'text-eon-success'
                          : agent.status === 'error'
                          ? 'text-eon-error'
                          : 'text-eon-textSecondary'
                      }
                    />
                  </div>
                  <div>
                    <h3 className="font-semibold text-eon-text">{agent.name}</h3>
                    <p className="text-sm text-eon-textSecondary">{agent.status}</p>
                  </div>
                </div>
                <div className="flex gap-2 opacity-0 group-hover:opacity-100 transition-opacity">
                  <button
                    onClick={() => handleDelete(agent.id)}
                    className="p-1 text-eon-textSecondary hover:text-eon-error"
                    title="Delete agent"
                  >
                    <Trash2 size={18} />
                  </button>
                </div>
              </div>

              {agent.description && (
                <p className="text-eon-textSecondary text-sm mb-4">{agent.description}</p>
              )}

              <div className="space-y-2 text-sm mb-4">
                <div>
                  <span className="text-eon-textSecondary">Endpoint: </span>
                  <code className="text-eon-primary bg-eon-surface px-2 py-0.5 rounded text-xs">
                    {agent.endpoint_url}
                  </code>
                </div>
              </div>

              <div className="flex flex-wrap gap-2">
                {agent.capabilities.map((cap) => (
                  <span
                    key={cap}
                    className="px-2 py-1 bg-eon-surfaceLight rounded text-xs text-eon-text"
                  >
                    {cap}
                  </span>
                ))}
              </div>

              <div className="mt-4 pt-4 border-t border-eon-border text-xs text-eon-textSecondary">
                Added {new Date(agent.created_at).toLocaleDateString()}
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

export default Agents;
