import React from 'react';
import { Handle, Position } from '@xyflow/react';
import { Compass, MapPin, User } from 'lucide-react';
import { Npc } from '../../types';

export interface LocationNodeData extends Record<string, unknown> {
  id: string;
  name: string;
  slug: string;
  description: string;
  isCurrentLocation: boolean;
  npcsPresent: Npc[];
  onSelect?: (locationId: string) => void;
}

export const LocationNode: React.FC<{ data: LocationNodeData }> = ({ data }) => {
  const { name, isCurrentLocation, npcsPresent, id, onSelect } = data;

  return (
    <div
      onClick={() => onSelect?.(id)}
      className={`relative min-w-[180px] max-w-[220px] rounded-xl border p-3.5 shadow-lg transition-all cursor-pointer ${
        isCurrentLocation
          ? 'border-amber-500 bg-slate-900/95 ring-2 ring-amber-500/40 shadow-amber-500/10'
          : 'border-slate-800 bg-slate-950/80 hover:border-slate-700'
      }`}
    >
      <Handle type="target" position={Position.Top} className="!bg-slate-700 !w-2 !h-2" />
      <Handle type="source" position={Position.Bottom} className="!bg-slate-700 !w-2 !h-2" />
      <Handle type="target" position={Position.Left} className="!bg-slate-700 !w-2 !h-2" />
      <Handle type="source" position={Position.Right} className="!bg-slate-700 !w-2 !h-2" />

      {/* Header with status badge */}
      <div className="flex items-center justify-between mb-1.5">
        <div className="flex items-center space-x-1.5">
          <MapPin
            className={`h-3.5 w-3.5 ${
              isCurrentLocation ? 'text-amber-400' : 'text-slate-500'
            }`}
          />
          <h3 className="font-semibold text-xs text-slate-200 truncate">{name}</h3>
        </div>
        {isCurrentLocation && (
          <span className="flex items-center space-x-1 rounded-full bg-amber-500/20 px-1.5 py-0.5 text-[9px] font-bold text-amber-300 border border-amber-500/30 animate-pulse">
            <Compass className="h-2.5 w-2.5" />
            <span>PJ</span>
          </span>
        )}
      </div>

      {/* NPCs present badge */}
      {npcsPresent.length > 0 ? (
        <div className="mt-2 flex flex-wrap gap-1 border-t border-slate-800/80 pt-1.5">
          {npcsPresent.map((npc) => (
            <span
              key={npc.id}
              className="inline-flex items-center space-x-1 rounded bg-slate-800/90 px-1.5 py-0.5 text-[10px] text-slate-300 border border-slate-700/50"
            >
              <User className="h-2.5 w-2.5 text-emerald-400" />
              <span>{npc.name}</span>
            </span>
          ))}
        </div>
      ) : (
        <div className="mt-1.5 text-[10px] text-slate-600 italic">Aucun PNJ</div>
      )}
    </div>
  );
};
