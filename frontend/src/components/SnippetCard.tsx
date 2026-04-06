import React from 'react';
import { Link } from 'react-router-dom';
import { Snippet } from '../types';
import { FileCode, Calendar, Tag } from 'lucide-react';

interface SnippetCardProps {
  snippet: Snippet;
  onDelete: (id: string) => void;
}

const SnippetCard: React.FC<SnippetCardProps> = ({ snippet, onDelete }) => {
  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
    });
  };

  return (
    <div className="bg-slate-800 rounded-lg border border-slate-700 p-4 hover:border-blue-500 transition-colors">
      <div className="flex justify-between items-start mb-3">
        <h3 className="text-lg font-semibold text-white truncate">{snippet.title}</h3>
        <button
          onClick={() => onDelete(snippet.id)}
          className="text-slate-400 hover:text-red-400 text-sm"
        >
          Delete
        </button>
      </div>

      <div className="flex items-center space-x-4 text-sm text-slate-400 mb-3">
        <span className="flex items-center">
          <FileCode className="mr-1" size={16} />
          {snippet.language}
        </span>
        <span className="flex items-center">
          <Calendar className="mr-1" size={16} />
          {formatDate(snippet.created_at)}
        </span>
      </div>

      {snippet.tags && (
        <div className="flex items-center flex-wrap gap-2 mb-3">
          <Tag className="text-slate-500" size={14} />
          {snippet.tags.split(',').map((tag, index) => (
            <span
              key={index}
              className="px-2 py-1 bg-slate-700 rounded text-xs text-slate-300"
            >
              {tag.trim()}
            </span>
          ))}
        </div>
      )}

      {snippet.description && (
        <p className="text-slate-400 text-sm mb-3 line-clamp-2">{snippet.description}</p>
      )}

      <div className="bg-slate-900 rounded p-3 mt-3">
        <pre className="text-xs overflow-x-auto">
          <code className="text-slate-300">
            {snippet.code.substring(0, 200)}
            {snippet.code.length > 200 && '...'}
          </code>
        </pre>
      </div>

      <div className="mt-3">
        <Link
          to={`/snippets/${snippet.id}`}
          className="text-blue-400 hover:text-blue-300 text-sm"
        >
          View Details →
        </Link>
      </div>
    </div>
  );
};

export default SnippetCard;