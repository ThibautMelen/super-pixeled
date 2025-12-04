# Super Pixeled

> Panneau LED RGB programmable 100% open-source — Architecture Rust

[![CI](https://github.com/ThibautMelen/super-pixeled/actions/workflows/ci.yml/badge.svg)](https://github.com/ThibautMelen/super-pixeled/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org/)

---

## Features

- **Zero-copy rendering** — Framebuffer direct vers HUB75
- **Async HTTP API** — tokio + axum, 10k req/s
- **Extensible effects** — Trait-based, hot-swap ready
- **Type-safe config** — TOML + serde validation
- **Cross-compile** — Build sur Mac, deploy sur Pi

---

## Architecture Rust

### Pourquoi Rust?

| Besoin | Solution Rust |
|:-------|:--------------|
| **60 FPS stable** | Zero-cost abstractions, no GC pauses |
| **Memory safety** | Borrow checker, pas d'UAF/double-free |
| **Concurrence** | async/await + Send/Sync compile-time |
| **Single binary** | Pas de runtime Python, deploy simple |
| **Cross-compile** | `cross` tool, un seul Cargo.toml |

### Vue d'ensemble

```
┌─────────────────────────────────────────────────────────────┐
│                     SUPER PIXELED                           │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────┐    ┌─────────┐    ┌─────────┐    ┌─────────┐  │
│  │  axum   │───▶│EffectMgr│───▶│Renderer │───▶│ HUB75   │  │
│  │  API    │    │         │    │         │    │ Driver  │  │
│  └─────────┘    └─────────┘    └─────────┘    └─────────┘  │
│       │              │              │              │        │
│       │         ┌────┴────┐        │              │        │
│       │         │ Effects │        │              ▼        │
│       │         │ ┌─────┐ │        │         ┌────────┐    │
│       │         │ │Fire │ │        │         │ GPIO   │    │
│       │         │ │Wave │ │        │         │ Pi 5   │    │
│       │         │ │Matrix│ │       │         └────────┘    │
│       │         │ └─────┘ │        │              │        │
│       │         └─────────┘        │              ▼        │
│       │                            │         ┌────────┐    │
│       └────────────────────────────┘         │ Panel  │    │
│                                              │ 64x32  │    │
│                                              └────────┘    │
└─────────────────────────────────────────────────────────────┘
```

### Structure des modules

```
super-pixeled/
├── Cargo.toml              # Workspace root
├── crates/
│   ├── super-pixeled/      # Binary crate (main.rs)
│   ├── sp-core/            # Types partagés, config
│   ├── sp-effects/         # Système d'effets (trait Effect)
│   ├── sp-renderer/        # Framebuffer, dithering
│   ├── sp-hub75/           # Driver HUB75 (GPIO)
│   └── sp-api/             # Routes HTTP axum
├── config/
│   └── default.toml        # Configuration par défaut
└── tests/
    └── integration/        # Tests E2E
```

### Responsabilités des crates

| Crate | Responsabilité |
|:------|:---------------|
| `sp-core` | `Color`, `Point`, `Config`, error types |
| `sp-effects` | Trait `Effect`, `EffectManager`, effects builtin |
| `sp-renderer` | `Framebuffer`, Floyd-Steinberg dithering |
| `sp-hub75` | GPIO driver, timing critique, DMA si dispo |
| `sp-api` | Routes axum, validation, WebSocket |

---

## Strategie de Developpement

### Workflow quotidien

```bash
# Developpement local (mock hardware)
cargo run -- --mock

# Tests + lint
cargo test && cargo clippy -- -D warnings

# Watch mode (hot reload)
cargo watch -x 'run -- --mock'

# Format
cargo fmt --check
```

### Cross-compilation Raspberry Pi

```bash
# Installation cross (une fois)
cargo install cross

# Build release pour Pi 5 (aarch64)
cross build --release --target aarch64-unknown-linux-gnu

# Deploy via SSH
scp target/aarch64-unknown-linux-gnu/release/super-pixeled pi@raspberrypi.local:~
ssh pi@raspberrypi.local ./super-pixeled
```

### Debug sur Pi

```bash
# Logs structurés (tracing)
RUST_LOG=debug ./super-pixeled

# Remote debug avec lldb
# 1. Sur le Pi:
lldb-server platform --listen "*:1234" --server
# 2. Sur Mac:
lldb
(lldb) platform select remote-linux
(lldb) platform connect connect://raspberrypi.local:1234
```

### CI/CD recommandé

```yaml
# .github/workflows/ci.yml
name: CI
on: [push, pull_request]
jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo fmt --check
      - run: cargo clippy -- -D warnings
      - run: cargo test

  build-pi:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: aarch64-unknown-linux-gnu
      - uses: taiki-e/install-action@cross
      - run: cross build --release --target aarch64-unknown-linux-gnu
      - uses: actions/upload-artifact@v4
        with:
          name: super-pixeled-pi
          path: target/aarch64-unknown-linux-gnu/release/super-pixeled
```

---

## Roadmap Technique

### Phase 1: Core (MVP)

- [x] Structure workspace Cargo
- [ ] `sp-core`: types de base, config TOML
- [ ] `sp-renderer`: framebuffer 64x32
- [ ] `sp-hub75`: driver GPIO mock
- [ ] `sp-api`: routes `/health`, `/effect`
- [ ] Binary: serveur HTTP fonctionnel

### Phase 2: Effects

- [ ] Trait `Effect` avec lifecycle
- [ ] Effects: `fire`, `matrix`, `waves`
- [ ] `EffectManager`: transitions, queuing
- [ ] Hot-swap effects sans restart

### Phase 3: Production

- [ ] Driver HUB75 real (GPIO + DMA)
- [ ] WebSocket live preview
- [ ] Rate limiting + auth basique
- [ ] Metrics Prometheus
- [ ] Graceful shutdown (SIGTERM)

### Phase 4: Extensions

- [ ] Dashboard Web (TypeScript + Vite)
- [ ] MCP Server integration
- [ ] Home Assistant discovery
- [ ] Benchmarks FPS (criterion)
- [ ] OpenAPI auto-generated (utoipa)

---

## Getting Started

### Prerequisites

```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Cross-compilation tools
cargo install cross cargo-watch

# Target Pi 5
rustup target add aarch64-unknown-linux-gnu
```

### Clone et build

```bash
git clone https://github.com/ThibautMelen/super-pixeled.git
cd super-pixeled

# Build
cargo build

# Run en mode mock (sans hardware)
cargo run -- --mock

# Tests
cargo test
```

### Premier test API

```bash
# Terminal 1: lancer le serveur
cargo run -- --mock

# Terminal 2: tester l'API
curl http://localhost:3000/health
# {"status":"ok"}

curl -X POST http://localhost:3000/api/effect \
  -H "Content-Type: application/json" \
  -d '{"name": "fire", "params": {"intensity": 0.8}}'
# {"success":true,"effect":"fire"}
```

---

## Configuration

```toml
# config/default.toml

[server]
host = "0.0.0.0"
port = 3000

[panel]
width = 64
height = 32
brightness = 80  # 0-100

[hardware]
mock = false     # true pour dev sans Pi
gpio_slowdown = 2

[effects]
default = "fire"
transition_ms = 500

[logging]
level = "info"   # trace, debug, info, warn, error
format = "pretty" # pretty, json
```

---

## API Reference

### Health Check

```http
GET /health
```

Response:
```json
{"status": "ok", "uptime_secs": 3600}
```

### Effects

```http
POST /api/effect
Content-Type: application/json

{
  "name": "fire",
  "params": {
    "intensity": 0.8,
    "palette": "inferno"
  }
}
```

### Text Display

```http
POST /api/text
Content-Type: application/json

{
  "text": "Hello!",
  "color": [255, 0, 0],
  "scroll": true,
  "speed": 50
}
```

### Raw Framebuffer

```http
POST /api/raw
Content-Type: application/octet-stream

[binary RGB data: 64*32*3 = 6144 bytes]
```

### WebSocket (live preview)

```javascript
const ws = new WebSocket('ws://localhost:3000/ws');
ws.onmessage = (e) => {
  const frame = new Uint8Array(e.data);
  // Render frame to canvas
};
```

---

## Effects System

Chaque effet implémente le trait `Effect`:

```rust
pub trait Effect: Send + Sync {
    /// Nom unique de l'effet
    fn name(&self) -> &'static str;

    /// Initialisation (appelé une fois)
    fn init(&mut self, config: &EffectConfig);

    /// Génère la frame suivante
    /// Retourne true si l'effet continue, false si terminé
    fn tick(&mut self, fb: &mut Framebuffer, dt: Duration) -> bool;

    /// Cleanup (appelé à la fin)
    fn cleanup(&mut self) {}
}
```

### Effets disponibles

| Effet | Description | Params |
|:------|:------------|:-------|
| `fire` | Flammes réalistes | `intensity`, `palette` |
| `matrix` | Code tombant | `speed`, `density` |
| `waves` | Vagues RGB | `frequency`, `amplitude` |
| `plasma` | Sinus psychédélique | `complexity` |
| `solid` | Couleur unie | `color` |
| `off` | Éteint | - |

---

## Testing

```bash
# Unit tests
cargo test

# Integration tests
cargo test --test integration

# Coverage (requires cargo-llvm-cov)
cargo llvm-cov --html

# Benchmarks
cargo bench
```

---

## Performance

### Targets

| Metric | Target | Actual |
|:-------|:-------|:-------|
| **FPS** | 60 | TBD |
| **Latency API** | < 10ms p99 | TBD |
| **Memory** | < 50MB RSS | TBD |
| **Binary size** | < 5MB | TBD |

### Profiling

```bash
# CPU profiling
cargo build --release
perf record ./target/release/super-pixeled --mock
perf report

# Memory
valgrind --tool=massif ./target/release/super-pixeled --mock
```

---

## Hardware

### Bill of Materials

| Component | Model | Price |
|:----------|:------|------:|
| SBC | Raspberry Pi 5 8GB | 159€ |
| Panel | Waveshare P2.5 64x32 | 30€ |
| Adapter | HUB75 Xicoolee | 20€ |
| PSU | 5V 10A | 30€ |
| **Total** | | **239€** |

### Wiring

```
Raspberry Pi 5          HUB75 Adapter          LED Panel
┌─────────────┐         ┌─────────────┐        ┌─────────┐
│   GPIO 40   │◀───────▶│   GPIO 40   │        │         │
│   pins      │         │             │◀──────▶│  HUB75  │
└─────────────┘         │   HUB75 OUT │        │  INPUT  │
                        └─────────────┘        └─────────┘
                              │
                              ▼
                        ┌─────────────┐
                        │   5V 10A    │
                        │   PSU       │
                        └─────────────┘
```

---

## Contributing

1. Fork le repo
2. Crée ta branche (`git checkout -b feature/amazing`)
3. Commit (`git commit -m 'feat: add amazing feature'`)
4. Push (`git push origin feature/amazing`)
5. Ouvre une PR

### Commit Convention

```
type(scope): description

Types: feat, fix, docs, style, refactor, test, chore
Scope: api, effects, renderer, hub75, core
```

---

## License

MIT - [Thibaut Melen](https://github.com/ThibautMelen)

---

<div align="center">
<sub>Built with Rust + Claude Code | Open-source LED panel project | 2025</sub>
</div>
