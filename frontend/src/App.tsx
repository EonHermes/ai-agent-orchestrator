import React, { useState, useEffect } from 'react';
import { Routes, Route, Navigate, Link } from 'react-router-dom';
import { createUser, login as apiLogin, getSnippets, createSnippet as apiCreateSnippet, deleteSnippet } from './api';
import { User, Snippet } from './types';
import Navigation from './components/Navigation';
import ProtectedRoute from './components/ProtectedRoute';
import Login from './pages/Login';
import Dashboard from './pages/Dashboard';
import Snippets from './pages/Snippets';
import CreateSnippet from './pages/CreateSnippet';
import Search from './pages/Search';

const App: React.FC = () => {
  const [user, setUser] = useState<User | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const storedUser = localStorage.getItem('user');
    if (storedUser) {
      try {
        setUser(JSON.parse(storedUser));
      } catch {
        localStorage.removeItem('user');
      }
    }
    setLoading(false);
  }, []);

  const handleLogin = (userData: User, token: string) => {
    setUser(userData);
    localStorage.setItem('token', token);
    localStorage.setItem('user', JSON.stringify(userData));
  };

  const handleLogout = () => {
    setUser(null);
    localStorage.removeItem('token');
    localStorage.removeItem('user');
  };

  if (loading) {
    return (
      <div className="min-h-screen bg-slate-900 flex items-center justify-center">
        <div className="text-white text-xl">Loading...</div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-slate-900">
      {user && <Navigation />}
      <main className="max-w-7xl mx-auto">
        <Routes>
          <Route path="/login" element={!user ? <Login onLogin={handleLogin} /> : <Navigate to="/" replace />} />
          
          <Route path="/" element={
            <ProtectedRoute>
              <Dashboard user={user} />
            </ProtectedRoute>
          } />
          
          <Route path="/snippets" element={
            <ProtectedRoute>
              <Snippets userId={user!.id} />
            </ProtectedRoute>
          } />
          
          <Route path="/snippets/:id" element={
            <ProtectedRoute>
              <SnippetDetail />
            </ProtectedRoute>
          } />
          
          <Route path="/snippets/:id/edit" element={
            <ProtectedRoute>
              <CreateSnippet />
            </ProtectedRoute>
          } />
          
          <Route path="/create" element={
            <ProtectedRoute>
              <CreateSnippet />
            </ProtectedRoute>
          } />
          
          <Route path="/search" element={
            <ProtectedRoute>
              <Search />
            </ProtectedRoute>
          } />
        </Routes>
      </main>
    </div>
  );
};

export default App;