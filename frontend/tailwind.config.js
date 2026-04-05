/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        eon: {
          primary: '#3B82F6',
          secondary: '#8B5CF6',
          background: '#0a0a0a',
          surface: '#1a1a1a',
          surfaceLight: '#262626',
          text: '#f3f4f6',
          textSecondary: '#9ca3af',
          border: '#374151',
          success: '#10b981',
          error: '#ef4444',
          warning: '#f59e0b',
        }
      },
    },
  },
  plugins: [],
}
