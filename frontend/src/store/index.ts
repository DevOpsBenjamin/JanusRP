import { create } from 'zustand';
import {
  Campaign,
  Location,
  LocationEdge,
  Npc,
  StateMutation,
  StreamStatus,
  TurnHistoryItem,
  TurnStreamEvent,
} from '../types';
import { parseRPStream } from '../utils/streamingRpParser';

export const STARTER_CAMPAIGN_ID = 'a0000000-0000-0000-0000-000000000001';
export const SALLE_COMMUNE_LOCATION_ID = 'b0000000-0000-0000-0000-000000000001';
export const ARRIERE_COUR_LOCATION_ID = 'b0000000-0000-0000-0000-000000000002';
export const CHAMBRE_HAUTE_LOCATION_ID = 'b0000000-0000-0000-0000-000000000003';
export const ELENA_NPC_ID = 'd0000000-0000-0000-0000-000000000001';
export const GASTON_NPC_ID = 'd0000000-0000-0000-0000-000000000002';

interface CurrentTurnState {
  turnId: string;
  turnIndex: number;
  playerInput: string;
  thinkingStatus: string;
  thinkingSummary: string;
  rawNarration: string;
  mutations: StateMutation[];
}

interface GameState {
  campaign: Campaign;
  locations: Location[];
  locationEdges: LocationEdge[];
  currentLocationId: string;
  selectedLocationId: string | null;
  npcs: Npc[];
  turns: TurnHistoryItem[];
  status: StreamStatus;
  error: string | null;
  currentTurn: CurrentTurnState;

  // Actions
  initStarterCampaign: () => void;
  fetchRemoteCampaign: (campaignId?: string) => Promise<void>;
  setSelectedLocationId: (locationId: string | null) => void;
  startTurn: (playerInput: string) => void;
  handleTurnStreamEvent: (event: TurnStreamEvent) => void;
  setError: (error: string | null) => void;
  abortTurn: () => void;
}

const defaultStarterCampaign: Campaign = {
  id: STARTER_CAMPAIGN_ID,
  title: 'Les Brumes de Val-Corbeau',
  description:
    'Une auberge isolée au cœur des forêts brumeuses du Val-Corbeau, carrefour de voyageurs et de fugitifs.',
  player_name: 'Aventurier',
  current_location_id: SALLE_COMMUNE_LOCATION_ID,
  turn_count: 0,
};

const defaultLocations: Location[] = [
  {
    id: SALLE_COMMUNE_LOCATION_ID,
    campaign_id: STARTER_CAMPAIGN_ID,
    slug: 'salle-commune',
    name: 'La Salle Commune',
    description:
      "La pièce maîtresse de l'auberge, chaleureuse et enfumée. Un grand feu crépite dans l'âtre en pierre de taille, projetant des ombres dansantes sur les tables de chêne massif.",
    atmosphere:
      "Odeur de pin brûlé, de bière tiède et de graisse rance. Les flammes dansent doucement.",
    secrets: 'Une trappe camouflée mène à la cave sous le comptoir.',
    position_x: 0,
    position_y: 0,
  },
  {
    id: ARRIERE_COUR_LOCATION_ID,
    campaign_id: STARTER_CAMPAIGN_ID,
    slug: 'arriere-cour',
    name: "L'Arrière-Cour",
    description:
      "Une cour boueuse et sombre ceinturée d'une palissade vermoulue. Des tonneaux vides s'empilent près d'une remise à bois.",
    atmosphere: 'Froideur humide, vent sifflant entre les planches disjointes.',
    position_x: 260,
    position_y: 0,
  },
  {
    id: CHAMBRE_HAUTE_LOCATION_ID,
    campaign_id: STARTER_CAMPAIGN_ID,
    slug: 'chambre-haute',
    name: 'La Chambre Haute',
    description:
      "Une chambre austère à l'étage, réservée aux hôtes de passage ou aux conciliabules secrets.",
    atmosphere: 'Silence feutré, plancher qui grince sous les pas.',
    secrets: 'Une missive scellée au cachet de cire noire est dissimulée sous une latte.',
    position_x: 0,
    position_y: -180,
  },
];

