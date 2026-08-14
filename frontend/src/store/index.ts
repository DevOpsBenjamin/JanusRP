import { create } from 'zustand';
import { Campaign, Location, Npc, StreamStatus } from '../types';

interface GameState {
  campaign: Campaign | null;
  currentLocation: Location | null;
  locations: Location[];
  npcs: Npc[];
  status: StreamStatus;
  rawNarration: string;
  setCampaign: (campaign: Campaign) => void;
  setCurrentLocation: (location: Location) => void;
  setStatus: (status: StreamStatus) => void;
  appendNarration: (chunk: string) => void;
  resetNarration: () => void;
}

export const useGameStore = create<GameState>((set) => ({
  campaign: null,
  currentLocation: null,
  locations: [],
  npcs: [],
  status: 'idle',
  rawNarration: '',
  setCampaign: (campaign) => set({ campaign }),
  setCurrentLocation: (currentLocation) => set({ currentLocation }),
  setStatus: (status) => set({ status }),
  appendNarration: (chunk) => set((state) => ({ rawNarration: state.rawNarration + chunk })),
  resetNarration: () => set({ rawNarration: '' }),
}));
