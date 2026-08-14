# ADR 0005 : Architecture de l'Éditeur de Monde & Créateur de PNJ (ReactFlow, Roster & Synchronisation REST)

## Statut
Accepté

## Date
2026-08-14

## Contexte
JanusRP nécessite un outil auteur intuitif et réactif pour permettre la création, la personnalisation et l'extension d'univers de jeu (campagnes) sans saisie SQL manuelle :
1. **Construction topologique du monde** : Création visuelle de nœuds de lieux (`locations`), positionnement 2D libre sur grille, définition du lieu de départ du joueur et établissement de liaisons de déplacement (`location_edges`) avec gestion de verrous et descriptions de voyage.
2. **Peuplement et profilage social** : Conception détaillée de PNJ (`npcs`) avec traits de personnalité, agenda secret, jauges relationnelles initiales (`npc_relationships`) et affectation spatiale à un lieu via une interaction visuelle.
3. **Assistance à la création par IA** : Fonctions optionnelles d'accélération par LLM (Qwen / Muse Glimmer) pour générer automatiquement du lore, des descriptions sensorielles et des intrigues cohérentes.
4. **Cohérence transactionnelle & ergonomie** : Sauvegarde instantanée sans friction, résilience aux erreurs et séparation claire entre le mode jeu immersif et le mode édition créative.

## Décision

### 1. Organisation de l'Écran & Routing Dédié (`/campaigns/:id/editor`)
L'éditeur adopte une route dédiée pour offrir un espace cartographique maximisé, tout en permettant un basculement immédiat vers la console de jeu (`/campaigns/:id/play`) :
- **Barre d'outils & Palette latérale gauche (20%)** :
  - Palette de création de lieux (glisser-déposer sur le canvas).
  - Roster global des PNJ de la campagne (recherche, statut de présence, création rapide et glisser-déposer d'affectation spatiale).
  - Outils de navigation (Zoom to fit, grille d'alignement, bouton « Basculer en mode Jeu »).
- **Canvas Cartographique Central (55%)** :
  - Espace ReactFlow étendu avec contrôles de zoom/pan, mini-carte et grille d'ancrage (*snap to grid* 20px).
  - Nœuds personnalisés `<EditableLocationNode />` affichant le titre, le type, le badge « Départ PJ » et les avatars des PNJ stationnés avec zone de largage (*drop target*).
  - Arêtes interactives `<EditableEdge />` cliquables avec étiquettes de verrouillage et de transit.
  - Tirage de poignées (*Handles*) magnétiques pour connecter les lieux.
- **Inspecteur de Propriétés Droit (25% - Tiroir contextuel réactif)** :
  - **Sélection Lieu** : Formulaire d'édition (Nom, Slug auto-généré, Description, Ambiance, Secrets, Props JSON, Case « Lieu de départ du PJ »).
  - **Sélection Arête** : Paramètres de liaison (Bidirectionnel, Description de voyage, Verrouillé `is_locked`, Raison/Clé requise `lock_reason`).
  - **Sélection PNJ** : Fiche complète (Nom, Titre, Traits JSON, Background, Agenda secret, Jauges de départ Affinité/Confiance [-100..100], Humeur, Lieu assigné).

### 2. Interactions de Création & Manipulation Spatiale
- **Création de Lieu par Glisser-Déposer** : Déposer un gabarit de lieu depuis la palette gauche sur le canvas ReactFlow calcule automatiquement les coordonnées mondiales (`position_x`, `position_y`) et émet une requête `POST /api/campaigns/:id/locations`.
- **Création d'Arêtes par Handles ReactFlow** : Relier la poignée de sortie du Nœud A à la poignée d'entrée du Nœud B ouvre instantanément l'arête et envoie `POST /api/campaigns/:id/edges` (bidirectionnelle par défaut).
- **Affectation de PNJ par Glisser-Déposer** : Glisser un PNJ depuis le Roster latéral et le lâcher sur un `<EditableLocationNode />` met à jour `current_location_id` via `PATCH /api/campaigns/:id/npcs/:npc_id`.
- **Déplacement de Nœuds** : Le déplacement libre sur le canvas met à jour la position locale en temps réel et applique un *debounce* de 300 ms sur `onNodeDragStop` pour persister `position_x` et `position_y` via `PATCH /api/campaigns/:id/locations/:id/position`.

### 3. Assistance à la Création par IA (*AI Lore & NPC Generation*)
L'éditeur intègre des points d'assistance ciblés pour stimuler la créativité de l'auteur :
- **Bouton « ✨ Générer profil PNJ »** : À partir d'un archétype court (ex: *« Alchimiste paranoïaque »*), un appel à `POST /api/campaigns/:id/generate/npc` produit un JSON structuré complétant le nom, titre, traits, agenda secret et réplique type.
- **Bouton « ✨ Enrichir le lieu »** : À partir du nom du lieu, génère une description multisensorielle, une ambiance et des secrets dissimulés.
- **Bouton « ✨ Suggérer des rumeurs & connexions »** : Propose des liens narratifs ou des rivalités entre les PNJ présents dans le lieu.

### 4. Contrat d'API REST Granulaire Backend
L'éditeur s'appuie sur une suite d'endpoints REST sécurisés et atomiques dans le backend Rust/Axum :
- `GET /api/campaigns/:id/editor-state` : Récupération agrégée (campagne, tous les lieux, arêtes, PNJ et jauges) pour l'initialisation du store ReactFlow/Zustand.
- `POST /api/campaigns/:id/locations` & `PATCH /api/campaigns/:id/locations/:id` & `DELETE /api/campaigns/:id/locations/:id`
- `PATCH /api/campaigns/:id/locations/:id/position` (`{ position_x: f32, position_y: f32 }`)
- `POST /api/campaigns/:id/edges` & `PATCH /api/campaigns/:id/edges/:id` & `DELETE /api/campaigns/:id/edges/:id`
- `POST /api/campaigns/:id/npcs` & `PATCH /api/campaigns/:id/npcs/:id` & `DELETE /api/campaigns/:id/npcs/:id`
- `PATCH /api/campaigns/:id/set-start-location` (`{ location_id: UUID }`)
- `POST /api/campaigns/:id/generate/npc` & `POST /api/campaigns/:id/generate/location`

## Conséquences

### Positives
- **Expérience Auteur Exceptionnelle** : Fluidité maximale grâce au glisser-déposer, au placement visuel libre et aux raccourcis ReactFlow.
- **Zéro Perte de Données** : Persistance granulaire immédiate avec synchronisation optimiste côté client et debounce sur les translations.
- **Alignement Parfait avec le Modèle SQLx** : Chaque action UI correspond exactement aux contraintes et tables de persistance définies dans l'ADR 0002.
- **Créativité Démultipliée** : Les modules d'assistance IA accélèrent la création de mondes profonds sans imposer de rigidité.

### Négatives / Risques
- Nécessite d'implémenter les contrôles de confirmation de suppression (cascade sur les arêtes et réinitialisation de `current_location_id` pour les PNJ).
