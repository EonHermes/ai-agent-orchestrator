import React, { useState, useEffect } from 'react';
import { searchSnippets as apiSearchSnippets } from '../api';
import { Snippet } from '../types';
import SnippetCard from '../components/SnippetCard';
import { Search, Loader2 } from 'lucide-react';

const SearchPage: React.FC = () => {
  const [query, setQuery] = useState('');
  const [snippets, setSnippets] = useState<Snippet[]>([]);
  const [loading, setLoading] = useState(false);
  const [searched, setSearched] = useState(false);

  const handleSearch = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!query.trim()) return;

    setLoading(true);
    setSearched(true);

    try {
      const response = await apiSearchSnippets(query.trim());
      if (response.data.success && response.data.data) {
        setSnippets(response.data.data);
      } else {
        setSnippets([]);
      }
    } catch (err: any) {
      console.error('Search failed:', err);
      setSnippets([]);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="py-6">
      <h1 className="text-3xl font-bold text-white mb-6">Search Snippets</h1>

      <form onSubmit={handleSearch} className="mb-6">
        <div className="flex space-x-2">
          <input
            type="text"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            placeholder="Search by title, code, or tags..."
            className="flex-1 px-4 py-2 bg-slate-800 border border-slate-700 rounded-l-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
          <button
            type="submit"
            disabled={loading || !query.trim()}
            className="px-6 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-slate-700 disabled:cursor-not-allowed rounded-r-md text-white flex items-center"
          >
            {loading ? <Loader2 className="animate-spin mr-2" size={18} /> : <Search className="mr-2" size={18} />}
            {loading ? 'Searching...' : 'Search'}
          </button>
        </div>
      </form>

      {loading ? (
        <div className="text-center py-12 text-slate-400">
          <Loader2 className="animate-spin mx-auto mb-4" size={32} />
          Searching full-text index...
        </div>
      ) : searched && snippets.length === 0 ? (
        <div className="bg-slate-800 rounded-lg border border-slate-700 p-8 text-center">
          <Search className="mx-auto mb-4 text-slate-500" size={48} />
          <h3 className="text-xl font-medium text-white mb-2">No results found</h3>
          <p className="text-slate-400">
            Try searching for something else, like a function name, language, or tag.
          </p>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {snippets.map(snippet => (
            <SnippetCard
              key={snippet.id}
              snippet={snippet}
              onDelete={() => {}}
            />
          ))}
        </div>
      )}
    </div>
  );
};

export default SearchPage;