const defaultEdges: LocationEdge[] = [
  {
    id: 'c0000000-0000-0000-0000-000000000001',
    campaign_id: STARTER_CAMPAIGN_ID,
    source_location_id: SALLE_COMMUNE_LOCATION_ID,
    target_location_id: ARRIERE_COUR_LOCATION_ID,
    is_locked: false,
    travel_description: 'Une porte battante mène vers la cour extérieure.',
  },
  {
    id: 'c0000000-0000-0000-0000-000000000002',
    campaign_id: STARTER_CAMPAIGN_ID,
    source_location_id: SALLE_COMMUNE_LOCATION_ID,
    target_location_id: CHAMBRE_HAUTE_LOCATION_ID,
    is_locked: true,
    lock_reason: "Porte verrouillée à clé depuis l'étage",
    travel_description: 'Un escalier étroit en colimaçon monte vers la chambre.',
  },
];

const defaultNpcs: Npc[] = [
  {
    id: ELENA_NPC_ID,
    campaign_id: STARTER_CAMPAIGN_ID,
    current_location_id: SALLE_COMMUNE_LOCATION_ID,
    slug: 'elena-taverniere',
    name: 'Elena',
    title: 'Tavernière de Val-Corbeau',
    personality_traits: ['vigilante', 'accueillante', 'protectrice'],
    background:
      "Elena tient l'auberge depuis la disparition de son époux. Elle protège discrètement les fugitifs de la milice.",
    relationship: {
      affinity: 0,
      trust: 20,
      mood: 'bienveillante mais vigilante',
      interaction_summary: "Vous observe attentivement derrière son comptoir.",
    },
  },
  {
    id: GASTON_NPC_ID,
    campaign_id: STARTER_CAMPAIGN_ID,
    current_location_id: ARRIERE_COUR_LOCATION_ID,
    slug: 'gaston-rodeur',
    name: 'Gaston',
    title: 'Rôdeur des collines',
    personality_traits: ['méfiant', 'silencieux', 'agile'],
    background:
      'Gaston surveille les allées et venues pour le compte des rebelles du col.',
    relationship: {
      affinity: -20,
      trust: 0,
      mood: 'méfiant',
      interaction_summary: 'Garde une main près de son poignard.',
    },
  },
];

const initialCurrentTurn: CurrentTurnState = {
  turnId: '',
  turnIndex: 1,
  playerInput: '',
  thinkingStatus: '',
  thinkingSummary: '',
  rawNarration: '',
  mutations: [],
};

