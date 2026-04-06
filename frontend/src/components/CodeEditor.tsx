import React, { useState } from 'react';
import Editor from '@monaco-editor/react';
import { X } from 'lucide-react';

interface CodeEditorProps {
  value: string;
  onChange: (value: string) => void;
  language?: string;
  readOnly?: boolean;
  onClose?: () => void;
}

const CodeEditor: React.FC<CodeEditorProps> = ({
  value,
  onChange,
  language = 'plaintext',
  readOnly = false,
  onClose,
}) => {
  const [isFullscreen, setIsFullscreen] = useState(false);

  const editorOptions = {
    minimap: { enabled: true },
    fontSize: 14,
    lineNumbers: 'on' as const,
    scrollBeyondLastLine: false,
    automaticLayout: true,
    tabSize: 2,
    wordWrap: 'on' as const,
    readOnly,
    padding: { top: 16, bottom: 16 },
  };

  return (
    <div className={`h-full ${isFullscreen ? 'fixed inset-0 z-50 bg-slate-900' : ''}`}>
      <div className={`flex justify-between items-center px-4 py-2 bg-slate-800 ${isFullscreen ? 'absolute top-0 w-full z-10' : ''}`}>
        <span className="text-sm text-slate-400 font-mono uppercase">{language}</span>
        <div className="flex items-center space-x-2">
          {onClose && (
            <button
              onClick={onClose}
              className="p-1 hover:bg-slate-700 rounded text-slate-400 hover:text-white"
            >
              <X size={20} />
            </button>
          )}
          <button
            onClick={() => setIsFullscreen(!isFullscreen)}
            className="px-3 py-1 text-xs bg-slate-700 hover:bg-slate-600 rounded text-slate-300"
          >
            {isFullscreen ? 'Exit Fullscreen' : 'Fullscreen'}
          </button>
        </div>
      </div>
      <div className={`${isFullscreen ? 'pt-10 h-full' : 'h-full mt-0'}`}>
        <Editor
          height="100%"
          language={language}
          value={value}
          onChange={onChange}
          theme="vs-dark"
          options={editorOptions}
        />
      </div>
    </div>
  );
};

export default CodeEditor;