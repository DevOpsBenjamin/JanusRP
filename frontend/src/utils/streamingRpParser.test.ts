import test from 'node:test';
import assert from 'node:assert/strict';
import { parseRPStream } from './streamingRpParser.ts';

test('parseRPStream wraps orphan text in narrative block', () => {
  const blocks = parseRPStream('Le vent souffle sur les collines du Val-Corbeau.');
  assert.equal(blocks.length, 1);
  assert.equal(blocks[0].type, 'narrative');
  assert.equal(blocks[0].content, 'Le vent souffle sur les collines du Val-Corbeau.');
});

test('parseRPStream parses dialogue with speaker and mood', () => {
  const raw = `<dialogue speaker="Elena" mood="accueillante" tone="chaleureux">
« Approchez-vous du feu voyageur. »
</dialogue>`;

  const blocks = parseRPStream(raw);
  assert.equal(blocks.length, 1);
  assert.equal(blocks[0].type, 'dialogue');
  if (blocks[0].type === 'dialogue') {
    assert.equal(blocks[0].speaker, 'Elena');
    assert.equal(blocks[0].mood, 'accueillante');
    assert.equal(blocks[0].tone, 'chaleureux');
    assert.equal(blocks[0].content, '« Approchez-vous du feu voyageur. »');
  }
});

test('parseRPStream parses thought block with hidden visibility', () => {
  const raw = `<thought speaker="Elena" visibility="hidden">
Elle jette un regard vers la trappe sous le comptoir.
</thought>`;

  const blocks = parseRPStream(raw);
  assert.equal(blocks.length, 1);
  assert.equal(blocks[0].type, 'thought');
  if (blocks[0].type === 'thought') {
    assert.equal(blocks[0].speaker, 'Elena');
    assert.equal(blocks[0].visibility, 'hidden');
    assert.equal(blocks[0].content, 'Elle jette un regard vers la trappe sous le comptoir.');
  }
});

test('parseRPStream parses comm block', () => {
  const raw = `<comm type="sms" from="Contact" to="Aventurier" app="Signal" time="23:14">
Attention, ils approchent.
</comm>`;

  const blocks = parseRPStream(raw);
  assert.equal(blocks.length, 1);
  assert.equal(blocks[0].type, 'comm');
  if (blocks[0].type === 'comm') {
    assert.equal(blocks[0].commType, 'sms');
    assert.equal(blocks[0].from, 'Contact');
    assert.equal(blocks[0].to, 'Aventurier');
    assert.equal(blocks[0].app, 'Signal');
    assert.equal(blocks[0].time, '23:14');
    assert.equal(blocks[0].content, 'Attention, ils approchent.');
  }
});

test('parseRPStream parses sensory block', () => {
  const raw = `<sensory type="sound">
Un craquement sinistre résonne à l'étage.
</sensory>`;

  const blocks = parseRPStream(raw);
  assert.equal(blocks.length, 1);
  assert.equal(blocks[0].type, 'sensory');
  if (blocks[0].type === 'sensory') {
    assert.equal(blocks[0].sensoryType, 'sound');
    assert.equal(blocks[0].content, "Un craquement sinistre résonne à l'étage.");
  }
});

test('parseRPStream parses document block', () => {
  const raw = `<document title="Lettre froissée">
Rendez-vous à l'orée du bois à minuit.
</document>`;

  const blocks = parseRPStream(raw);
  assert.equal(blocks.length, 1);
  assert.equal(blocks[0].type, 'document');
  if (blocks[0].type === 'document') {
    assert.equal(blocks[0].title, 'Lettre froissée');
    assert.equal(blocks[0].content, "Rendez-vous à l'orée du bois à minuit.");
  }
});

test('parseRPStream parses self-closing illustration block', () => {
  const raw = `<illustration prompt="Une silhouette dans la brume" />`;

  const blocks = parseRPStream(raw);
  assert.equal(blocks.length, 1);
  assert.equal(blocks[0].type, 'illustration');
  if (blocks[0].type === 'illustration') {
    assert.equal(blocks[0].prompt, 'Une silhouette dans la brume');
  }
});

test('parseRPStream handles open tag tolerance (stream ends before closing tag)', () => {
  const partialStream = `<dialogue speaker="Elena" mood="troublée">
« Je ne devrais pas vous dire cela mais...`;

  const blocks = parseRPStream(partialStream);
  assert.equal(blocks.length, 1);
  assert.equal(blocks[0].type, 'dialogue');
  if (blocks[0].type === 'dialogue') {
    assert.equal(blocks[0].speaker, 'Elena');
    assert.equal(blocks[0].content, '« Je ne devrais pas vous dire cela mais...');
  }
});

test('parseRPStream handles implicit auto-closing on new block tag', () => {
  const stream = `<narrative>
La nuit est tombée.
<dialogue speaker="Elena">
« Qui est là ? »
<thought speaker="Elena">
Elle a peur.`;

  const blocks = parseRPStream(stream);
  assert.equal(blocks.length, 3);
  assert.equal(blocks[0].type, 'narrative');
  assert.equal(blocks[0].content, 'La nuit est tombée.');
  assert.equal(blocks[1].type, 'dialogue');
  if (blocks[1].type === 'dialogue') {
    assert.equal(blocks[1].content, '« Qui est là ? »');
  }
  assert.equal(blocks[2].type, 'thought');
  if (blocks[2].type === 'thought') {
    assert.equal(blocks[2].content, 'Elle a peur.');
  }
});

test('parseRPStream handles empty input gracefully', () => {
  assert.deepEqual(parseRPStream(''), []);
  assert.deepEqual(parseRPStream('   \n\t  '), []);
});
