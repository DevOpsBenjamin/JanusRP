import React, { useState } from 'react';
import { Compass, Sparkles, MessageSquare, Shield, Users, MapPin, Send } from 'lucide-react';

export const App: React.FC = () => {
  const [inputAction, setInputAction] = useState('');

  return (
    <div className="flex flex-col h-screen w-screen bg-[#090d14] text-slate-200">
      {/* Top Navigation Bar */}
      <header className="h-14 border-b border-slate-800/80 bg-[#0f1420]/90 px-6 flex items-center justify-between backdrop-blur">
        <div className="flex items-center space-x-3">
          <div className="h-8 w-8 rounded-lg bg-gradient-to-tr from-amber-500 to-amber-300 flex items-center justify-center shadow-lg shadow-amber-500/20">
            <Compass className="h-5 w-5 text-slate-950" />
          </div>
          <div>
            <h1 className="font-bold text-base tracking-wide bg-gradient-to-r from-slate-100 to-slate-400 bg-clip-text text-transparent">
              JanusRP
            </h1>
            <p className="text-[11px] text-slate-400 font-medium -mt-0.5">
              Les Brumes de Val-Corbeau
            </p>
          </div>
        </div>

        <div className="flex items-center space-x-4">
          <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-emerald-500/10 text-emerald-400 border border-emerald-500/20">
            <span className="w-1.5 h-1.5 rounded-full bg-emerald-400 mr-1.5 animate-pulse"></span>
            Prêt
          </span>
          <div className="h-4 w-px bg-slate-800" />
          <span className="text-xs text-slate-400 font-mono">Tour #0</span>
        </div>
      </header>

      {/* 3-Panel Main Layout */}
      <main className="flex-1 flex overflow-hidden">
        {/* Left Panel: Spatial Map (ReactFlow Viewport) */}
        <section className="w-[32%] border-r border-slate-800/80 bg-[#0b0f19] flex flex-col relative">
          <div className="p-3 border-b border-slate-800/60 flex items-center justify-between bg-slate-900/40">
            <div className="flex items-center space-x-2 text-xs font-semibold text-slate-300 uppercase tracking-wider">
              <MapPin className="h-4 w-4 text-amber-400" />
              <span>Graphe Spatial</span>
            </div>
            <span className="text-[11px] text-slate-500 font-mono">Vue Topologique</span>
          </div>

          <div className="flex-1 flex items-center justify-center p-6 text-center text-slate-500 bg-[#0b0f19]">
            <div className="border border-dashed border-slate-800 rounded-xl p-8 max-w-xs">
              <Compass className="h-8 w-8 mx-auto text-slate-600 mb-2 animate-spin-slow" />
              <p className="text-sm font-medium text-slate-400">Salle Commune</p>
              <p className="text-xs text-slate-600 mt-1">L'auberge est silencieuse, les braises crépitent.</p>
            </div>
          </div>
        </section>

        {/* Center Panel: Narrative Stream Console */}
        <section className="w-[46%] flex flex-col bg-[#0d121c]">
          <div className="p-3 border-b border-slate-800/60 flex items-center justify-between bg-slate-900/40">
            <div className="flex items-center space-x-2 text-xs font-semibold text-slate-300 uppercase tracking-wider">
              <MessageSquare className="h-4 w-4 text-cyan-400" />
              <span>Console Narrative</span>
            </div>
            <div className="flex items-center space-x-1.5 text-[11px] text-slate-400">
              <Sparkles className="h-3.5 w-3.5 text-amber-400" />
              <span>Qwen 3.8 & Muse Glimmer</span>
            </div>
          </div>

          {/* Messages & Narration Flow */}
          <div className="flex-1 overflow-y-auto p-6 space-y-6">
            <div className="bg-slate-900/60 rounded-xl p-5 border border-slate-800/70 shadow-sm leading-relaxed text-sm text-slate-300 space-y-4">
              <p className="text-slate-400 italic">
                L'odeur de pin brûlé et de bière tiède emplit la salle commune. À travers les vitres embuées, les collines de Val-Corbeau s'effacent sous une brume d'encre.
              </p>
              <div className="border-l-2 border-amber-500/80 pl-4 py-1 text-slate-200">
                <span className="text-xs font-bold text-amber-400 uppercase tracking-wider block mb-1">Elena la tavernière</span>
                <p>« La nuit sera rude pour les voyageurs égarés. Approchez-vous de l'âtre avant que le froid ne vous gagne. »</p>
              </div>
            </div>
          </div>

          {/* Player Input Form */}
          <div className="p-4 border-t border-slate-800/80 bg-[#0f1422]">
            <form
              onSubmit={(e) => {
                e.preventDefault();
                setInputAction('');
              }}
              className="flex items-center space-x-2 bg-slate-900/80 rounded-xl border border-slate-700/60 px-3.5 py-2 focus-within:border-amber-500/60 focus-within:ring-1 focus-within:ring-amber-500/30 transition-all"
            >
              <input
                type="text"
                value={inputAction}
                onChange={(e) => setInputAction(e.target.value)}
                placeholder="Exprimez votre intention ou réplique..."
                className="flex-1 bg-transparent border-0 text-sm text-slate-100 placeholder-slate-500 focus:outline-none"
              />
              <button
                type="submit"
                className="p-1.5 rounded-lg bg-amber-500 text-slate-950 font-semibold hover:bg-amber-400 transition-colors disabled:opacity-50"
                disabled={!inputAction.trim()}
              >
                <Send className="h-4 w-4" />
              </button>
            </form>
          </div>
        </section>

        {/* Right Panel: Social & Context Inspector */}
        <section className="w-[22%] border-l border-slate-800/80 bg-[#0b0f19] flex flex-col">
          <div className="p-3 border-b border-slate-800/60 flex items-center justify-between bg-slate-900/40">
            <div className="flex items-center space-x-2 text-xs font-semibold text-slate-300 uppercase tracking-wider">
              <Users className="h-4 w-4 text-emerald-400" />
              <span>PNJ & Relations</span>
            </div>
            <Shield className="h-3.5 w-3.5 text-slate-500" />
          </div>

          <div className="flex-1 overflow-y-auto p-4 space-y-4">
            <div className="bg-slate-900/70 border border-slate-800 rounded-xl p-4 space-y-3">
              <div className="flex items-center justify-between">
                <div>
                  <h2 className="text-sm font-semibold text-slate-200">Elena</h2>
                  <p className="text-[11px] text-slate-400">Tavernière</p>
                </div>
                <span className="text-[10px] px-2 py-0.5 rounded-md bg-amber-500/10 text-amber-300 border border-amber-500/20">
                  Bienveillante
                </span>
              </div>

              {/* Gauges */}
              <div className="space-y-2 pt-2 border-t border-slate-800/80 text-xs">
                <div>
                  <div className="flex justify-between text-[11px] text-slate-400 mb-1">
                    <span>Affinité</span>
                    <span className="font-mono text-slate-300">0 / 100</span>
                  </div>
                  <div className="h-1.5 w-full bg-slate-800 rounded-full overflow-hidden">
                    <div className="h-full bg-amber-400 rounded-full w-1/2" />
                  </div>
                </div>
                <div>
                  <div className="flex justify-between text-[11px] text-slate-400 mb-1">
                    <span>Confiance</span>
                    <span className="font-mono text-slate-300">20 / 100</span>
                  </div>
                  <div className="h-1.5 w-full bg-slate-800 rounded-full overflow-hidden">
                    <div className="h-full bg-emerald-400 rounded-full w-3/5" />
                  </div>
                </div>
              </div>
            </div>
          </div>
        </section>
      </main>
    </div>
  );
};

export default App;
