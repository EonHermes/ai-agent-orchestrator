import React, { useState, useEffect } from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import { apiCreateSnippet, getSnippetById, apiUpdateSnippet } from '../api';
import { Snippet, CreateSnippet } from '../types';
import CodeEditor from '../components/CodeEditor';
import { Loader2, Save, Edit } from 'lucide-react';

const LANGUAGES = [
  { value: 'rust', label: 'Rust' },
  { value: 'javascript', label: 'JavaScript' },
  { value: 'typescript', label: 'TypeScript' },
  { value: 'python', label: 'Python' },
  { value: 'go', label: 'Go' },
  { value: 'java', label: 'Java' },
  { value: 'cpp', label: 'C++' },
  { value: 'csharp', label: 'C#' },
  { value: 'sql', label: 'SQL' },
  { value: 'html', label: 'HTML' },
  { value: 'css', label: 'CSS' },
  { value: 'json', label: 'JSON' },
  { value: 'yaml', label: 'YAML' },
  { value: 'bash', label: 'Bash' },
  { value: 'plaintext', label: 'Plain Text' },
];

const CreateSnippetPage: React.FC = () => {
  const navigate = useNavigate();
  const { id } = useParams<{ id: string }>();
  const isEditing = Boolean(id);

  const [formData, setFormData] = useState<CreateSnippet>({
    title: '',
    code: '',
    language: 'rust',
    description: '',
    tags: '',
  });
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [initialLoading, setInitialLoading] = useState(isEditing);

  useEffect(() => {
    if (isEditing && id) {
      loadSnippet(id);
    }
  }, [id, isEditing]);

  const loadSnippet = async (snippetId: string) => {
    try {
      const response = await getSnippetById(snippetId);
      if (response.data.success && response.data.data) {
        const snippet = response.data.data;
        setFormData({
          title: snippet.title,
          code: snippet.code,
          language: snippet.language,
          description: snippet.description || '',
          tags: snippet.tags || '',
        });
      } else {
        setError(response.data.error || 'Snippet not found');
      }
    } catch (err: any) {
      setError(err.response?.data?.error || 'Failed to load snippet');
    } finally {
      setInitialLoading(false);
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);
    setLoading(true);

    try {
      if (isEditing && id) {
        const response = await apiUpdateSnippet(id, formData);
        if (response.data.success && response.data.data) {
          navigate(`/snippets/${id}`);
        } else {
          setError(response.data.error || 'Failed to update snippet');
        }
      } else {
        const response = await apiCreateSnippet(formData);
        if (response.data.success && response.data.data) {
          navigate('/snippets');
        } else {
          setError(response.data.error || 'Failed to create snippet');
        }
      }
    } catch (err: any) {
      setError(err.response?.data?.error || 'Failed to save snippet');
    } finally {
      setLoading(false);
    }
  };

  const handleChange = (field: keyof CreateSnippet, value: string) => {
    setFormData({ ...formData, [field]: value });
  };

  return (
    <div className="py-6">
      <h1 className="text-3xl font-bold text-white mb-6">Create New Snippet</h1>

      <form onSubmit={handleSubmit} className="space-y-6">
        <div className="bg-slate-800 rounded-lg border border-slate-700 p-6 space-y-4">
          <div>
            <label htmlFor="title" className="block text-sm font-medium text-slate-300 mb-1">
              Title *
            </label>
            <input
              id="title"
              type="text"
              required
              value={formData.title}
              onChange={(e) => handleChange('title', e.target.value)}
              className="w-full px-3 py-2 bg-slate-900 border border-slate-700 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="Enter a descriptive title..."
            />
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label htmlFor="language" className="block text-sm font-medium text-slate-300 mb-1">
                Language *
              </label>
              <select
                id="language"
                value={formData.language}
                onChange={(e) => handleChange('language', e.target.value)}
                className="w-full px-3 py-2 bg-slate-900 border border-slate-700 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                {LANGUAGES.map(lang => (
                  <option key={lang.value} value={lang.value}>{lang.label}</option>
                ))}
              </select>
            </div>

            <div>
              <label htmlFor="tags" className="block text-sm font-medium text-slate-300 mb-1">
                Tags
              </label>
              <input
                id="tags"
                type="text"
                value={formData.tags}
                onChange={(e) => handleChange('tags', e.target.value)}
                className="w-full px-3 py-2 bg-slate-900 border border-slate-700 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                placeholder="e.g., rust, performance, utility (comma separated)"
              />
            </div>
          </div>

          <div>
            <label htmlFor="description" className="block text-sm font-medium text-slate-300 mb-1">
              Description
            </label>
            <textarea
              id="description"
              value={formData.description}
              onChange={(e) => handleChange('description', e.target.value)}
              className="w-full px-3 py-2 bg-slate-900 border border-slate-700 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
              rows={3}
              placeholder="Describe what this snippet does..."
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-slate-300 mb-1">
              Code *
            </label>
            <div className="h-96 border border-slate-700 rounded-md overflow-hidden">
              <CodeEditor
                value={formData.code}
                onChange={(code) => handleChange('code', code)}
                language={formData.language}
              />
            </div>
          </div>
        </div>

        {initialLoading ? (
        <div className="flex justify-center py-12">
          <Loader2 className="animate-spin text-blue-500" size={32} />
        </div>
      ) : (
        <>
          {error && (
            <div className="p-3 bg-red-900 border border-red-700 rounded-md text-sm text-red-200">
              {error}
            </div>
          )}

          <div className="flex items-center space-x-4">
            <button
              type="submit"
              disabled={loading || !formData.title.trim() || !formData.code.trim()}
              className="flex items-center px-6 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-slate-700 disabled:cursor-not-allowed rounded-md text-white font-medium"
            >
              {loading ? <Loader2 className="animate-spin mr-2" size={18} /> : (isEditing ? <Edit className="mr-2" size={18} /> : <Save className="mr-2" size={18} />)}
              {loading ? 'Saving...' : (isEditing ? 'Update Snippet' : 'Create Snippet')}
            </button>
            <button
              type="button"
              onClick={() => navigate(-1)}
              className="px-6 py-2 bg-slate-700 hover:bg-slate-600 rounded-md text-slate-300"
            >
              Cancel
            </button>
          </div>
        </>
      )}
      </form>
    </div>
  );
};

export default CreateSnippetPage;