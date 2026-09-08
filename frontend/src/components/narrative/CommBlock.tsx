import React from 'react';
import { Radio, Smartphone, Terminal } from 'lucide-react';

interface CommBlockProps {
  commType?: string;
  from?: string;
  to?: string;
  app?: string;
  time?: string;
  content: string;
}

export const CommBlock: React.FC<CommBlockProps> = ({
  commType = 'message',
  from,
  to,
  app,
  time,
  content,
}) => {
  const getIcon = () => {
    switch (commType.toLowerCase()) {
      case 'radio':
        return <Radio className="h-3.5 w-3.5 text-cyan-400" />;
      case 'terminal':
        return <Terminal className="h-3.5 w-3.5 text-emerald-400" />;
      default:
        return <Smartphone className="h-3.5 w-3.5 text-blue-400" />;
    }
  };

  return (
    <div className="my-3 rounded-xl border border-cyan-500/30 bg-slate-900/90 p-3.5 shadow-md">
      <div className="flex items-center justify-between border-b border-slate-800 pb-2 mb-2">
        <div className="flex items-center space-x-2">
          {getIcon()}
          <span className="text-[11px] font-bold uppercase tracking-wider text-cyan-300">
            {app ? `${app}` : `Transmission (${commType})`}
          </span>
        </div>
        {time && <span className="font-mono text-[10px] text-slate-500">{time}</span>}
      </div>

      {(from || to) && (
        <div className="flex space-x-3 text-[10px] text-slate-400 mb-2 font-mono">
          {from && <span>De : <strong className="text-slate-200">{from}</strong></span>}
          {to && <span>À : <strong className="text-slate-200">{to}</strong></span>}
        </div>
      )}

      <div className="rounded-lg bg-slate-950/60 p-2.5 font-mono text-xs leading-relaxed text-cyan-100 border border-slate-800/60">
        {content}
      </div>
    </div>
  );
};
