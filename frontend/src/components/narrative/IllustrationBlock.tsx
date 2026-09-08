import React from 'react';
import { Image as ImageIcon } from 'lucide-react';

interface IllustrationBlockProps {
  prompt: string;
}

export const IllustrationBlock: React.FC<IllustrationBlockProps> = ({ prompt }) => {
  return (
    <div className="my-3 rounded-xl border border-slate-700/50 bg-slate-900/60 p-4 text-center">
      <div className="flex flex-col items-center justify-center py-4 border border-dashed border-slate-800 rounded-lg">
        <ImageIcon className="h-6 w-6 text-slate-500 mb-2" />
        <span className="text-[11px] font-semibold text-slate-400 uppercase tracking-wider">
          Illustration suggérée
        </span>
        <p className="mt-1 text-xs italic text-slate-500 max-w-md px-4 font-serif">
          « {prompt} »
        </p>
      </div>
    </div>
  );
};
