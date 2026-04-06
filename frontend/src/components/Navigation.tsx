import React from 'react';
import { Link, useLocation } from 'react-router-dom';
import { Home, Search, FileCode, PlusCircle, LogOut } from 'lucide-react';

const Navigation: React.FC = () => {
  const location = useLocation();

  const isActive = (path: string) => location.pathname === path;

  const navItems = [
    { path: '/', icon: Home, label: 'Dashboard' },
    { path: '/snippets', icon: FileCode, label: 'My Snippets' },
    { path: '/search', icon: Search, label: 'Search' },
    { path: '/create', icon: PlusCircle, label: 'Create' },
  ];

  const handleLogout = () => {
    localStorage.removeItem('token');
    localStorage.removeItem('user');
    window.location.href = '/login';
  };

  return (
    <nav className="bg-slate-800 border-b border-slate-700 px-4 py-3">
      <div className="max-w-7xl mx-auto flex items-center justify-between">
        <div className="flex items-center space-x-4">
          <Link to="/" className="text-xl font-bold text-white flex items-center">
            <FileCode className="mr-2" size={24} />
            Snippets
          </Link>
          
          <div className="flex space-x-1">
            {navItems.map(item => (
              <Link
                key={item.path}
                to={item.path}
                className={`flex items-center px-3 py-2 rounded-md text-sm font-medium transition-colors ${
                  isActive(item.path)
                    ? 'bg-slate-900 text-white'
                    : 'text-slate-300 hover:bg-slate-700 hover:text-white'
                }`}
              >
                <item.icon className="mr-2" size={18} />
                {item.label}
              </Link>
            ))}
          </div>
        </div>

        <button
          onClick={handleLogout}
          className="flex items-center px-3 py-2 rounded-md text-sm font-medium text-slate-300 hover:bg-slate-700 hover:text-white transition-colors"
        >
          <LogOut className="mr-2" size={18} />
          Logout
        </button>
      </div>
    </nav>
  );
};

export default Navigation;