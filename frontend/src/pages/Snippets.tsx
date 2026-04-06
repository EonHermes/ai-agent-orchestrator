import React, { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import { getSnippets, deleteSnippet as apiDeleteSnippet } from '../api';
import { Snippet } from '../types';
import SnippetCard from '../components/SnippetCard';
import { FileCode, Plus } from 'lucide-react';

const Snippets: React.FC<{ userId: string }> = ({ userId }) => {
  const [snippets, setSnippets] = useState<Snippet[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    loadSnippets();
  }, [userId]);

  const loadSnippets = async () => {
    try {
      setLoading(true);
      const response = await getSnippets({ user_id: userId });
      if (response.data.success && response.data.data) {
        setSnippets(response.data.data);
      } else {
        setError(response.data.error || 'Failed to load snippets');
      }
    } catch (err: any) {
      setError(err.response?.data?.error || 'Failed to load snippets');
    } finally {
      setLoading(false);
    }
  };

  const handleDelete = async (id: string) => {
    if (!window.confirm('Are you sure you want to delete this snippet?')) {
      return;
    }

    try {
      const response = await apiDeleteSnippet(id);
      if (response.data.success || response.status === 204) {
        setSnippets(snippets.filter(s => s.id !== id));
      } else {
        alert(response.data.error || 'Failed to delete snippet');
      }
    } catch (err: any) {
      alert(err.response?.data?.error || 'Failed to delete snippet');
    }
  };

  if (loading) {
    return (
      <div className="py-6 text-center text-slate-400">
        Loading snippets...
      </div>
    );
  }

  if (error) {
    return (
      <div className="py-6">
        <div className="bg-red-900 border border-red-700 rounded-md p-4 text-red-200">
          Error: {error}
        </div>
      </div>
    );
  }

  return (
    <div className="py-6">
      <div className="flex justify-between items-center mb-6">
        <div>
          <h1 className="text-3xl font-bold text-white">My Snippets</h1>
          <p className="text-slate-400 mt-1">{snippets.length} snippet(s)</p>
        </div>
        <Link
          to="/create"
          className="flex items-center px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded-md text-white"
        >
          <Plus className="mr-2" size={18} />
          New Snippet
        </Link>
      </div>

      {snippets.length === 0 ? (
        <div className="bg-slate-800 rounded-lg border border-slate-700 p-8 text-center">
          <FileCode className="mx-auto mb-4 text-slate-500" size={48} />
          <h3 className="text-xl font-medium text-white mb-2">No snippets yet</h3>
          <p className="text-slate-400 mb-4">
            Create your first code snippet to get started.
          </p>
          <Link
            to="/create"
            className="inline-flex items-center px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded-md text-white"
          >
            <Plus className="mr-2" size={18} />
            Create Snippet
          </Link>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {snippets.map(snippet => (
            <SnippetCard
              key={snippet.id}
              snippet={snippet}
              onDelete={handleDelete}
            />
          ))}
        </div>
      )}
    </div>
  );
};

export default Snippets;