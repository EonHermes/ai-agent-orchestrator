import React from 'react';
import { useStore } from './store';
import Dashboard from './pages/Dashboard';
import Agents from './pages/Agents';
import Tasks from './pages/Tasks';
import Executions from './pages/Executions';
import Sidebar from './components/Sidebar';

function App() {
  const { activeTab, sidebarOpen } = useStore();

  const renderContent = () => {
    switch (activeTab) {
      case 'dashboard':
        return <Dashboard />;
      case 'agents':
        return <Agents />;
      case 'tasks':
        return <Tasks />;
      case 'executions':
        return <Executions />;
      default:
        return <Dashboard />;
    }
  };

  return (
    <div className="flex h-screen bg-gray-900 text-gray-100">
      <Sidebar />
      <main className={`flex-1 overflow-auto transition-all duration-300 ${sidebarOpen ? 'ml-64' : 'ml-16'}`}>
        <div className="p-6">{renderContent()}</div>
      </main>
    </div>
  );
}

export default App;