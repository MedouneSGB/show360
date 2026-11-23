# LiveKit Cloud (dual-mode)

Même API et mêmes clients que le self-host. Seule la config LiveKit change.

## Usage

```bash
cp deploy/cloud/.env.cloud.example deploy/cloud/.env.cloud
# éditer URL + clés projet LiveKit Cloud

set -a && source deploy/cloud/.env.cloud && set +a
cargo run -p show360-api
```

Ne lancez **pas** le service `livekit` du `docker-compose` en mode cloud.
MinIO / Redis restent optionnels selon les besoins P1 (enregistrement).

## Ce que voient les clients

- `livekit_url` (wss) renvoyée dans `/health` et dans les réponses token
- **jamais** `SHOW360_LIVEKIT_API_KEY` / `SHOW360_LIVEKIT_API_SECRET`
