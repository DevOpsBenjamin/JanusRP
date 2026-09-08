import React, { useState } from 'react';
import { ChevronDown, ChevronUp, FileText } from 'lucide-react';

interface DocumentBlockProps {
  title: string;
  content: string;
}

export const DocumentBlock: React.FC<DocumentBlockProps> = ({
  title,
  content,
}) => {
  const [isExpanded, setIsExpanded] = useState(true);

  return (
    <div className="my-3 rounded-xl border border-yellow-700/40 bg-gradient-to-b from-[#1c1811] to-[#14120e] p-4 text-amber-100 shadow-md">
      <div
        className="flex items-center justify-between cursor-pointer border-b border-yellow-800/40 pb-2 mb-2"
        onClick={() => setIsExpanded(!isExpanded)}
      >
        <div className="flex items-center space-x-2">
          <FileText className="h-4 w-4 text-yellow-500" />
          <span className="font-serif font-bold text-xs uppercase tracking-wider text-yellow-400">
            {title}
          </span>
        </div>
        <button
          type="button"
          className="text-yellow-600 hover:text-yellow-400 transition-colors"
        >
          {isExpanded ? <ChevronUp className="h-4 w-4" /> : <ChevronDown className="h-4 w-4" />}
        </button>
      </div>

      {isExpanded && (
        <div className="rounded-lg bg-black/30 p-3 font-serif italic text-xs leading-relaxed text-amber-200/90 whitespace-pre-wrap border border-yellow-900/30">
          {content}
        </div>
      )}
    </div>
  );
};
