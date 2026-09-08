import React, { useState } from 'react';
import { Eye, EyeOff, Sparkles } from 'lucide-react';

interface ThoughtBlockProps {
  speaker?: string;
  visibility?: 'hidden' | 'visible';
  content: string;
}

export const ThoughtBlock: React.FC<ThoughtBlockProps> = ({
  speaker,
  visibility = 'hidden',
  content,
}) => {
  const [isRevealed, setIsRevealed] = useState(visibility === 'visible');

  return (
    <div className="my-2.5 rounded-xl border border-purple-500/30 bg-purple-950/15 p-3.5 transition-all">
      <div className="flex items-center justify-between mb-1.5">
        <div className="flex items-center space-x-2">
          <Sparkles className="h-3.5 w-3.5 text-purple-400" />
          <span className="text-[11px] font-semibold text-purple-300 uppercase tracking-wider">
            {speaker ? `Pensée intime (${speaker})` : 'Pensée intime'}
          </span>
        </div>
        <button
          type="button"
          onClick={() => setIsRevealed(!isRevealed)}
          className="flex items-center space-x-1 text-[10px] text-purple-300/80 hover:text-purple-200 bg-purple-900/30 hover:bg-purple-900/50 px-2 py-0.5 rounded-md transition-colors"
        >
          {isRevealed ? (
            <>
              <EyeOff className="h-3 w-3 mr-1" />
              <span>Masquer</span>
            </>
          ) : (
            <>
              <Eye className="h-3 w-3 mr-1" />
              <span>Révéler</span>
            </>
          )}
        </button>
      </div>

      {isRevealed ? (
        <p className="font-serif italic text-xs leading-relaxed text-purple-200/90 pl-3 border-l border-purple-400/40">
          {content}
        </p>
      ) : (
        <p className="text-xs text-purple-400/50 italic tracking-wide select-none">
          [Pensée cachée — cliquez sur Révéler pour déchiffrer]
        </p>
      )}
    </div>
  );
};
