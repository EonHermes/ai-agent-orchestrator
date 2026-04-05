import React from 'react';
import { NavLink } from 'react-router-dom';
import { LayoutDashboard, Bot, ListTodo, BarChart3 } from 'lucide-react';
import { useStore } from '../store/useStore';

interface LayoutProps {
  children: React.ReactNode;
}

const Layout: React.FC<LayoutProps> = ({ children }) => {
  const { health, systemStatus } = useStore();

  return (
    <div className="flex h-screen bg-eon-background">
      {/* Sidebar */}
      <nav className="w-64 bg-eon-surface border-r border-eon-border flex flex-col">
        <div className="p-6 border-b border-eon-border">
          <h1 className="text-xl font-bold text-eon-primary">EON Orchestrator</h1>
          <p className="text-sm text-eon-textSecondary mt-1">AI Agent Framework</p>
        </div>

        <div className="flex-1 p-4 space-y-2">
          <NavLink
            to="/dashboard"
            className={({ isActive }) =>
              `flex items-center gap-3 px-4 py-3 rounded-lg transition-colors ${
                isActive
                  ? 'bg-eon-primary text-white'
                  : 'text-eon-textSecondary hover:bg-eon-surfaceLight hover:text-eon-text'
              }`
            }
          >
            <LayoutDashboard size={20} />
            <span>Dashboard</span>
          </NavLink>

          <NavLink
            to="/agents"
            className={({ isActive }) =>
              `flex items-center gap-3 px-4 py-3 rounded-lg transition-colors ${
                isActive
                  ? 'bg-eon-primary text-white'
                  : 'text-eon-textSecondary hover:bg-eon-surfaceLight hover:text-eon-text'
              }`
            }
          >
            <Bot size={20} />
            <span>Agents</span>
          </NavLink>

          <NavLink
            to="/tasks"
            className={({ isActive }) =>
              `flex items-center gap-3 px-4 py-3 rounded-lg transition-colors ${
                isActive
                  ? 'bg-eon-primary text-white'
                  : 'text-eon-textSecondary hover:bg-eon-surfaceLight hover:text-eon-text'
              }`
            }
          >
            <ListTodo size={20} />
            <span>Tasks</span>
          </NavLink>

          <NavLink
            to="/analytics"
            className={({ isActive }) =>
              `flex items-center gap-3 px-4 py-3 rounded-lg transition-colors ${
                isActive
                  ? 'bg-eon-primary text-white'
                  : 'text-eon-textSecondary hover:bg-eon-surfaceLight hover:text-eon-text'
              }`
            }
          >
            <BarChart3 size={20} />
            <span>Analytics</span>
          </NavLink>
        </div>

        <div className="p-4 border-t border-eon-border">
          <div className="flex items-center gap-2">
            <div
              className={`w-2 h-2 rounded-full ${
                health?.status === 'healthy' ? 'bg-eon-success' : 'bg-eon-error'
              }`}
            />
            <span className="text-sm text-eon-textSecondary">
              {systemStatus ? `${systemStatus.agent_count} agents` : 'Loading...'}
            </span>
          </div>
        </div>
      </nav>

      {/* Main Content */}
      <main className="flex-1 overflow-auto">
        <div className="p-8">{children}</div>
      </main>
    </div>
  );
};

export default Layout;
