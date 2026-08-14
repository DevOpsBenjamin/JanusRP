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

export interface NpcRelationship {
  affinity: number;
  trust: number;
  mood: string;
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
}

export type StreamStatus = 'idle' | 'thinking' | 'streaming' | 'complete' | 'error';
