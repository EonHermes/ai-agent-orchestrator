import React from 'react';
import { useStore } from '../store';
import { Activity, Server, Send, BarChart2, Menu } from 'lucide-react';

const Sidebar: React.FC = () => {
  const { activeTab, setActiveTab, sidebarOpen, toggleSidebar } = useStore();

  const navItems = [
    { id: 'dashboard' as const, label: 'Dashboard', icon: Activity },
    { id: 'agents' as const, label: 'Agents', icon: Server },
    { id: 'tasks' as const, label: 'Tasks', icon: Send },
    { id: 'executions' as const, label: 'Executions', icon: BarChart2 },
  ];

  return (
    <aside
      className={`fixed left-0 top-0 h-full bg-gray-800 border-r border-gray-700 transition-all duration-300 z-40 ${
        sidebarOpen ? 'w-64' : 'w-16'
      }`}
    >
      <div className="flex flex-col h-full">
        {/* Header */}
        <div className="flex items-center justify-between p-4 border-b border-gray-700">
          {sidebarOpen && (
            <h1 className="text-xl font-bold truncate">Orchestrator</h1>
          )}
          <button
            onClick={toggleSidebar}
            className={`p-2 hover:bg-gray-700 rounded ${!sidebarOpen ? 'mx-auto' : ''}`}
          >
            <Menu className="w-5 h-5" />
          </button>
        </div>

        {/* Navigation */}
        <nav className="flex-1 p-4 space-y-2">
          {navItems.map((item) => (
            <button
              key={item.id}
              onClick={() => setActiveTab(item.id)}
              className={`w-full flex items-center gap-3 px-3 py-3 rounded-lg transition-colors ${
                activeTab === item.id
                  ? 'bg-blue-600 text-white'
                  : 'text-gray-400 hover:bg-gray-700 hover:text-white'
              }`}
            >
              <item.icon className="w-5 h-5 flex-shrink-0" />
              {sidebarOpen && <span className="truncate">{item.label}</span>}
            </button>
          ))}
        </nav>

        {/* Footer */}
        {sidebarOpen && (
          <div className="p-4 border-t border-gray-700 text-xs text-gray-500">
            <p>AI Agent Orchestrator</p>
            <p>v1.0.0</p>
          </div>
        )}
      </div>
    </aside>
  );
};

export default Sidebar;