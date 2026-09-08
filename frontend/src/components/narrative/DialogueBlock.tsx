import React from 'react';
import { MessageCircle } from 'lucide-react';

interface DialogueBlockProps {
  speaker: string;
  mood?: string;
  tone?: string;
  content: string;
}

export const DialogueBlock: React.FC<DialogueBlockProps> = ({
  speaker,
  mood,
  tone,
  content,
}) => {
  return (
    <div className="my-3 rounded-xl border border-amber-500/30 bg-amber-950/20 p-4 shadow-sm transition-all hover:border-amber-500/50">
      <div className="mb-2 flex items-center justify-between">
        <div className="flex items-center space-x-2">
          <div className="flex h-6 w-6 items-center justify-center rounded-full bg-amber-500/20 text-amber-400">
            <MessageCircle className="h-3.5 w-3.5" />
          </div>
          <span className="text-xs font-bold uppercase tracking-wider text-amber-300">
            {speaker}
          </span>
        </div>
        <div className="flex items-center space-x-2">
          {mood && (
            <span className="rounded-md bg-amber-500/10 px-2 py-0.5 text-[10px] font-medium text-amber-300 border border-amber-500/20">
              {mood}
            </span>
          )}
          {tone && (
            <span className="rounded-md bg-slate-800 px-2 py-0.5 text-[10px] font-medium text-slate-400">
              {tone}
            </span>
          )}
        </div>
      </div>
      <div className="pl-3 border-l-2 border-amber-500/60 font-serif text-sm italic leading-relaxed text-slate-100">
        {content.startsWith('«') || content.startsWith('"') ? content : `« ${content} »`}
      </div>
    </div>
  );
};
