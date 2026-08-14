# Domain Documentation

## Layout

**Layout: single-context**

This repo uses a single root-level domain context:

- `CONTEXT.md` at repo root — Ubiquitous language glossary
- `docs/adr/` — Architecture Decision Records

## Consumer rules

- Read `CONTEXT.md` before designing or implementing domain logic.
- Use exact terms defined in `CONTEXT.md`.
- Propose additions to `CONTEXT.md` when new domain concepts are introduced.
- **Commit and push** `CONTEXT.md` and any newly created/modified ADRs in `docs/adr/` immediately (`git add -A && git commit -m "..." && git push`). Never leave ADRs or domain documentation uncommitted.
