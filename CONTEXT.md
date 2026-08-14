# JanusRP — Ubiquitous Language & Domain Glossary

## Core Concepts

### Sandbox Narratif
Un environnement de jeu de rôle interactif piloté par l'intention du joueur, où l'histoire et les conséquences émergent dynamiquement sans rails rigides ni systèmes de règles mathématiques contraignants.

### Joueur (PJ - Personnage Joueur)
L'utilisateur humain qui interagit avec le monde en saisissant des intentions d'actions brutes, des dialogues ou des choix narratifs.

### PNJ (Personnage Non-Joueur)
Entité vivante du monde dotée d'une identité, d'un profil psychologique, de relations (affinité, statut émotionnel, rancunes), de secrets et de souvenirs persistants.

### Le Maître du Jeu (MJ / Décisionnaire)
L'agent d'arbitrage logique et contextuel (Meta Muse Glimmer). Il interprète l'intention brute du joueur, interroge l'état du monde, prend les décisions d'arbitrage (succès, échecs, évolutions relationnelles), commande les mutations d'état et rédige le briefing narratif pour la Plume.

### La Plume (Le Narrateur / Incarnateur)
L'agent de rédaction littéraire et stylistique (Qwen). Il transforme les directives narratives et les décisions d'arbitrage du MJ en descriptions immersives, sensorielles et en dialogues de PNJ fidèles à leur ton.

### Boucle de Tour (*Turn Lifecycle*)
La séquence complète déclenchée par une entrée du joueur : réception de l'intention -> consultation de l'état (MCP) -> arbitrage & décision par le MJ -> mutation d'état (MCP) -> briefing narratif -> génération du récit par la Plume -> streaming vers le joueur.

### Outils MCP (*Model Context Protocol*)
L'ensemble des fonctions exposées par le backend pour permettre au MJ d'interroger et de modifier l'état persistant du monde (lieux, PNJ, inventaires, relations, mémoire) de manière structurée et sans hallucination.

### Graphe du Monde (*World Graph*)
La structure relationnelle représentant les lieux (nœuds), leurs liaisons topologiques (arêtes), ainsi que la présence actuelle des PNJ et objets dans ces lieux.

### Jauges Relationnelles (*Relationship Gauges*)
Les métriques numériques bornées (`Affinité`, `Confiance`, sur une échelle de -100 à +100) et qualitatives (`Humeur`) mesurant la disposition psychologique et l'attitude d'un PNJ envers le PJ.

### Événement Narratif Persistant (*Narrative Event Log*)
Fait saillant immuable consigné à l'issue de chaque tour notable, servant d'ancrage chronologique et de base d'indexation pour la mémoire épisodique du monde.

### Briefing Narratif (*Director Briefing*)
Consigne scénaristique et stylistique explicite produite par le MJ (Muse Glimmer) à l'attention exclusive de la Plume (Qwen), définissant le ton, les réactions sensorielles et les dialogues à générer sans réévaluer les règles.

