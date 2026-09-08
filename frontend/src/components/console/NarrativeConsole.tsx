import React, { useEffect, useRef, useState } from 'react';
import { AlertCircle, Brain, Loader2, Sparkles, User } from 'lucide-react';
import { useGameStore } from '../../store';
import { TurnHistoryItem } from '../../types';
import { parseRPStream } from '../../utils/streamingRpParser';
import { RPBlockRenderer } from '../narrative/RPBlockRenderer';
import { MutationCard } from '../narrative/MutationCard';
import { TurnDebugModal } from '../debug/TurnDebugModal';

export const NarrativeConsole: React.FC = () => {
  const { turns, currentTurn, status, error } = useGameStore();
  const bottomRef = useRef<HTMLDivElement>(null);
  const [inspectingTurn, setInspectingTurn] = useState<TurnHistoryItem | null>(null);

  // Parse streaming chunks in real-time
  const currentBlocks = parseRPStream(currentTurn.rawNarration);

  // Auto-scroll on new content
  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [currentTurn.rawNarration, currentTurn.mutations, status, turns.length]);

  const isWorking = status === 'thinking' || status === 'calling_tools' || status === 'streaming';

  return (
    <div className="flex-1 overflow-y-auto p-6 space-y-6">
      {/* Welcome Intro if no turns yet */}
      {turns.length === 0 && !currentTurn.playerInput && (
        <div className="rounded-xl border border-slate-800 bg-slate-900/40 p-6 text-center space-y-3">
          <div className="mx-auto flex h-10 w-10 items-center justify-center rounded-full bg-amber-500/10 text-amber-400">
            <Sparkles className="h-5 w-5" />
          </div>
          <h3 className="font-bold text-base text-slate-100 font-serif">
            Bienvenue dans Les Brumes de Val-Corbeau
          </h3>
          <p className="text-xs text-slate-400 max-w-md mx-auto leading-relaxed">
            Vous vous trouvez dans la salle commune de l'auberge. Exprimez librement vos intentions,
            parlez aux personnages présents ou explorez les alentours. Le Maître du Jeu arbitrera vos
            actions et La Plume contera votre aventure.
          </p>
        </div>
      )}

      {/* Historical Turns */}
      {turns.map((turn) => (
        <div key={turn.turnId} className="space-y-4">
          {/* Player Input Entry */}
          <div className="flex justify-end">
            <div className="flex items-start space-x-2 max-w-lg">
              <div className="rounded-xl bg-slate-800/90 border border-slate-700/60 px-4 py-2.5 text-xs text-slate-100 shadow-md">
                <span className="font-bold text-amber-400 text-[10px] uppercase block mb-0.5">
                  Action — Tour #{turn.turnNumber}
                </span>
                <p>{turn.playerInput}</p>
              </div>
              <div className="flex h-7 w-7 items-center justify-center rounded-full bg-amber-500/20 text-amber-400 shrink-0 mt-0.5">
                <User className="h-4 w-4" />
              </div>
            </div>
          </div>

          {/* Mutations during this turn */}
          {turn.mutations && turn.mutations.length > 0 && (
            <div className="space-y-1 my-2">
              {turn.mutations.map((m, idx) => (
                <MutationCard key={idx} mutation={m} />
              ))}
            </div>
          )}

          {/* Narrative Response */}
          <div className="rounded-xl border border-slate-800/80 bg-slate-900/60 p-5 shadow-sm space-y-2">
            <div className="flex items-center justify-between border-b border-slate-800/50 pb-2 mb-2">
              <span className="text-[10px] font-mono text-slate-500 uppercase tracking-wider">
                Tour #{turn.turnNumber} — La Plume
              </span>
              <button
                onClick={() => setInspectingTurn(turn)}
                className="flex items-center space-x-1.5 rounded-md bg-slate-800/80 hover:bg-amber-500/20 hover:text-amber-300 hover:border-amber-500/40 border border-slate-700/60 px-2 py-0.5 text-[11px] font-medium text-slate-300 transition-all cursor-pointer shadow-xs"
                title="Inspecter le raisonnement du MJ, les outils MCP et le briefing"
              >
                <Brain className="h-3 w-3 text-amber-400" />
                <span>Inspecter Tour #{turn.turnNumber}</span>
              </button>
            </div>

            {turn.blocks.length > 0 ? (
              turn.blocks.map((block, idx) => (
                <RPBlockRenderer key={idx} block={block} />
              ))
            ) : (
              <p className="font-serif text-xs italic text-slate-400">
                {turn.rawNarration || "(Narration vide)"}
              </p>
            )}
          </div>
        </div>
      ))}

      {/* Active Streaming Turn */}
      {currentTurn.playerInput && (
        <div className="space-y-4 pt-2 border-t border-slate-800/60">
          {/* Current Player Action */}
          <div className="flex justify-end">
            <div className="flex items-start space-x-2 max-w-lg">
              <div className="rounded-xl bg-slate-800/90 border border-slate-700/60 px-4 py-2.5 text-xs text-slate-100 shadow-md">
                <span className="font-bold text-amber-400 text-[10px] uppercase block mb-0.5">
                  Action — Tour #{currentTurn.turnIndex}
                </span>
                <p>{currentTurn.playerInput}</p>
              </div>
              <div className="flex h-7 w-7 items-center justify-center rounded-full bg-amber-500/20 text-amber-400 shrink-0 mt-0.5">
                <User className="h-4 w-4" />
              </div>
            </div>
          </div>

          {/* MJ Thinking Indicator */}
          {isWorking && (
            <div className="flex items-center space-x-2.5 rounded-lg border border-cyan-500/20 bg-cyan-950/20 px-3.5 py-2 text-xs text-cyan-300">
              <Loader2 className="h-3.5 w-3.5 animate-spin text-cyan-400" />
              <span>
                {currentTurn.thinkingSummary || 'Le Maître du Jeu délibère...'}
              </span>
            </div>
          )}

          {/* Mutations occurring in real-time */}
          {currentTurn.mutations.length > 0 && (
            <div className="space-y-1 my-2">
              {currentTurn.mutations.map((m, idx) => (
                <MutationCard key={idx} mutation={m} />
              ))}
            </div>
          )}

          {/* Real-time Streaming Narration */}
          {(currentBlocks.length > 0 || currentTurn.rawNarration) && (
            <div className="rounded-xl border border-slate-800/80 bg-slate-900/60 p-5 shadow-sm relative">
              {currentBlocks.map((block, idx) => (
                <RPBlockRenderer key={idx} block={block} />
              ))}

              {status === 'streaming' && (
                <span className="inline-block h-3 w-1.5 bg-amber-400 animate-pulse ml-1" />
              )}
            </div>
          )}

          {/* Error display */}
          {status === 'error' && error && (
            <div className="flex items-center space-x-2 rounded-lg border border-rose-500/40 bg-rose-950/40 p-3 text-xs text-rose-200">
              <AlertCircle className="h-4 w-4 text-rose-400 shrink-0" />
              <span>{error}</span>
            </div>
          )}
        </div>
      )}

      <div ref={bottomRef} />

      {/* Turn Inspection / Debug Modal */}
      <TurnDebugModal
        turn={inspectingTurn}
        onClose={() => setInspectingTurn(null)}
      />
    </div>
  );
};
