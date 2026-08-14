# ADR 0004 : Architecture Frontend React, Visualisation ReactFlow et Moteur Narratif Balisé (DSL XML)

## Statut
Accepté

## Date
2026-08-14

## Contexte
L'interface utilisateur de JanusRP doit restituer simultanément et en temps réel :
1. **La spatialité du monde** : Un graphe interactif des lieux et des déplacements (ReactFlow) mis à jour instantanément lors des mutations de tour.
2. **L'immersion narrative riche** : Une console de jeu capable de restituer la prose littéraire de Qwen 3.8 en streaming, mais avec une mise en forme dépassant les limitations du Markdown standard (dialogues avec avatars, pensées intimes révélables, interfaces de communication diégétiques comme les SMS/radios/terminaux, documents trouvés, et illustrations de scène générées).
3. **L'état social et contextuel** : Un inspecteur réactif affichant les détails du lieu actif et les fiches des PNJ présents (jauges d'Affinité et de Confiance animées de -100 à +100).
4. **La résilience au streaming** : La consommation asynchrone d'un endpoint unique `POST /api/campaigns/:id/turns` en `text/event-stream` (ADR 0003) avec capacité d'interruption par le joueur (`AbortController`).

## Décision

### 1. Organisation Spatiale (Layout 3 Panneaux)
L'interface de jeu adopte une disposition à trois volets synchronisés :
- **Panneau Gauche (35%) — Graphe Spatial (ReactFlow)** : Visualisation de la carte topologique, zoom/pan fluide, centrage automatique (`setCenter`) sur le lieu actif lors d'une mutation `location_change`, badge visuel du PJ et avatars des PNJ présents.
- **Panneau Central (45%) — Console Narrative & Saisie** : Historique chronologique des tours, rendu en streaming des blocs narratifs balisés, indicateurs d'état du Maître du Jeu (`mj_thinking`), encarts de mutations discrètes (`state_mutation`), et champ de saisie d'action brute avec raccourcis clavier.
- **Panneau Droit (20% - Rétractable) — Inspecteur de Contexte** : Fiche immersive du lieu actuel (ambiance, description, secrets découverts) et liste détaillée des PNJ présents (portrait, humeur, jauges d'Affinité et de Confiance interactives, historique d'interactions).

### 2. DSL Narratif & Grammaire des Balises RP (Qwen 3.8)
Pour structurer la mise en scène sans contraindre la liberté stylistique de la Plume, le prompt système et le briefing de Qwen instruisent l'utilisation d'un flux séquentiel plat (*flat chunks*) de balises sémantiques :

```xml
<narrative>
La pluie bat les carreaux de l'atelier, saturant l'air d'une odeur d'ozone et de métal froid.
</narrative>

<dialogue speaker="Elena" mood="troubled" tone="hesitant">
« Tu n'aurais pas dû revenir ici ce soir... »
</dialogue>

<thought speaker="Elena" visibility="hidden">
Elle dissimule nerveusement un document scellé sous sa veste.
</thought>

<comm type="sms" from="Contact Inconnu" to="Joueur" app="Signal" time="22:15">
Ne reste pas là. Quelqu'un approche de l'atelier.
</comm>

<sensory type="sound">
Un grincement sec de parquet retentit dans l'antichambre.
</sensory>

<document title="Journal d'Elena - Page arrachée">
"Si ces lignes sont lues, c'est que l'Ordonnance a déjà franchi les portes..."
</document>

<illustration prompt="Une femme aux cheveux sombres cachant un parchemin sous une lampe à huile, ambiance noir et pluie" />
```

### 3. Moteur de Rendu Streaming Tolérant (`StreamingRPParser`)
Le composant de restitution textuelle intègre un parseur incrémental hautement tolérant :
- **Tolérance aux balises ouvertes** : Ouverture immédiate du composant React dès la détection de la balise ouvrante (ex: `<dialogue speaker="...">`), permettant un affichage progressif token-par-token sans attendre la balise fermante.
- **Enveloppement du texte orphelin** : Tout fragment de texte généré hors balise est automatiquement encapsulé dans un bloc `<narrative>`.
- **Auto-fermeture implicite** : Fermeture automatique de toute balise en cours lors de la détection d'une nouvelle balise de bloc ou à la fin du flux SSE.
- **Micro-Markdown inline** : Rendu des styles typographiques légers (`**gras**`, `*italique*`, listes) à l'intérieur des nœuds textuels.
- **Balises inconnues ou illustrations** : Rendu gracieux sous forme de cartes dédiées sans interruption du flux.

### 4. Composants React Dédiés
- `<DialogueBlock />` : Encart de dialogue avec nom du personnage, avatar cliquable (ouvrant la fiche PNJ), indicateur d'humeur et bouton de réponse rapide.
- `<ThoughtBlock />` : Encart à typographie éthérée/italique, avec état masqué/révélable (`visibility="hidden"` par défaut ou déverrouillé selon les aptitudes de perception).
- `<CommBlock />` : Interface de messagerie diégétique (smartphone, fréquence radio, téléscripteur, parchemin) avec métadonnées d'expéditeur et horodatage.
- `<SensoryBlock />` : Encart immersif soulignant un stimulus sensoriel clé (visuel, sonore, olfactif).
- `<DocumentBlock />` : Rendu de document interactif lisible et dépliable en plein écran.
- `<IllustrationBlock />` : Emplacement d'image générée ou référence visuelle intégrée au flux narratif.
- `<SystemMutationToast />` : Notification visuelle discrète pour les deltas de jauges (ex: *« Elena : +10 Affinité »*).

### 5. Gestion de l'État Client (Zustand) et Hook SSE (`useTurnStream`)
- **Store modulaire Zustand** :
  - `campaignSlice` : État global de la campagne et lieu actif.
  - `graphSlice` : Synchronisation des nœuds/arêtes ReactFlow avec les événements `state_mutation: location_change`.
  - `turnSlice` : Statut du tour (`idle`, `thinking`, `streaming`), accumulateur des blocs narratifs, historique séquentiel.
  - `inspectorSlice` : PNJ ou lieu sélectionné pour inspection approfondie.
- **Hook `useTurnStream`** :
  - Consommation de `POST /api/campaigns/:id/turns` via `fetch` + `ReadableStream` + `eventsource-parser`.
  - Intégration d'un `AbortController` pour l'interruption coopérative de tour par le joueur.

## Conséquences

### Positives
- **Immersion roleplay sans équivalent** : Richesse visuelle des dialogues, communications SMS/radio, documents et pensées bien supérieure au Markdown brut.
- **Résilience absolue au streaming** : Tolérance native aux coupures et aux imperfections syntaxiques des LLM.
- **Synchronisation dynamique** : Interaction fluide entre le clic sur la carte ReactFlow, la console de texte et l'inspecteur social.
- **Performances optimales** : Découpage granulaire avec sélecteurs Zustand évitant les re-renders globaux lors de l'arrivée des tokens.

### Négatives / Risques
- Nécessite d'exposer clairement la grammaire XML dans le prompt système de Qwen 3.8.
