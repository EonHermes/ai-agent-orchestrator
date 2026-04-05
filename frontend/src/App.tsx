import React from 'react';
import { BrowserRouter as Router, Routes, Route, Navigate } from 'react-router-dom';
import Layout from './components/Layout';
import Dashboard from './pages/Dashboard';
import Agents from './pages/Agents';
import Tasks from './pages/Tasks';
import Analytics from './pages/Analytics';
import { useStore } from './store/useStore';

function App() {
  const { fetchStats, fetchAgents, fetchHealth } = useStore();

  // Initial data fetch
  React.useEffect(() => {
    fetchStats();
    fetchAgents();
    fetchHealth();
  }, [fetchStats, fetchAgents, fetchHealth]);

  return (
    <Router>
      <Layout>
        <Routes>
          <Route path="/" element={<Navigate to="/dashboard" replace />} />
          <Route path="/dashboard" element={<Dashboard />} />
          <Route path="/agents" element={<Agents />} />
          <Route path="/tasks" element={<Tasks />} />
          <Route path="/analytics" element={<Analytics />} />
        </Routes>
      </Layout>
    </Router>
  );
}

export default App;
