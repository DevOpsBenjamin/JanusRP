import type { RPBlock } from '../types/index.ts';

interface TagAttributes {
  [key: string]: string;
}

function parseAttributes(attrString: string): TagAttributes {
  const attrs: TagAttributes = {};
  const attrRegex = /([a-zA-Z0-9_-]+)=(?:"([^"]*)"|'([^']*)'|([^\s>]+))/g;
  let match: RegExpExecArray | null;

  while ((match = attrRegex.exec(attrString)) !== null) {
    const key = match[1];
    const val = match[2] ?? match[3] ?? match[4] ?? '';
    attrs[key] = val;
  }

  return attrs;
}

/**
 * Tolerant incremental parser for narrative DSL tags in streaming text.
 * Implements ADR-0004 specifications:
 * - Open tag tolerance (renders partial chunks inside open tag immediately)
 * - Implicit auto-closing upon new block tag
 * - Orphan text wrapped into <narrative>
 * - Self-closing tag support (<illustration prompt="..." />)
 */
export function parseRPStream(raw: string): RPBlock[] {
  if (!raw) return [];

  const blocks: RPBlock[] = [];
  const tagRegex = /<(\/?)([a-zA-Z0-9_-]+)([^>]*?)(\/?)>/g;

  let currentBlockType: string | null = null;
  let currentBlockAttrs: TagAttributes = {};
  let currentContent = '';
  let lastIndex = 0;

  let match: RegExpExecArray | null;

  const pushCurrentBlock = () => {
    if (!currentBlockType) {
      const trimmed = currentContent.trim();
      if (trimmed) {
        blocks.push({
          type: 'narrative',
          content: trimmed,
        });
      }
      currentContent = '';
      return;
    }

    const trimmed = currentContent.trim();
    switch (currentBlockType) {
      case 'narrative':
        if (trimmed) {
          blocks.push({
            type: 'narrative',
            content: trimmed,
          });
        }
        break;
      case 'dialogue':
        blocks.push({
          type: 'dialogue',
          speaker: currentBlockAttrs.speaker || 'Inconnu',
          mood: currentBlockAttrs.mood,
          tone: currentBlockAttrs.tone,
          content: trimmed,
        });
        break;
      case 'thought':
        blocks.push({
          type: 'thought',
          speaker: currentBlockAttrs.speaker,
          visibility: (currentBlockAttrs.visibility as 'hidden' | 'visible') || 'hidden',
          content: trimmed,
        });
        break;
      case 'comm':
        blocks.push({
          type: 'comm',
          commType: currentBlockAttrs.type || 'message',
          from: currentBlockAttrs.from,
          to: currentBlockAttrs.to,
          app: currentBlockAttrs.app,
          time: currentBlockAttrs.time,
          content: trimmed,
        });
        break;
      case 'sensory':
        blocks.push({
          type: 'sensory',
          sensoryType: currentBlockAttrs.type || 'sound',
          content: trimmed,
        });
        break;
      case 'document':
        blocks.push({
          type: 'document',
          title: currentBlockAttrs.title || 'Document',
          content: trimmed,
        });
        break;
      case 'illustration':
        blocks.push({
          type: 'illustration',
          prompt: currentBlockAttrs.prompt || trimmed,
        });
        break;
      default:
        // Unknown tag: wrap content gracefully into narrative
        if (trimmed) {
          blocks.push({
            type: 'narrative',
            content: trimmed,
          });
        }
        break;
    }

    currentBlockType = null;
    currentBlockAttrs = {};
    currentContent = '';
  };

  while ((match = tagRegex.exec(raw)) !== null) {
    const isClosing = match[1] === '/';
    const tagName = match[2].toLowerCase();
    const rawAttrs = match[3] || '';
    const isSelfClosing = match[4] === '/';
    const tagStartIndex = match.index;

    // Text between previous tag end and current tag start
    const betweenText = raw.substring(lastIndex, tagStartIndex);
    currentContent += betweenText;

    if (isClosing) {
      // Closing tag: closes current block if it matches or if any was open
      pushCurrentBlock();
    } else if (isSelfClosing) {
      // Close previous block first
      pushCurrentBlock();
      const attrs = parseAttributes(rawAttrs);
      if (tagName === 'illustration') {
        blocks.push({
          type: 'illustration',
          prompt: attrs.prompt || '',
        });
      }
    } else {
      // Opening tag: auto-close any previously open block (implicit closing)
      pushCurrentBlock();
      currentBlockType = tagName;
      currentBlockAttrs = parseAttributes(rawAttrs);
    }

    lastIndex = tagRegex.lastIndex;
  }

  // Trailing text after the last tag
  const remainingText = raw.substring(lastIndex);
  currentContent += remainingText;

  // If there's an open block or remaining text at the end of the stream, push it
  if (currentBlockType || currentContent.trim()) {
    pushCurrentBlock();
  }

  // If after all parsing no blocks were produced but text exists, wrap in narrative
  if (blocks.length === 0 && raw.trim()) {
    blocks.push({
      type: 'narrative',
      content: raw.trim(),
    });
  }

  return blocks;
}
