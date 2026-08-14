# Spécifications Fonctionnelles : RP Agent Manager (Glimmer x Qwen)

## 1. Vision du Projet

Le projet a pour but de créer un **Gestionnaire de Jeu de Rôle (JDR) textuel autonome, immersif et cohérent**. 

Contrairement aux chatbots RP classiques qui finissent par halluciner des objets, oublier des règles ou perdre le fil des événements, ce système sépare strictement :
1. **La Logique & l'Arbitrage du Monde** (garanties par le système et arbitrées par un modèle logique).
2. **La Narration Littéraire & l'Incarnation des PNJ** (déléguées à un modèle créatif débridé).

---

## 2. Rôles et Responsabilités des Agents

### A. Muse Glimmer (Le Maître du Jeu / L'Orchestrateur & Juge)
* **Rôle :** Maître du Jeu (MJ), arbitre des règles et contrôleur qualité.
* **Missions fonctionnelles :**
  * **Compréhension de l'intention :** Analyse les actions et dialogues du joueur pour en déduire les actions mécaniques requises.
  * **Interactions avec le monde (via MCP) :** Consulte les fiches de personnages, déclenche les lancers de dés, vérifie les prérequis d'inventaire et met à jour l'état de la partie.
  * **Briefing narratif :** Rédige des consignes précises et sans ambiguïté pour le modèle narrateur.
  * **Validation "LLM-as-a-Judge" :** Relit la prose générée avant affichage pour s'assurer de l'absence d'hallucinations factuelles (respect des PV, objets réels, lore, état des PNJ).

### B. Qwen 3.8 (La Plume / Le Narrateur & Acteur)
* **Rôle :** Auteur de la fiction, conteur d'ambiance et voix des PNJ.
* **Missions fonctionnelles :**
  * **Narration immersive :** Transforme les résultats techniques bruts du MJ en descriptions viscérales, sensorielles et vivantes.
  * **Incarnation des PNJ :** Adopte le ton, le niveau de langue et la personnalité de chaque personnage rencontré.
  * **Liberté créative débridée :** Décrit les scènes de combat, les moments de tension, l'horreur ou les dialogues matures sans blocages moralisateurs.

---

## 3. Les Fonctionnalités Clés du Système

### 1. La Boucle de Gameplay Principale (*Turn Lifecycle*)
1. **Entrée Joueur :** Le joueur saisit une action libre (ex: *"Je tente d'amadouer le garde en lui glissant une bourse de pièces"*).
2. **Évaluation Mécanique :** Glimmer vérifie si le joueur possède les pièces requises et demande un test d'éloquence.
3. **Résolution Déterministe :** Le moteur calcule le résultat (Succès critique / Succès / Échec / Échec critique).
4. **Mise à jour du Monde :** L'état du jeu est immédiatement modifié (déduction des pièces, changement d'attitude du garde).
5. **Génération Littéraire :** Qwen rédige la scène selon le résultat exact.
6. **Contrôle Qualité :** Glimmer valide le texte et l'affiche au joueur.

---

### 2. Le Hub MCP (Model Context Protocol) — Vue Fonctionnelle

Le protocole MCP sert de passerelle entre le Maître du Jeu (Glimmer) et le moteur de jeu :

* **Module Règles & Hasard :**
  * Calcul de tests de compétences (avec modificateurs, avantage/désavantage).
  * Résolution des tours de combat (calcul des dégâts, statuts, initiative).
* **Module Personnages & Inventaire :**
  * Fiches complètes des PJ et PNJ (caractéristiques, traits de personnalité, secrets).
  * Inventaire dynamique avec poids, encombrement et état des objets.
* **Module Monde & Environnement :**
  * Suivi du lieu actuel, de la météo, de l'heure et du temps qui passe.
  * État persistant des lieux (ex: une porte défoncée reste défoncée).
* **Module Quêtes & Événements :**
  * Arbre des objectifs principaux et secondaires.
  * Journal d'événements marquants pour alimenter la mémoire à long terme.
* **Module Encyclopédie du Monde (Lore) :**
  * Base documentaire sur les factions, la géographie, l'histoire et les monstres de l'univers.

---

### 3. Le Filtre Anti-Hallucination ("LLM-as-a-Judge")

Pour garantir une immersion sans faille, Glimmer applique une grille de relecture systématique :

```text
Grille d'évaluation de Glimmer :
[✓] Règle 1 : Les conséquences mécaniques sont-elles fidèlement retranscrites ? (Pas de mort inventée si simple blessure).
[✓] Règle 2 : Les objets mentionnés existent-ils dans l'inventaire ou le décor ?
[✓] Règle 3 : Le comportement du PNJ est-il fidèle à son profil psychologique ?
[✓] Règle 4 : Le ton et l'ambiance respectent-ils l'univers choisi ?
```

Si un critère échoue, Glimmer renvoie une consigne d'ajustement au narrateur avant diffusion.

---

### 4. Expérience Utilisateur & Fonctionnalités Futures

* **Créateur de Campagne :** Possibilité de choisir un univers (Dark Fantasy, Cyberpunk, Lovecraftien, Space Opera) et d'injecter son propre lore.
* **Fiche de Personnage Interactive :** Visualisation en temps réel des PV, jauges de mana/fatigue, inventaire et réputation.
* **Journal de Bord Automatique :** Génération de résumés de session pour reprendre une partie sans perte de contexte.
* **Mode Scénario Libre ou Guidé :** Support de quêtes dirigées ou d'exploration 100% "bac à sable" (Sandbox).
