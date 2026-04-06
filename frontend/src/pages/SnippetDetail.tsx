import React, { useEffect, useState } from 'react';
import { useParams, Link, useNavigate } from 'react-router-dom';
import { getSnippetById, deleteSnippet as apiDeleteSnippet } from '../api';
import { Snippet } from '../types';
import CodeEditor from '../components/CodeEditor';
import { ArrowLeft, Calendar, Tag, FileCode, Loader2, Edit, Trash2 } from 'lucide-react';

const SnippetDetail: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [snippet, setSnippet] = useState<Snippet | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [deleteConfirm, setDeleteConfirm] = useState(false);

  useEffect(() => {
    if (id) {
      loadSnippet();
    }
  }, [id]);

  const loadSnippet = async () => {
    if (!id) return;
    
    try {
      setLoading(true);
      const response = await getSnippetById(id);
      if (response.data.success && response.data.data) {
        setSnippet(response.data.data);
      } else {
        setError(response.data.error || 'Snippet not found');
      }
    } catch (err: any) {
      setError(err.response?.data?.error || 'Failed to load snippet');
    } finally {
      setLoading(false);
    }
  };

  const handleDelete = async () => {
    if (!id) return;
    
    try {
      const response = await apiDeleteSnippet(id);
      if (response.data.success || response.status === 204) {
        navigate('/snippets');
      } else {
        alert(response.data.error || 'Failed to delete snippet');
      }
    } catch (err: any) {
      alert(err.response?.data?.error || 'Failed to delete snippet');
    }
  };

  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    });
  };

  if (loading) {
    return (
      <div className="py-6 flex justify-center items-center min-h-[400px]">
        <Loader2 className="animate-spin text-blue-500" size={32} />
      </div>
    );
  }

  if (error || !snippet) {
    return (
      <div className="py-6">
        <div className="bg-red-900 border border-red-700 rounded-md p-4 text-red-200">
          {error || 'Snippet not found'}
        </div>
        <Link to="/snippets" className="mt-4 inline-flex items-center text-blue-400 hover:text-blue-300">
          <ArrowLeft className="mr-2" size={18} />
          Back to snippets
        </Link>
      </div>
    );
  }

  return (
    <div className="py-6">
      <div className="mb-6">
        <Link to="/snippets" className="inline-flex items-center text-slate-400 hover:text-white mb-4">
          <ArrowLeft className="mr-2" size={18} />
          Back to snippets
        </Link>

        <div className="flex justify-between items-start">
          <div>
            <h1 className="text-3xl font-bold text-white mb-2">{snippet.title}</h1>
            <div className="flex items-center space-x-4 text-slate-400">
              <span className="flex items-center">
                <FileCode className="mr-1" size={18} />
                {snippet.language}
              </span>
              <span className="flex items-center">
                <Calendar className="mr-1" size={18} />
                Created: {formatDate(snippet.created_at)}
              </span>
              {snippet.updated_at !== snippet.created_at && (
                <span className="flex items-center">
                  Updated: {formatDate(snippet.updated_at)}
                </span>
              )}
            </div>
            {snippet.tags && (
              <div className="flex items-center flex-wrap gap-2 mt-3">
                <Tag className="text-slate-500" size={16} />
                {snippet.tags.split(',').map((tag, index) => (
                  <span
                    key={index}
                    className="px-3 py-1 bg-slate-700 rounded-full text-sm text-slate-300"
                  >
                    {tag.trim()}
                  </span>
                ))}
              </div>
            )}
          </div>

          <div className="flex items-center space-x-2">
            <button
              onClick={() => navigate(`/snippets/${snippet.id}/edit`)}
              className="flex items-center px-4 py-2 bg-slate-700 hover:bg-slate-600 rounded-md text-white"
            >
              <Edit className="mr-2" size={18} />
              Edit
            </button>
            <button
              onClick={() => setDeleteConfirm(true)}
              className="flex items-center px-4 py-2 bg-red-700 hover:bg-red-600 rounded-md text-white"
            >
              <Trash2 className="mr-2" size={18} />
              Delete
            </button>
          </div>
        </div>

        {snippet.description && (
          <div className="mt-4 p-4 bg-slate-800 border border-slate-700 rounded-md">
            <h3 className="text-sm font-semibold text-slate-400 mb-2">Description</h3>
            <p className="text-slate-300">{snippet.description}</p>
          </div>
        )}
      </div>

      <div className="bg-slate-800 rounded-lg border border-slate-700 overflow-hidden">
        <div className="bg-slate-900 px-4 py-2 border-b border-slate-700">
          <span className="text-sm text-slate-400 font-mono uppercase">{snippet.language}</span>
        </div>
        <CodeEditor
          value={snippet.code}
          onChange={() => {}}
          language={snippet.language}
          readOnly
        />
      </div>

      {deleteConfirm && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-slate-800 border border-slate-700 rounded-lg p-6 max-w-md w-full mx-4">
            <h3 className="text-xl font-semibold text-white mb-2">Delete Snippet?</h3>
            <p className="text-slate-400 mb-6">
              This action cannot be undone. Are you sure you want to delete "{snippet.title}"?
            </p>
            <div className="flex justify-end space-x-3">
              <button
                onClick={() => setDeleteConfirm(false)}
                className="px-4 py-2 bg-slate-700 hover:bg-slate-600 rounded-md text-white"
              >
                Cancel
              </button>
              <button
                onClick={handleDelete}
                className="px-4 py-2 bg-red-700 hover:bg-red-600 rounded-md text-white"
              >
                Delete
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default SnippetDetail;