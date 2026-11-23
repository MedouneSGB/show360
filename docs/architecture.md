# Architecture Show360 (scaffold)

Voir le plan produit validé pour le détail. Résumé figé dans ce dépôt :

```
PC (Tauri 2) ──┐
               ├── HTTPS/WSS → API Axum → tokens LiveKit
Web (Dioxus) ──┘                      │
                                      ▼
                              LiveKit (self | cloud)
```

## Crates

- **show360-core** — domaine (salle, rôles, tracks, session), zéro UI
- **show360-media** — JWT LiveKit + mapping rôle → grants (screen / 360°)
- **show360-api** — HTTP ; secrets SFU serveur uniquement
- **show360-ui** — Dioxus partagé (placeholder)

## Présentateur (P0)

Un seul `presenter_id` par salle. Permissions screen share / projection 360 via `Role` + `MediaGrants`. UI spotlight : lots clients suivants.
