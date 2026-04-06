import React, { useEffect, useState } from 'react';
import { User } from '../types';

const Dashboard: React.FC<{ user: User }> = ({ user }) => {
  return (
    <div className="py-6">
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-white mb-2">Welcome, {user.username}!</h1>
        <p className="text-slate-400">
          Manage your code snippets with full-text search and beautiful syntax highlighting.
        </p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
        <div className="bg-slate-800 rounded-lg border border-slate-700 p-6">
          <h3 className="text-xl font-semibold text-white mb-2">Quick Actions</h3>
          <div className="space-y-3">
            <a
              href="/create"
              className="block w-full px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded text-center text-white"
            >
              Create New Snippet
            </a>
            <a
              href="/snippets"
              className="block w-full px-4 py-2 bg-slate-700 hover:bg-slate-600 rounded text-center text-slate-300"
            >
              Browse All Snippets
            </a>
            <a
              href="/search"
              className="block w-full px-4 py-2 bg-slate-700 hover:bg-slate-600 rounded text-center text-slate-300"
            >
              Search Snippets
            </a>
          </div>
        </div>

        <div className="bg-slate-800 rounded-lg border border-slate-700 p-6">
          <h3 className="text-xl font-semibold text-white mb-2">Features</h3>
          <ul className="space-y-2 text-slate-400">
            <li>✓ Full-text search with ranking</li>
            <li>✓ Syntax highlighting (Monaco Editor)</li>
            <li>✓ Tags and categorization</li>
            <li>✓ Real-time updates</li>
            <li>✓ Import/Export support</li>
            <li>✓ Rate limiting and security</li>
          </ul>
        </div>

        <div className="bg-slate-800 rounded-lg border border-slate-700 p-6">
          <h3 className="text-xl font-semibold text-white mb-2">Tech Stack</h3>
          <div className="space-y-2 text-slate-400 text-sm">
            <div>
              <span className="text-blue-400 font-semibold">Backend:</span> Rust + Actix-web + SQLite + Tantivy
            </div>
            <div>
              <span className="text-blue-400 font-semibold">Frontend:</span> React + TypeScript + Vite + Tailwind
            </div>
            <div>
              <span className="text-blue-400 font-semibold">Editor:</span> Monaco (VS Code core)
            </div>
            <div>
              <span className="text-blue-400 font-semibold">Deployment:</span> Docker + Nginx + Systemd
            </div>
            <div>
              <span className="text-green-400 font-semibold">Tests:</span> Unit + Integration + E2E
            </div>
          </div>
        </div>
      </div>

      <div className="bg-slate-800 rounded-lg border border-slate-700 p-6">
        <h3 className="text-xl font-semibold text-white mb-4">Getting Started</h3>
        <div className="prose prose-invert max-w-none">
          <p className="text-slate-400 mb-4">
            This is a production-ready code snippet manager with enterprise-grade features.
          </p>
          <ol className="list-decimal list-inside space-y-2 text-slate-300">
            <li>Create a new snippet from the <strong>Create</strong> tab</li>
            <li>Use tags to organize your snippets</li>
            <li>Full-text search finds content in titles, code, and tags</li>
            <li>Edit or delete your snippets anytime</li>
            <li>All snippets are private to your account</li>
          </ol>
        </div>
      </div>
    </div>
  );
};

export default Dashboard;