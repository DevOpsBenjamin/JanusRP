export interface Campaign {
  id: string;
  title: string;
  description?: string;
  system_prompt_theme?: string;
  player_name: string;
  current_location_id?: string;
  turn_count: number;
}

export interface Location {
  id: string;
  campaign_id: string;
  slug: string;
  name: string;
  description: string;
  atmosphere?: string;
  secrets?: string;
  position_x: number;
  position_y: number;
}

export interface LocationEdge {
  id: string;
  campaign_id: string;
  source_location_id: string;
  target_location_id: string;
  is_locked: boolean;
  lock_reason?: string;
  travel_description?: string;
}

export interface NpcRelationship {
  affinity: number; // -100 to 100
  trust: number;    // -100 to 100
  mood: string;
  last_interaction_turn?: number;
  interaction_summary?: string;
}

export interface Npc {
  id: string;
  campaign_id: string;
  current_location_id?: string;
  slug: string;
  name: string;
  title?: string;
  relationship?: NpcRelationship;
  personality_traits: string[];
  background?: string;
  secret_agenda?: string;
}

// State Mutations from MCP Tools
export type StateMutation =
  | {
      type: 'location_change';
      payload: {
        location_id: string;
        location_name: string;
        narration_hint?: string;
      };
    }
  | {
      type: 'relationship_update';
      payload: {
        npc_id: string;
        npc_name: string;
        affinity: number;
        trust: number;
        mood: string;
        delta_affinity?: number;
        delta_trust?: number;
        reason: string;
      };
    }
  | {
      type: 'event_logged';
      payload: {
        event_id: string;
        summary: string;
        significance: string;
      };
    };

// Server-Sent Events Protocol
export type TurnStreamEvent =
  | {
      event: 'turn_start';
      turn_id: string;
      campaign_id: string;
      turn_index: number;
    }
  | {
      event: 'mj_thinking';
      status: 'arbitrating' | 'calling_tools' | 'narrating' | string;
      summary?: string;
    }
  | {
      event: 'state_mutation';
      type: 'location_change' | 'relationship_update' | 'event_logged';
      payload: any;
    }
  | {
      event: 'narration_chunk';
      chunk: string;
    }
  | {
      event: 'turn_complete';
      turn_id: string;
      current_location_id?: string;
      current_location?: string;
      turn_summary: string;
      mj_reasoning?: string;
      mj_briefing?: string;
      tool_calls?: any;
    }
  | {
      event: 'error';
      code: string;
      message: string;
      retryable: boolean;
    };

export type StreamStatus =
  | 'idle'
  | 'thinking'
  | 'calling_tools'
  | 'streaming'
  | 'complete'
  | 'error';

// Narrative DSL XML Typed Blocks
export type RPBlock =
  | {
      type: 'narrative';
      content: string;
    }
  | {
      type: 'dialogue';
      speaker: string;
      mood?: string;
      tone?: string;
      content: string;
    }
  | {
      type: 'thought';
      speaker?: string;
      visibility?: 'hidden' | 'visible';
      content: string;
    }
  | {
      type: 'comm';
      commType?: string;
      from?: string;
      to?: string;
      app?: string;
      time?: string;
      content: string;
    }
  | {
      type: 'sensory';
      sensoryType?: 'sound' | 'smell' | 'sight' | 'touch' | string;
      content: string;
    }
  | {
      type: 'document';
      title: string;
      content: string;
    }
  | {
      type: 'illustration';
      prompt: string;
    };

export interface TurnHistoryItem {
  turnId: string;
  turnNumber: number;
  playerInput: string;
  rawNarration: string;
  blocks: RPBlock[];
  mutations: StateMutation[];
  turnSummary?: string;
  mjReasoning?: string;
  mjBriefing?: string;
  toolCalls?: any[];
  createdAt?: string;
}
