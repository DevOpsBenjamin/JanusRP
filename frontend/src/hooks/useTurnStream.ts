import { useCallback, useRef } from 'react';
import { createParser, ParseEvent } from 'eventsource-parser';
import { useGameStore } from '../store';
import { TurnStreamEvent } from '../types';

export function useTurnStream() {
  const { campaign, status, startTurn, handleTurnStreamEvent, setError, abortTurn: storeAbortTurn } =
    useGameStore();
  const abortControllerRef = useRef<AbortController | null>(null);

  const isStreaming = status === 'thinking' || status === 'calling_tools' || status === 'streaming';

  const abort = useCallback(() => {
    if (abortControllerRef.current) {
      abortControllerRef.current.abort();
      abortControllerRef.current = null;
    }
    storeAbortTurn();
  }, [storeAbortTurn]);

  const submitTurn = useCallback(
    async (playerInput: string) => {
      const trimmed = playerInput.trim();
      if (!trimmed || isStreaming) return;

      // Initialize turn in store
      startTurn(trimmed);

      const controller = new AbortController();
      abortControllerRef.current = controller;

      try {
        const response = await fetch(`/api/campaigns/${campaign.id}/turns`, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            Accept: 'text/event-stream',
          },
          body: JSON.stringify({ player_input: trimmed }),
          signal: controller.signal,
        });

        if (!response.ok) {
          let errorMsg = `Erreur HTTP ${response.status} (${response.statusText})`;
          try {
            const errData = await response.json();
            if (errData.error) errorMsg = errData.error;
          } catch {
            // non-json error
          }
          setError(errorMsg);
          return;
        }

        if (!response.body) {
          setError('Aucun flux de données SSE reçu du serveur.');
          return;
        }

        const reader = response.body.getReader();
        const decoder = new TextDecoder();

        const parser = createParser((event: ParseEvent) => {
          if (event.type === 'event') {
            try {
              const data = JSON.parse(event.data);
              const eventName = event.event || data.event;

              // Reconstruct TurnStreamEvent
              let streamEvent: TurnStreamEvent | null = null;

              if (eventName === 'turn_start') {
                streamEvent = {
                  event: 'turn_start',
                  turn_id: data.turn_id,
                  campaign_id: data.campaign_id,
                  turn_index: data.turn_index,
                };
              } else if (eventName === 'mj_thinking') {
                streamEvent = {
                  event: 'mj_thinking',
                  status: data.status,
                  summary: data.summary,
                };
              } else if (eventName === 'state_mutation') {
                streamEvent = {
                  event: 'state_mutation',
                  type: data.type,
                  payload: data.payload,
                };
              } else if (eventName === 'narration_chunk') {
                streamEvent = {
                  event: 'narration_chunk',
                  chunk: data.chunk,
                };
              } else if (eventName === 'turn_complete') {
                streamEvent = {
                  event: 'turn_complete',
                  turn_id: data.turn_id,
                  current_location_id: data.current_location_id || data.current_location,
                  turn_summary: data.turn_summary,
                };
              } else if (eventName === 'error') {
                streamEvent = {
                  event: 'error',
                  code: data.code,
                  message: data.message,
                  retryable: data.retryable ?? false,
                };
              }

              if (streamEvent) {
                handleTurnStreamEvent(streamEvent);
              }
            } catch (err) {
              console.warn('Failed to parse SSE JSON payload:', event.data, err);
            }
          }
        });

        while (true) {
          const { done, value } = await reader.read();
          if (done) break;
          parser.feed(decoder.decode(value, { stream: true }));
        }
      } catch (err: any) {
        if (err.name === 'AbortError') {
          console.info('Turn stream aborted by user');
        } else {
          console.error('Turn stream network failure:', err);
          setError(err.message || 'Erreur réseau lors de la communication avec le serveur.');
        }
      } finally {
        abortControllerRef.current = null;
      }
    },
    [campaign.id, isStreaming, startTurn, handleTurnStreamEvent, setError]
  );

  return {
    submitTurn,
    abort,
    isStreaming,
  };
}
