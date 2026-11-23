# Show360

Plateforme de **visioconférence orientée projection** (écran, présentateur, trajectoire 360°) — stack Rust bout-en-bout, dual **self-hosted + LiveKit Cloud**.

> Scaffold monorepo (P0 fondation). Pas encore de 360° fonctionnel ni de SFU custom.

## Architecture (aperçu)

| Couche | Crate / app | Rôle |
|--------|-------------|------|
| Types métier | `crates/show360-core` | Room, Role (host/présentateur/participant/spectator), tracks, session |
| Médias | `crates/show360-media` | Tokens LiveKit, grants, config dual-mode |
| API | `crates/show360-api` | Axum — health, salles, join, token, présentateur |
| UI partagée | `crates/show360-ui` | Composants Dioxus (placeholder) |
| Web | `apps/web` | Client Dioxus WASM |
| Desktop | `apps/desktop` | Shell Tauri 2 |
| Deploy | `deploy/` | docker-compose LiveKit + overlays cloud |

Les **secrets LiveKit** (`API_KEY` / `API_SECRET`) restent côté API — jamais dans les clients.

## Prérequis

- Rust 1.100+ (`rustup`, voir `rust-toolchain.toml`)
- Docker + Docker Compose (stack LiveKit locale)
- Optionnel : [prerequisites Tauri 2](https://v2.tauri.app/start/prerequisites/) pour le desktop
- Optionnel : `wasm32-unknown-unknown` + `dx` / `dioxus-cli` pour le web

```bash
rustup target add wasm32-unknown-unknown
```

## Démarrage rapide (self-hosted)

### 1. Variables d'environnement

```bash
cp deploy/.env.self-hosted.example .env
set -a && source .env && set +a
```

### 2. LiveKit + Redis + MinIO (+ API containerisée)

```bash
docker compose -f deploy/docker-compose.yml up --build
```

### 3. Ou API en local (LiveKit via compose)

```bash
docker compose -f deploy/docker-compose.yml up redis livekit minio
cargo run -p show360-api
```

### 4. Vérifier

```bash
curl -s http://127.0.0.1:8080/health | jq
```

Créer une salle et rejoindre :

```bash
CREATE=$(curl -s -X POST http://127.0.0.1:8080/v1/rooms \
  -H 'content-type: application/json' \
  -d '{"name":"Demo","mode":"self_hosted"}')
echo "$CREATE" | jq
CODE=$(echo "$CREATE" | jq -r .room.code)

curl -s -X POST http://127.0.0.1:8080/v1/rooms/join \
  -H 'content-type: application/json' \
  -d "{\"display_name\":\"Alice\",\"room_code\":\"$CODE\"}" | jq
```

## Dual-mode LiveKit Cloud

```bash
cp deploy/cloud/.env.cloud.example deploy/cloud/.env.cloud
# renseigner SHOW360_LIVEKIT_URL / API_KEY / API_SECRET
set -a && source deploy/cloud/.env.cloud && set +a
cargo run -p show360-api
```

Détails : [`deploy/cloud/README.md`](deploy/cloud/README.md).

| Variable | Self-host (dev) | Cloud |
|----------|-----------------|-------|
| `SHOW360_LIVEKIT_MODE` | `self_hosted` | `cloud` |
| `SHOW360_LIVEKIT_URL` | `ws://127.0.0.1:7880` | `wss://….livekit.cloud` |
| `SHOW360_LIVEKIT_API_KEY` | `devkey` | clé projet |
| `SHOW360_LIVEKIT_API_SECRET` | `secret` | secret projet |

## Workspace Cargo

```bash
cargo check          # membres par défaut (sans desktop Tauri)
cargo test -p show360-core -p show360-media
cargo run -p show360-api
```

### Web (Dioxus)

```bash
cargo check -p show360-web
# avec dioxus-cli : cd apps/web && dx serve
```

### Desktop (Tauri 2)

Voir [`apps/desktop/README.md`](apps/desktop/README.md). Exclu du `cargo check` par défaut (dépendances système WebKit/GTK).

## API stub (P0)

| Méthode | Chemin | Description |
|---------|--------|-------------|
| `GET` | `/health` | Santé + mode LiveKit (sans secrets) |
| `POST` | `/v1/rooms` | Créer une salle |
| `POST` | `/v1/rooms/join` | Rejoindre + session + token LiveKit |
| `GET` | `/v1/rooms/{id}` | Résumé salle |
| `POST` | `/v1/rooms/{id}/token` | Réémettre un token |
| `POST` | `/v1/rooms/{id}/presenter` | Transférer le présentateur |

Rôles dans `show360-core` : `host`, `presenter`, `participant`, `spectator` — prêts pour A/V, screen share et projection 360° (flag grants, rendu P2).

## Hors scaffold

- Visioconférence WebRTC réelle / SDK LiveKit client
- Auth OIDC complète
- Rendu 360°
- SFU custom

## Docs

- Contexte & décisions : dépôt `docs/` + plan produit Show360 (store projet)
- Plan : cadrage ambitieux validé — pas de réduction « MVP minimal »
