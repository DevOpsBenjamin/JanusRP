import React from 'react';
import { Bookmark, Heart, MapPin, TrendingDown, TrendingUp } from 'lucide-react';
import { StateMutation } from '../../types';

interface MutationCardProps {
  mutation: StateMutation;
}

export const MutationCard: React.FC<MutationCardProps> = ({ mutation }) => {
  switch (mutation.type) {
    case 'relationship_update': {
      const { npc_name, delta_affinity, delta_trust, mood, reason } = mutation.payload;
      const hasAffinityDelta = delta_affinity !== undefined && delta_affinity !== null && delta_affinity !== 0;
      const hasTrustDelta = delta_trust !== undefined && delta_trust !== null && delta_trust !== 0;

      return (
        <div className="my-2 flex items-center justify-between rounded-lg border border-amber-500/20 bg-amber-950/30 px-3.5 py-2 text-xs">
          <div className="flex items-center space-x-2">
            <Heart className="h-3.5 w-3.5 text-amber-400" />
            <span className="font-semibold text-slate-200">{npc_name}</span>
            <span className="text-slate-400 font-serif italic">— « {reason} »</span>
          </div>
          <div className="flex items-center space-x-3 font-mono text-[11px]">
            {hasAffinityDelta && (
              <span
                className={`flex items-center space-x-0.5 ${
                  delta_affinity! > 0 ? 'text-emerald-400' : 'text-rose-400'
                }`}
              >
                {delta_affinity! > 0 ? (
                  <TrendingUp className="h-3 w-3 inline" />
                ) : (
                  <TrendingDown className="h-3 w-3 inline" />
                )}
                <span>
                  Affinité {delta_affinity! > 0 ? `+${delta_affinity}` : delta_affinity}
                </span>
              </span>
            )}
            {hasTrustDelta && (
              <span
                className={`flex items-center space-x-0.5 ${
                  delta_trust! > 0 ? 'text-emerald-400' : 'text-rose-400'
                }`}
              >
                {delta_trust! > 0 ? (
                  <TrendingUp className="h-3 w-3 inline" />
                ) : (
                  <TrendingDown className="h-3 w-3 inline" />
                )}
                <span>
                  Confiance {delta_trust! > 0 ? `+${delta_trust}` : delta_trust}
                </span>
              </span>
            )}
            {mood && (
              <span className="rounded bg-slate-800/80 px-1.5 py-0.5 text-[10px] text-amber-300">
                {mood}
              </span>
            )}
          </div>
        </div>
      );
    }
    case 'location_change': {
      const { location_name, narration_hint } = mutation.payload;
      return (
        <div className="my-2 flex items-center justify-between rounded-lg border border-cyan-500/20 bg-cyan-950/30 px-3.5 py-2 text-xs text-cyan-200">
          <div className="flex items-center space-x-2">
            <MapPin className="h-3.5 w-3.5 text-cyan-400" />
            <span>
              Déplacement vers : <strong className="text-cyan-100">{location_name}</strong>
            </span>
          </div>
          {narration_hint && (
            <span className="text-[11px] italic text-cyan-400/80 font-serif">
              {narration_hint}
            </span>
          )}
        </div>
      );
    }
    case 'event_logged': {
      const { summary, significance } = mutation.payload;
      return (
        <div className="my-2 flex items-center justify-between rounded-lg border border-slate-700/60 bg-slate-900/60 px-3.5 py-2 text-xs text-slate-300">
          <div className="flex items-center space-x-2">
            <Bookmark className="h-3.5 w-3.5 text-amber-400" />
            <span>Événement consigné : <em>{summary}</em></span>
          </div>
          <span className="rounded bg-slate-800 px-1.5 py-0.5 text-[10px] font-mono text-slate-400 uppercase">
            {significance}
          </span>
        </div>
      );
    }
    default:
      return null;
  }
};
