# Super Pixeled

> Panneau LED RGB programmable open-source — Architecture Rust

---

## Projet

| Aspect | Valeur |
|:-------|:-------|
| **Stack** | Rust 1.75+ (monolith) • TypeScript (web) |
| **Hardware** | Raspberry Pi 5 • Waveshare P2.5 64×32 • HUB75 |
| **Architecture** | Cargo workspace (`crates/`) |

---

## Structure

```
crates/
├── super-pixeled/   → Binary (main.rs)
├── sp-core/         → Types, config, errors
├── sp-effects/      → Effect system (trait + builtin)
├── sp-renderer/     → Framebuffer
├── sp-hub75/        → LED driver (mockable)
└── sp-api/          → HTTP routes (axum)

config/              → TOML configuration
00-docs/             → Documentation legacy
```

---

## Regles Critiques

1. **TDD obligatoire** — `cargo test` AVANT push
2. **Clippy clean** — `cargo clippy -- -D warnings`
3. **Format** — `cargo fmt --check`
4. **CI verte** — Jamais de push sans CI

---

## Commandes

```bash
# Developpement
cargo run -- --mock           # Run avec mock hardware
cargo watch -x 'run -- --mock' # Hot reload
cargo test                    # Tests
cargo clippy                  # Lint

# Cross-compilation Pi
cross build --release --target aarch64-unknown-linux-gnu

# Deploy
scp target/aarch64-unknown-linux-gnu/release/super-pixeled pi@raspberrypi.local:~
```

---

## Conventions

| Type | Convention |
|:-----|:-----------|
| **Commits** | `type(scope): description` — Conventional Commits |
| **Branches** | `feature/xxx`, `fix/xxx`, `docs/xxx` |
| **Rust** | `snake_case` fonctions, `PascalCase` types |
| **Modules** | Un fichier = un module, `mod.rs` pour dossiers |

---

## Ne Pas Faire

- Skip les tests
- `#[allow(clippy::...)]` sans justification
- `unsafe` sans revue
- Push sur `main` directement (PR obligatoire)
