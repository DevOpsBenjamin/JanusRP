import React from 'react';
import { Heart, MapPin, Shield, User, Users } from 'lucide-react';
import { useGameStore } from '../../store';

export const ContextInspector: React.FC = () => {
  const { locations, currentLocationId, selectedLocationId, npcs } = useGameStore();

  const inspectedLocId = selectedLocationId || currentLocationId;
  const inspectedLoc = locations.find((l) => l.id === inspectedLocId);
  const isPlayerPresent = inspectedLocId === currentLocationId;

  // Filter NPCs present in the inspected location
  const presentNpcs = npcs.filter((n) => n.current_location_id === inspectedLocId);

  return (
    <div className="flex h-full flex-col bg-[#0b0f19] overflow-hidden">
      {/* Header */}
      <div className="flex h-11 items-center justify-between border-b border-slate-800/80 bg-slate-900/40 px-4">
        <div className="flex items-center space-x-2 text-xs font-semibold uppercase tracking-wider text-slate-300">
          <Users className="h-4 w-4 text-emerald-400" />
          <span>Inspecteur de Contexte</span>
        </div>
        <Shield className="h-3.5 w-3.5 text-slate-500" />
      </div>

      <div className="flex-1 overflow-y-auto p-4 space-y-5">
        {/* Location Section */}
        {inspectedLoc && (
          <div className="rounded-xl border border-slate-800 bg-slate-900/70 p-4 shadow-sm">
            <div className="flex items-center justify-between mb-2">
              <div className="flex items-center space-x-2">
                <MapPin className="h-4 w-4 text-amber-400" />
                <h3 className="font-bold text-sm text-slate-200">{inspectedLoc.name}</h3>
              </div>
              {isPlayerPresent && (
                <span className="rounded bg-amber-500/20 px-1.5 py-0.5 text-[10px] font-semibold text-amber-300 border border-amber-500/30">
                  Lieu actif PJ
                </span>
              )}
            </div>

            <p className="font-serif text-xs leading-relaxed text-slate-300 italic mb-2">
              {inspectedLoc.description}
            </p>

            {inspectedLoc.atmosphere && (
              <div className="mt-2 rounded-lg bg-slate-950/60 p-2 text-[11px] text-slate-400 border border-slate-800/60">
                <span className="font-semibold text-slate-300 block mb-0.5">Ambiance :</span>
                {inspectedLoc.atmosphere}
              </div>
            )}
          </div>
        )}

        {/* NPCs Section */}
        <div className="space-y-3">
          <div className="flex items-center justify-between text-xs font-bold uppercase tracking-wider text-slate-400">
            <span>PNJ Présents ({presentNpcs.length})</span>
          </div>

          {presentNpcs.length > 0 ? (
            presentNpcs.map((npc) => {
              const rel = npc.relationship || { affinity: 0, trust: 0, mood: 'neutre' };

              // Map -100..100 scale to 0..100% for the visual progress bar
              const affinityPct = Math.min(100, Math.max(0, ((rel.affinity + 100) / 200) * 100));
              const trustPct = Math.min(100, Math.max(0, ((rel.trust + 100) / 200) * 100));

              return (
                <div
                  key={npc.id}
                  className="rounded-xl border border-slate-800/90 bg-slate-900/70 p-4 space-y-3 shadow-sm hover:border-slate-700 transition-colors"
                >
                  <div className="flex items-start justify-between">
                    <div>
                      <div className="flex items-center space-x-1.5">
                        <User className="h-4 w-4 text-emerald-400" />
                        <h4 className="font-bold text-sm text-slate-200">{npc.name}</h4>
                      </div>
                      {npc.title && (
                        <p className="text-[11px] text-slate-400 ml-5.5">{npc.title}</p>
                      )}
                    </div>
                    <span className="rounded-md bg-amber-500/10 px-2 py-0.5 text-[10px] font-medium text-amber-300 border border-amber-500/20">
                      {rel.mood}
                    </span>
                  </div>

                  {npc.personality_traits && npc.personality_traits.length > 0 && (
                    <div className="flex flex-wrap gap-1">
                      {npc.personality_traits.map((trait) => (
                        <span
                          key={trait}
                          className="rounded bg-slate-800 px-1.5 py-0.5 text-[9px] text-slate-400"
                        >
                          {trait}
                        </span>
                      ))}
                    </div>
                  )}

                  {/* Relationship Gauges */}
                  <div className="space-y-2.5 pt-2 border-t border-slate-800/80 text-xs">
                    {/* Affinity Gauge */}
                    <div>
                      <div className="flex justify-between text-[11px] text-slate-400 mb-1">
                        <span className="flex items-center space-x-1">
                          <Heart className="h-3 w-3 text-rose-400" />
                          <span>Affinité</span>
                        </span>
                        <span className="font-mono text-slate-200 font-semibold">
                          {rel.affinity > 0 ? `+${rel.affinity}` : rel.affinity} / 100
                        </span>
                      </div>
                      <div className="h-1.5 w-full bg-slate-800 rounded-full overflow-hidden">
                        <div
                          className="h-full bg-gradient-to-r from-rose-500 to-amber-400 transition-all duration-500"
                          style={{ width: `${affinityPct}%` }}
                        />
                      </div>
                    </div>

                    {/* Trust Gauge */}
                    <div>
                      <div className="flex justify-between text-[11px] text-slate-400 mb-1">
                        <span className="flex items-center space-x-1">
                          <Shield className="h-3 w-3 text-emerald-400" />
                          <span>Confiance</span>
                        </span>
                        <span className="font-mono text-slate-200 font-semibold">
                          {rel.trust > 0 ? `+${rel.trust}` : rel.trust} / 100
                        </span>
                      </div>
                      <div className="h-1.5 w-full bg-slate-800 rounded-full overflow-hidden">
                        <div
                          className="h-full bg-gradient-to-r from-slate-600 to-emerald-400 transition-all duration-500"
                          style={{ width: `${trustPct}%` }}
                        />
                      </div>
                    </div>
                  </div>

                  {rel.interaction_summary && (
                    <div className="rounded bg-slate-950/50 p-2 text-[10px] text-slate-400 italic border border-slate-800/60">
                      « {rel.interaction_summary} »
                    </div>
                  )}
                </div>
              );
            })
          ) : (
            <div className="rounded-xl border border-dashed border-slate-800/80 p-6 text-center text-xs text-slate-500">
              Aucun personnage n'est présent dans ce lieu.
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
