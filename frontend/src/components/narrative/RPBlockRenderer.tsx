import React from 'react';
import { RPBlock } from '../../types';
import { DialogueBlock } from './DialogueBlock';
import { ThoughtBlock } from './ThoughtBlock';
import { CommBlock } from './CommBlock';
import { SensoryBlock } from './SensoryBlock';
import { DocumentBlock } from './DocumentBlock';
import { IllustrationBlock } from './IllustrationBlock';

interface RPBlockRendererProps {
  block: RPBlock;
}

export const RPBlockRenderer: React.FC<RPBlockRendererProps> = ({ block }) => {
  switch (block.type) {
    case 'dialogue':
      return (
        <DialogueBlock
          speaker={block.speaker}
          mood={block.mood}
          tone={block.tone}
          content={block.content}
        />
      );
    case 'thought':
      return (
        <ThoughtBlock
          speaker={block.speaker}
          visibility={block.visibility}
          content={block.content}
        />
      );
    case 'comm':
      return (
        <CommBlock
          commType={block.commType}
          from={block.from}
          to={block.to}
          app={block.app}
          time={block.time}
          content={block.content}
        />
      );
    case 'sensory':
      return (
        <SensoryBlock
          sensoryType={block.sensoryType}
          content={block.content}
        />
      );
    case 'document':
      return (
        <DocumentBlock
          title={block.title}
          content={block.content}
        />
      );
    case 'illustration':
      return <IllustrationBlock prompt={block.prompt} />;
    case 'narrative':
    default:
      return (
        <p className="my-2.5 font-serif text-sm leading-relaxed text-slate-300 whitespace-pre-wrap">
          {block.content}
        </p>
      );
  }
};
