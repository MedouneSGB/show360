# Show360 Desktop (Tauri 2)

Shell natif PC. L’UI cible est **Dioxus** partagée via `show360-ui` (même logique que `apps/web`).

## Prérequis

- Rust toolchain
- [Tauri 2 prerequisites](https://v2.tauri.app/start/prerequisites/) (GTK/WebKit sur Linux, etc.)
- API locale + LiveKit (`deploy/docker-compose.yml`)

## Dev

```bash
# depuis la racine du monorepo
cargo tauri dev --manifest-path apps/desktop/src-tauri/Cargo.toml
```

Ou, une fois `cargo install tauri-cli` :

```bash
cd apps/desktop/src-tauri && cargo tauri dev
```

La commande Rust `desktop_info` expose déjà le modèle de rôles (host / présentateur / participant / spectator) pour la trajectoire P0.
