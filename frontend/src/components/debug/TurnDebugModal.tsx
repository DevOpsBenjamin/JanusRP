import React, { useState } from 'react';
import {
  Brain,
  Check,
  Code,
  Copy,
  FileText,
  Layers,
  Sparkles,
  Terminal,
  Wrench,
  X,
} from 'lucide-react';
import { TurnHistoryItem } from '../../types';
import { MutationCard } from '../narrative/MutationCard';

interface TurnDebugModalProps {
  turn: TurnHistoryItem | null;
  onClose: () => void;
}

type TabType = 'reasoning' | 'tools' | 'briefing' | 'narration' | 'raw';

export const TurnDebugModal: React.FC<TurnDebugModalProps> = ({ turn, onClose }) => {
  const [activeTab, setActiveTab] = useState<TabType>('reasoning');
  const [copied, setCopied] = useState(false);

  if (!turn) return null;

  const handleCopyJson = () => {
    navigator.clipboard.writeText(JSON.stringify(turn, null, 2));
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const toolCalls = turn.toolCalls || [];
  const mutations = turn.mutations || [];

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/80 p-4 backdrop-blur-sm"
      onClick={onClose}
    >
      <div
        className="relative flex h-[85vh] w-full max-w-4xl flex-col rounded-2xl border border-slate-800 bg-[#0c111d] shadow-2xl overflow-hidden text-slate-200"
        onClick={(e) => e.stopPropagation()}
      >
        {/* Modal Header */}
        <div className="flex h-14 items-center justify-between border-b border-slate-800/80 bg-slate-900/60 px-6">
          <div className="flex items-center space-x-3">
            <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-amber-500/10 text-amber-400 border border-amber-500/20">
              <Brain className="h-4 w-4" />
            </div>
            <div>
              <div className="flex items-center space-x-2">
                <h2 className="text-sm font-bold text-slate-100">
                  Inspecteur de Tour #{turn.turnNumber}
                </h2>
                <span className="rounded bg-slate-800 px-2 py-0.5 text-[10px] font-mono text-slate-400">
                  {turn.turnId.slice(0, 8)}...
                </span>
              </div>
              <p className="text-[11px] text-slate-400">
                Traces d'exécution Meta Muse Glimmer (MJ) & Qwen 3.8 (La Plume)
              </p>
            </div>
          </div>

          <div className="flex items-center space-x-2">
            <button
              onClick={handleCopyJson}
              className="flex items-center space-x-1.5 rounded-lg border border-slate-700 bg-slate-800/60 px-2.5 py-1 text-xs text-slate-300 hover:bg-slate-700/80 transition-colors"
              title="Copier le JSON complet"
            >
              {copied ? (
                <>
                  <Check className="h-3.5 w-3.5 text-emerald-400" />
                  <span className="text-emerald-300 font-medium">Copié !</span>
                </>
              ) : (
                <>
                  <Copy className="h-3.5 w-3.5 text-slate-400" />
                  <span>Copier JSON</span>
                </>
              )}
            </button>
            <button
              onClick={onClose}
              className="rounded-lg p-1.5 text-slate-400 hover:bg-slate-800 hover:text-slate-100 transition-colors"
            >
              <X className="h-4 w-4" />
            </button>
          </div>
        </div>

        {/* Navigation Tabs */}
        <div className="flex border-b border-slate-800/80 bg-slate-950/40 px-6 text-xs font-medium">
          <button
            onClick={() => setActiveTab('reasoning')}
            className={`flex items-center space-x-2 border-b-2 py-3 px-3 transition-colors ${
              activeTab === 'reasoning'
                ? 'border-amber-400 text-amber-300 font-semibold'
                : 'border-transparent text-slate-400 hover:text-slate-200'
            }`}
          >
            <Brain className="h-3.5 w-3.5" />
            <span>Délibération MJ</span>
          </button>

          <button
            onClick={() => setActiveTab('tools')}
            className={`flex items-center space-x-2 border-b-2 py-3 px-3 transition-colors ${
              activeTab === 'tools'
                ? 'border-cyan-400 text-cyan-300 font-semibold'
                : 'border-transparent text-slate-400 hover:text-slate-200'
            }`}
          >
            <Wrench className="h-3.5 w-3.5" />
            <span>Outils MCP & Mutations ({toolCalls.length})</span>
          </button>

          <button
            onClick={() => setActiveTab('briefing')}
            className={`flex items-center space-x-2 border-b-2 py-3 px-3 transition-colors ${
              activeTab === 'briefing'
                ? 'border-indigo-400 text-indigo-300 font-semibold'
                : 'border-transparent text-slate-400 hover:text-slate-200'
            }`}
          >
            <FileText className="h-3.5 w-3.5" />
            <span>Briefing Narratif</span>
          </button>

          <button
            onClick={() => setActiveTab('narration')}
            className={`flex items-center space-x-2 border-b-2 py-3 px-3 transition-colors ${
              activeTab === 'narration'
                ? 'border-emerald-400 text-emerald-300 font-semibold'
                : 'border-transparent text-slate-400 hover:text-slate-200'
            }`}
          >
            <Layers className="h-3.5 w-3.5" />
            <span>DSL & Rendu</span>
          </button>

          <button
            onClick={() => setActiveTab('raw')}
            className={`flex items-center space-x-2 border-b-2 py-3 px-3 transition-colors ${
              activeTab === 'raw'
                ? 'border-purple-400 text-purple-300 font-semibold'
                : 'border-transparent text-slate-400 hover:text-slate-200'
            }`}
          >
            <Code className="h-3.5 w-3.5" />
            <span>JSON Brut</span>
          </button>
        </div>

        {/* Tab Content */}
        <div className="flex-1 overflow-y-auto p-6 text-xs leading-relaxed">
          {/* Action Player banner */}
          <div className="mb-5 rounded-xl border border-slate-800 bg-slate-900/50 p-3.5">
            <span className="text-[10px] uppercase tracking-wider font-bold text-amber-400 block mb-1">
              Intention saisie par le joueur
            </span>
            <p className="font-medium text-slate-100 italic">« {turn.playerInput} »</p>
          </div>

          {/* TAB 1: REASONING */}
          {activeTab === 'reasoning' && (
            <div className="space-y-4">
              <div className="flex items-center justify-between text-xs text-slate-400 pb-2 border-b border-slate-800/60">
                <span className="font-semibold text-slate-200">
                  Raisonnement logique & Délibération impartiale (Muse Glimmer 30B)
                </span>
                <span className="text-[11px] text-amber-400/80">Canal reasoning_content</span>
              </div>

              {turn.mjReasoning ? (
                <div className="rounded-xl border border-slate-800/80 bg-slate-950/60 p-4 font-mono text-[11px] text-slate-300 whitespace-pre-wrap leading-relaxed shadow-inner">
                  {turn.mjReasoning}
                </div>
              ) : (
                <div className="rounded-xl border border-dashed border-slate-800 p-8 text-center text-slate-500">
                  Aucun raisonnement consigné pour ce tour.
                </div>
              )}
            </div>
          )}

          {/* TAB 2: TOOLS & MUTATIONS */}
          {activeTab === 'tools' && (
            <div className="space-y-6">
              {/* Tool Calls Section */}
              <div className="space-y-3">
                <h3 className="font-bold text-slate-200 text-xs uppercase tracking-wider flex items-center space-x-1.5">
                  <Terminal className="h-3.5 w-3.5 text-cyan-400" />
                  <span>Appels d'outils MCP ({toolCalls.length})</span>
                </h3>

                {toolCalls.length > 0 ? (
                  <div className="space-y-3">
                    {toolCalls.map((call: any, idx: number) => (
                      <div
                        key={idx}
                        className="rounded-xl border border-cyan-500/30 bg-cyan-950/20 p-4 shadow-sm"
                      >
                        <div className="flex items-center justify-between mb-2">
                          <span className="rounded bg-cyan-500/20 px-2 py-0.5 font-mono text-xs font-bold text-cyan-300 border border-cyan-500/30">
                            {call.name}
                          </span>
                          <span className="text-[10px] text-slate-400">Outil MCP #{idx + 1}</span>
                        </div>
                        <div className="rounded bg-slate-950/80 p-2.5 font-mono text-[11px] text-slate-300 overflow-x-auto">
                          <pre>{JSON.stringify(call.arguments, null, 2)}</pre>
                        </div>
                      </div>
                    ))}
                  </div>
                ) : (
                  <div className="rounded-xl border border-dashed border-slate-800 p-5 text-center text-slate-500">
                    Aucun outil MCP appelé lors de ce tour (interaction purement verbale ou d'observation).
                  </div>
                )}
              </div>

              {/* Mutations Section */}
              <div className="space-y-3">
                <h3 className="font-bold text-slate-200 text-xs uppercase tracking-wider flex items-center space-x-1.5">
                  <Sparkles className="h-3.5 w-3.5 text-emerald-400" />
                  <span>Mutations d'état appliquées ({mutations.length})</span>
                </h3>

                {mutations.length > 0 ? (
                  <div className="space-y-2">
                    {mutations.map((m, idx) => (
                      <MutationCard key={idx} mutation={m} />
                    ))}
                  </div>
                ) : (
                  <div className="rounded-xl border border-dashed border-slate-800 p-5 text-center text-slate-500">
                    Aucune mutation d'état persistante enregistrée pour ce tour.
                  </div>
                )}
              </div>
            </div>
          )}

          {/* TAB 3: BRIEFING */}
          {activeTab === 'briefing' && (
            <div className="space-y-4">
              <div className="flex items-center justify-between text-xs text-slate-400 pb-2 border-b border-slate-800/60">
                <span className="font-semibold text-slate-200">
                  Director Briefing transmis à La Plume (Qwen 3.8)
                </span>
                <span className="text-[11px] text-indigo-400/80">Consignes scénaristiques</span>
              </div>

              {turn.mjBriefing ? (
                <div className="rounded-xl border border-indigo-500/30 bg-indigo-950/20 p-5 shadow-inner space-y-3">
                  <div className="flex items-center space-x-2 text-indigo-300 font-semibold text-xs">
                    <FileText className="h-4 w-4" />
                    <span>Directives du Maître du Jeu :</span>
                  </div>
                  <div className="font-serif text-xs leading-relaxed text-slate-200 whitespace-pre-wrap">
                    {turn.mjBriefing}
                  </div>
                </div>
              ) : (
                <div className="rounded-xl border border-dashed border-slate-800 p-8 text-center text-slate-500">
                  Aucun briefing spécifique consigné pour ce tour.
                </div>
              )}
            </div>
          )}

          {/* TAB 4: NARRATION & DSL */}
          {activeTab === 'narration' && (
            <div className="space-y-5">
              <div>
                <h3 className="font-bold text-slate-200 text-xs uppercase tracking-wider mb-2">
                  Flux brut avec balises DSL XML
                </h3>
                <div className="rounded-xl border border-slate-800 bg-slate-950/80 p-4 font-mono text-[11px] text-emerald-300 whitespace-pre-wrap overflow-x-auto max-h-56">
                  {turn.rawNarration || '(Narration vide)'}
                </div>
              </div>

              <div>
                <h3 className="font-bold text-slate-200 text-xs uppercase tracking-wider mb-2">
                  Blocs découpés par le Streaming Parser ({turn.blocks.length})
                </h3>
                <div className="space-y-2">
                  {turn.blocks.map((block, idx) => (
                    <div
                      key={idx}
                      className="rounded-lg border border-slate-800 bg-slate-900/60 p-3 space-y-1"
                    >
                      <div className="flex items-center justify-between text-[10px] text-slate-400 font-mono">
                        <span className="font-bold text-amber-400 uppercase">{block.type}</span>
                        <span>Bloc #{idx + 1}</span>
                      </div>
                      <p className="font-serif text-xs text-slate-200 italic">
                        {'content' in block ? block.content : ''}
                      </p>
                    </div>
                  ))}
                </div>
              </div>
            </div>
          )}

          {/* TAB 5: RAW JSON */}
          {activeTab === 'raw' && (
            <div className="rounded-xl border border-slate-800 bg-slate-950 p-4 font-mono text-[11px] text-slate-300 overflow-x-auto">
              <pre>{JSON.stringify(turn, null, 2)}</pre>
            </div>
          )}
        </div>

        {/* Modal Footer */}
        <div className="flex h-11 items-center justify-between border-t border-slate-800/80 bg-slate-950/60 px-6 text-[11px] text-slate-500">
          <span>Campagne Les Brumes de Val-Corbeau</span>
          <span>Appuyez sur Échap ou cliquez à l'extérieur pour fermer</span>
        </div>
      </div>
    </div>
  );
};
