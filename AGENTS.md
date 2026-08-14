# JanusRP Agents Guide

## Git & Version Control (Obligatoire)

- **Toujours commiter et pusher** : Tout fichier créé ou modifié (ADR dans `docs/adr/`, `CONTEXT.md`, code source, documentation) lors du traitement ou de la résolution d'un ticket **DOIT être obligatoirement commité et pushé sur Git (`git add`, `git commit`, `git push`) AVANT de clôturer l'issue ou de terminer la session**.
- **Zéro fichier non-commité en fin de ticket** : Vérifier systématiquement `git status` pour s'assurer que l'arbre de travail est propre et synchronisé avec le dépôt distant.

## Agent skills

### Issue tracker

GitHub Issues via `gh` CLI. Voir `docs/agents/issue-tracker.md`.

### Domain docs

Single-context (`CONTEXT.md` + `docs/adr/`). Voir `docs/agents/domain.md`.
