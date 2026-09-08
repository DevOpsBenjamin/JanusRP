import React, { useEffect, useState } from 'react';
import { Compass, MessageSquare, Send, Sparkles, Square } from 'lucide-react';
import { useGameStore } from './store';
import { useTurnStream } from './hooks/useTurnStream';
import { SpatialMap } from './components/map/SpatialMap';
import { NarrativeConsole } from './components/console/NarrativeConsole';
import { ContextInspector } from './components/inspector/ContextInspector';

export const App: React.FC = () => {
  const [inputAction, setInputAction] = useState('');
  const { campaign, status, currentLocationId, locations, fetchRemoteCampaign } = useGameStore();
  const { submitTurn, abort, isStreaming } = useTurnStream();

  useEffect(() => {
    fetchRemoteCampaign(campaign.id);
  }, [campaign.id, fetchRemoteCampaign]);

  const currentLoc = locations.find((l) => l.id === currentLocationId);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!inputAction.trim() || isStreaming) return;
    submitTurn(inputAction);
    setInputAction('');
  };

  const getStatusBadge = () => {
    switch (status) {
      case 'thinking':
        return (
          <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-cyan-500/10 text-cyan-400 border border-cyan-500/20">
            <span className="w-1.5 h-1.5 rounded-full bg-cyan-400 mr-1.5 animate-ping"></span>
            Arbitrage MJ...
          </span>
        );
      case 'calling_tools':
        return (
          <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-purple-500/10 text-purple-400 border border-purple-500/20">
            <span className="w-1.5 h-1.5 rounded-full bg-purple-400 mr-1.5 animate-pulse"></span>
            Mise à jour du monde...
          </span>
        );
      case 'streaming':
        return (
          <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-amber-500/10 text-amber-400 border border-amber-500/20">
            <span className="w-1.5 h-1.5 rounded-full bg-amber-400 mr-1.5 animate-pulse"></span>
            Narration en direct...
          </span>
        );
      case 'error':
        return (
          <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-rose-500/10 text-rose-400 border border-rose-500/20">
            <span className="w-1.5 h-1.5 rounded-full bg-rose-400 mr-1.5"></span>
            Erreur
          </span>
        );
      case 'idle':
      default:
        return (
          <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-emerald-500/10 text-emerald-400 border border-emerald-500/20">
            <span className="w-1.5 h-1.5 rounded-full bg-emerald-400 mr-1.5"></span>
            Prêt
          </span>
        );
    }
  };

  return (
    <div className="flex flex-col h-screen w-screen bg-[#090d14] text-slate-200 overflow-hidden select-none">
      {/* Top Navigation Bar */}
      <header className="h-14 border-b border-slate-800/80 bg-[#0f1420]/90 px-6 flex items-center justify-between backdrop-blur shrink-0 z-10">
        <div className="flex items-center space-x-3">
          <div className="h-8 w-8 rounded-lg bg-gradient-to-tr from-amber-500 to-amber-300 flex items-center justify-center shadow-lg shadow-amber-500/20">
            <Compass className="h-5 w-5 text-slate-950" />
          </div>
          <div>
            <h1 className="font-bold text-base tracking-wide bg-gradient-to-r from-slate-100 to-slate-400 bg-clip-text text-transparent">
              JanusRP
            </h1>
            <p className="text-[11px] text-slate-400 font-medium -mt-0.5">
              {campaign.title} {currentLoc ? `— ${currentLoc.name}` : ''}
            </p>
          </div>
        </div>

        <div className="flex items-center space-x-4">
          {getStatusBadge()}
          <div className="h-4 w-px bg-slate-800" />
          <span className="text-xs text-slate-400 font-mono">
            Tour #{campaign.turn_count}
          </span>
        </div>
      </header>

      {/* 3-Panel Main Layout */}
      <main className="flex-1 flex overflow-hidden">
        {/* Left Panel (35%): Spatial Topology Map (ReactFlow) */}
        <section className="w-[35%] border-r border-slate-800/80 bg-[#0b0f19] flex flex-col relative">
          <div className="h-11 border-b border-slate-800/60 flex items-center justify-between px-4 bg-slate-900/40 shrink-0">
            <div className="flex items-center space-x-2 text-xs font-semibold text-slate-300 uppercase tracking-wider">
              <Compass className="h-4 w-4 text-amber-400" />
              <span>Graphe Spatial</span>
            </div>
            <span className="text-[11px] text-slate-500 font-mono">Vue Topologique</span>
          </div>

          <div className="flex-1 w-full h-full overflow-hidden">
            <SpatialMap />
          </div>
        </section>

        {/* Center Panel (45%): Narrative Stream Console */}
        <section className="w-[45%] flex flex-col bg-[#0d121c] border-r border-slate-800/80">
          <div className="h-11 border-b border-slate-800/60 flex items-center justify-between px-4 bg-slate-900/40 shrink-0">
            <div className="flex items-center space-x-2 text-xs font-semibold text-slate-300 uppercase tracking-wider">
              <MessageSquare className="h-4 w-4 text-cyan-400" />
              <span>Console Narrative</span>
            </div>
            <div className="flex items-center space-x-1.5 text-[11px] text-slate-400">
              <Sparkles className="h-3.5 w-3.5 text-amber-400" />
              <span>Qwen 3.8 & Muse Glimmer</span>
            </div>
          </div>

          {/* Narrative Stream & Turns History */}
          <NarrativeConsole />

          {/* Player Input Form */}
          <div className="p-4 border-t border-slate-800/80 bg-[#0f1422] shrink-0">
            <form
              onSubmit={handleSubmit}
              className="flex items-center space-x-2 bg-slate-900/90 rounded-xl border border-slate-700/60 px-3.5 py-2 focus-within:border-amber-500/60 focus-within:ring-1 focus-within:ring-amber-500/30 transition-all shadow-inner"
            >
              <input
                type="text"
                value={inputAction}
                onChange={(e) => setInputAction(e.target.value)}
                disabled={isStreaming}
                placeholder={
                  isStreaming
                    ? 'Tour en cours de génération...'
                    : 'Exprimez votre action, dialogue ou intention...'
                }
                className="flex-1 bg-transparent border-0 text-sm text-slate-100 placeholder-slate-500 focus:outline-none disabled:opacity-60"
              />

              {isStreaming ? (
                <button
                  type="button"
                  onClick={abort}
                  title="Interrompre le tour"
                  className="flex items-center space-x-1 px-2.5 py-1.5 rounded-lg bg-rose-600/80 text-white text-xs font-semibold hover:bg-rose-600 transition-colors shadow-sm"
                >
                  <Square className="h-3.5 w-3.5 fill-current" />
                  <span>Stop</span>
                </button>
              ) : (
                <button
                  type="submit"
                  disabled={!inputAction.trim()}
                  className="p-1.5 rounded-lg bg-amber-500 text-slate-950 font-semibold hover:bg-amber-400 transition-colors disabled:opacity-40 disabled:cursor-not-allowed"
                >
                  <Send className="h-4 w-4" />
                </button>
              )}
            </form>
          </div>
        </section>

        {/* Right Panel (20%): Social & Context Inspector */}
        <section className="w-[20%] flex flex-col bg-[#0b0f19]">
          <ContextInspector />
        </section>
      </main>
    </div>
  );
};

export default App;