export const useGameStore = create<GameState>((set, get) => ({
  campaign: defaultStarterCampaign,
  locations: defaultLocations,
  locationEdges: defaultEdges,
  currentLocationId: SALLE_COMMUNE_LOCATION_ID,
  selectedLocationId: null,
  npcs: defaultNpcs,
  turns: [],
  status: 'idle',
  error: null,
  currentTurn: initialCurrentTurn,

  initStarterCampaign: () => {
    set({
      campaign: defaultStarterCampaign,
      locations: defaultLocations,
      locationEdges: defaultEdges,
      currentLocationId: SALLE_COMMUNE_LOCATION_ID,
      npcs: defaultNpcs,
      turns: [],
      status: 'idle',
      error: null,
      currentTurn: initialCurrentTurn,
    });
  },

  fetchRemoteCampaign: async (campaignId?: string) => {
    const id = campaignId || get().campaign.id;
    try {
      const res = await fetch(`/api/campaigns/${id}`);
      if (res.ok) {
        const data = await res.json();
        if (data.campaign) {
          set((state) => ({
            campaign: {
              ...state.campaign,
              ...data.campaign,
            },
            currentLocationId:
              data.campaign.current_location_id || state.currentLocationId,
          }));
        }
      }

      // Also fetch turn history from backend
      const turnsRes = await fetch(`/api/campaigns/${id}/turns`);
      if (turnsRes.ok) {
        const remoteTurns = await turnsRes.json();
        if (Array.isArray(remoteTurns) && remoteTurns.length > 0) {
          const sorted = [...remoteTurns].sort((a, b) => a.turn_number - b.turn_number);
          const historyItems: TurnHistoryItem[] = sorted.map((t) => ({
            turnId: t.id,
            turnNumber: t.turn_number,
            playerInput: t.player_input,
            rawNarration: t.final_narration,
            blocks: parseRPStream(t.final_narration),
            mutations: t.metadata?.mutations || [],
            turnSummary: t.mj_reasoning,
            mjReasoning: t.mj_reasoning,
            mjBriefing: t.mj_briefing,
            toolCalls: t.metadata?.tool_calls || [],
            createdAt: t.created_at,
          }));
          set({ turns: historyItems });
        }
      }
    } catch {
      // Fallback silently to in-memory state
    }
  },

  setSelectedLocationId: (selectedLocationId) => set({ selectedLocationId }),

  startTurn: (playerInput: string) => {
    const nextTurnIndex = get().campaign.turn_count + 1;
    set({
      status: 'thinking',
      error: null,
      currentTurn: {
        ...initialCurrentTurn,
        turnIndex: nextTurnIndex,
        playerInput,
        thinkingSummary: "Préparation de l'arbitrage...",
      },
    });
  },

  handleTurnStreamEvent: (event: TurnStreamEvent) => {
    switch (event.event) {
      case 'turn_start': {
        set((state) => ({
          status: 'thinking',
          currentTurn: {
            ...state.currentTurn,
            turnId: event.turn_id,
            turnIndex: event.turn_index,
          },
        }));
        break;
      }
      case 'mj_thinking': {
        const isStreaming = event.status === 'narrating';
        const isCallingTools = event.status === 'calling_tools';
        set((state) => ({
          status: isStreaming ? 'streaming' : isCallingTools ? 'calling_tools' : 'thinking',
          currentTurn: {
            ...state.currentTurn,
            thinkingStatus: event.status,
            thinkingSummary: event.summary || state.currentTurn.thinkingSummary,
          },
        }));
        break;
      }
      case 'state_mutation': {
        const mutation: StateMutation = {
          type: event.type,
          payload: event.payload,
        };

        // Update active world entities reactively
        if (event.type === 'relationship_update') {
          const { npc_id, affinity, trust, mood } = event.payload;
          set((state) => ({
            npcs: state.npcs.map((npc) =>
              npc.id === npc_id
                ? {
                    ...npc,
                    relationship: {
                      ...npc.relationship,
                      affinity,
                      trust,
                      mood,
                    },
                  }
                : npc
            ),
          }));
        } else if (event.type === 'location_change') {
          const { location_id } = event.payload;
          set({ currentLocationId: location_id });
        }

        set((state) => ({
          currentTurn: {
            ...state.currentTurn,
            mutations: [...state.currentTurn.mutations, mutation],
          },
        }));
        break;
      }
      case 'narration_chunk': {
        set((state) => ({
          status: 'streaming',
          currentTurn: {
            ...state.currentTurn,
            rawNarration: state.currentTurn.rawNarration + event.chunk,
          },
        }));
        break;
      }
      case 'turn_complete': {
        const { currentTurn, turns, campaign } = get();
        const parsedBlocks = parseRPStream(currentTurn.rawNarration);

        const newTurnItem: TurnHistoryItem = {
          turnId: event.turn_id,
          turnNumber: currentTurn.turnIndex,
          playerInput: currentTurn.playerInput,
          rawNarration: currentTurn.rawNarration,
          blocks: parsedBlocks,
          mutations: currentTurn.mutations,
          turnSummary: event.turn_summary,
          mjReasoning: event.mj_reasoning || event.turn_summary,
          mjBriefing: event.mj_briefing,
          toolCalls: event.tool_calls,
          createdAt: new Date().toISOString(),
        };

        set({
          status: 'idle',
          turns: [...turns, newTurnItem],
          campaign: {
            ...campaign,
            turn_count: currentTurn.turnIndex,
            current_location_id: event.current_location_id || campaign.current_location_id,
          },
          currentLocationId: event.current_location_id || get().currentLocationId,
          currentTurn: initialCurrentTurn,
        });
        break;
      }
      case 'error': {
        set({
          status: 'error',
          error: event.message,
        });
        break;
      }
    }
  },

  setError: (error: string | null) => set({ error, status: error ? 'error' : 'idle' }),

  abortTurn: () => {
    set({
      status: 'idle',
      error: 'Tour interrompu par le joueur.',
    });
  },
}));
