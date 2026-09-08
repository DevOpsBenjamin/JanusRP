import React from 'react';
import { Ear, Eye, Flame, Hand, Wind } from 'lucide-react';

interface SensoryBlockProps {
  sensoryType?: string;
  content: string;
}

export const SensoryBlock: React.FC<SensoryBlockProps> = ({
  sensoryType = 'sound',
  content,
}) => {
  const getIconAndLabel = () => {
    switch (sensoryType.toLowerCase()) {
      case 'sound':
      case 'sonore':
        return {
          icon: <Ear className="h-3.5 w-3.5 text-rose-400" />,
          label: 'Stimulus sonore',
          color: 'border-rose-500/30 bg-rose-950/15 text-rose-200',
        };
      case 'smell':
      case 'olfactif':
        return {
          icon: <Wind className="h-3.5 w-3.5 text-emerald-400" />,
          label: 'Odeur & Ambiance',
          color: 'border-emerald-500/30 bg-emerald-950/15 text-emerald-200',
        };
      case 'sight':
      case 'visuel':
        return {
          icon: <Eye className="h-3.5 w-3.5 text-sky-400" />,
          label: 'Détail visuel',
          color: 'border-sky-500/30 bg-sky-950/15 text-sky-200',
        };
      case 'touch':
      case 'tactile':
        return {
          icon: <Hand className="h-3.5 w-3.5 text-orange-400" />,
          label: 'Sensation tactile',
          color: 'border-orange-500/30 bg-orange-950/15 text-orange-200',
        };
      default:
        return {
          icon: <Flame className="h-3.5 w-3.5 text-amber-400" />,
          label: 'Perception sensorielle',
          color: 'border-amber-500/30 bg-amber-950/15 text-amber-200',
        };
    }
  };

  const { icon, label, color } = getIconAndLabel();

  return (
    <div className={`my-2.5 rounded-xl border p-3 shadow-sm ${color}`}>
      <div className="flex items-center space-x-2 mb-1">
        {icon}
        <span className="text-[10px] font-bold uppercase tracking-wider opacity-90">
          {label}
        </span>
      </div>
      <p className="font-serif italic text-xs leading-relaxed opacity-95 pl-2 border-l border-current">
        {content}
      </p>
    </div>
  );
};
