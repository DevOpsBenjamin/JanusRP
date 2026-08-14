# JanusRP Agents Guide

## Git & PR Workflow (Obligatoire)

- **Une branche par ticket / tâche** : Pour chaque ticket traité ou tâche, créer systématiquement une branche dédiée (ex: `feat/issue-<n>-<slug>`, `docs/issue-<n>-<slug>`, ou `chore/<slug>`). Ne jamais commiter directement sur `main`.
- **Commit & Push sur la branche** : Commiter tous les fichiers créés/modifiés (ADR dans `docs/adr/`, `CONTEXT.md`, code source, documentation) avec un message conventionnel clair et pusher la branche (`git push origin <branch>`).
- **Création de PR et Merge** :
  1. Ouvrir une Pull Request via la CLI `gh` : `gh pr create --title "<type>: resolve issue #<n> - <title>" --body "Closes #<n>\n\n<résumé>"`.
  2. Procéder au merge automatique : `gh pr merge` (le squash et la suppression de branche étant configurés par défaut sur le repo).
  3. Revenir sur `main` et mettre à jour l'arbre local : `git checkout main && git pull origin main`.
- **Zéro travail résiduel non-fusionné** : Vérifier systématiquement `git status` pour s'assurer que le workspace est propre, aligné sur `main` à jour et sans branche locale orpheline.

## Agent skills

### Issue tracker

GitHub Issues & Pull Requests via `gh` CLI. Voir `docs/agents/issue-tracker.md`.

### Domain docs

Single-context (`CONTEXT.md` + `docs/adr/`). Voir `docs/agents/domain.md`.
