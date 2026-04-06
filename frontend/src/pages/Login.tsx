import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { login as apiLogin, ApiResponse } from '../api';
import { LoginCredentials } from '../types';
import { Loader2 } from 'lucide-react';

const Login: React.FC<{ onLogin: (user: any, token: string) => void }> = ({ onLogin }) => {
  const navigate = useNavigate();
  const [credentials, setCredentials] = useState<LoginCredentials>({ username: '', password: '' });
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);
    setLoading(true);

    try {
      const response = await apiLogin(credentials.username, credentials.password);
      if (response.data.success && response.data.data) {
        onLogin(response.data.data.user, response.data.data.token);
        navigate('/');
      } else {
        setError(response.data.error || 'Login failed');
      }
    } catch (err: any) {
      setError(err.response?.data?.error || 'Login failed');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="min-h-screen bg-slate-900 flex items-center justify-center p-4">
      <div className="max-w-md w-full space-y-8">
        <div className="text-center">
          <h2 className="mt-6 text-3xl font-bold text-white">Code Snippet Manager</h2>
          <p className="mt-2 text-slate-400">Sign in to your account</p>
        </div>

        <form className="mt-8 space-y-6" onSubmit={handleSubmit}>
          <div className="space-y-4">
            <div>
              <label htmlFor="username" className="block text-sm font-medium text-slate-300 mb-1">
                Username
              </label>
              <input
                id="username"
                type="text"
                required
                value={credentials.username}
                onChange={(e) => setCredentials({ ...credentials, username: e.target.value })}
                className="w-full px-3 py-2 bg-slate-800 border border-slate-700 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>
            <div>
              <label htmlFor="password" className="block text-sm font-medium text-slate-300 mb-1">
                Password
              </label>
              <input
                id="password"
                type="password"
                required
                value={credentials.password}
                onChange={(e) => setCredentials({ ...credentials, password: e.target.value })}
                className="w-full px-3 py-2 bg-slate-800 border border-slate-700 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>
          </div>

          {error && (
            <div className="p-3 bg-red-900 border border-red-700 rounded-md text-sm text-red-200">
              {error}
            </div>
          )}

          <button
            type="submit"
            disabled={loading}
            className="w-full flex justify-center items-center px-4 py-2 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {loading ? <Loader2 className="animate-spin mr-2" size={18} /> : null}
            {loading ? 'Signing in...' : 'Sign in'}
          </button>

          <div className="text-center text-sm text-slate-400">
            Don't have an account?{' '}
            <button
              type="button"
              onClick={() => {/* Implement registration flow */}}
              className="text-blue-400 hover:text-blue-300"
            >
              Register
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};

export default Login;