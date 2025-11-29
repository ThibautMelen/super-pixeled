# 🎮 Super Pixeled

> Panneau LED RGB programmable 100% open-source et hackable

[![CI](https://github.com/ThibautMelen/super-pixeled/actions/workflows/ci.yml/badge.svg)](https://github.com/ThibautMelen/super-pixeled/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Python 3.11+](https://img.shields.io/badge/python-3.11+-blue.svg)](https://www.python.org/)
[![C++17](https://img.shields.io/badge/C++-17-00599C.svg)](https://isocpp.org/)

---

## ✨ Features

- 🔥 **Effets temps réel** — Fire, Matrix, Waves, Plasma
- 🐍 **API REST + WebSocket** — FastAPI async
- ⚡ **Renderer C++ haute perf** — 60 FPS, Floyd dithering
- 🤖 **MCP Server** — Contrôle via Claude Code
- 🏠 **Intégrations** — Home Assistant, MQTT, Webhooks

---

## 🏗️ Architecture

```mermaid
flowchart LR
    subgraph CONTROL["🖥️ Contrôle"]
        WEB["🌐 Web UI"]
        MCP["🤖 Claude MCP"]
        API["📨 Webhooks"]
    end

    subgraph PI["🍓 Raspberry Pi 5"]
        FASTAPI["🐍 FastAPI"]
        CPP["⚡ C++ Renderer"]
        FASTAPI <-->|IPC| CPP
    end

    subgraph HW["🔌 Hardware"]
        LED["💡 Panel 64×32"]
    end

    CONTROL --> FASTAPI
    CPP --> LED
```

---

## 📁 Structure

```
super-pixeled/
├── 00-docs/        # 📚 Documentation
├── 10-firmware/    # ⚡ C++ Renderer
├── 20-server/      # 🐍 Python API
├── 30-mcp/         # 🤖 MCP Server
└── 40-web/         # 🌐 Dashboard
```

---

## 🚀 Quick Start

### Prérequis

- Raspberry Pi 5 (8GB recommandé)
- Panneau LED HUB75 (Waveshare P2.5 64×32)
- Python 3.11+ & CMake 3.20+

### Installation

```bash
# Clone
git clone --recursive https://github.com/ThibautMelen/super-pixeled.git
cd super-pixeled

# C++ Renderer
cd 10-firmware
cmake -B build && cmake --build build

# Python Server
cd ../20-server
uv sync
uv run uvicorn app.main:app --host 0.0.0.0

# Web UI
open http://raspberrypi.local:8000
```

---

## 🧪 Tests

```bash
# Python
cd 20-server && uv run pytest --cov

# C++
cd 10-firmware && ctest --test-dir build

# Tous les hooks
pre-commit run --all-files
```

---

## 🎨 Effets Disponibles

| Effet | Description | Priorité |
|:------|:------------|:--------:|
| `fire` | Flammes réalistes | 🔴 P1 |
| `matrix` | Code tombant | 🔴 P1 |
| `waves` | Vagues RGB | 🔴 P1 |
| `plasma` | Sinus psychédélique | 🟡 P2 |
| `clock` | Horloge digitale | 🟡 P2 |

---

## 📡 API

```bash
# Afficher du texte
curl -X POST http://pi:8000/api/text \
  -H "Content-Type: application/json" \
  -d '{"text": "Hello!", "color": [255, 0, 0]}'

# Lancer un effet
curl -X POST http://pi:8000/api/effect \
  -d '{"name": "fire"}'
```

---

## 🤝 Contributing

1. Fork le repo
2. Crée ta branche (`git checkout -b feature/amazing`)
3. Commit (`git commit -m 'feat: add amazing feature'`)
4. Push (`git push origin feature/amazing`)
5. Ouvre une PR

---

## 📄 License

MIT © [Thibaut Melen](https://github.com/ThibautMelen)

---

<div align="center">

<sub>Built with Claude Code | Open-source LED panel project | 2025</sub><br/>
<sub>Powered by Raspberry Pi 5 + Waveshare HUB75</sub>

<br/>

<a href="https://github.com/ThibautMelen">
  <img src="https://avatars.githubusercontent.com/u/20891897?s=200&v=4" alt="ThibautMelen" width="32"/>
</a>
&nbsp;❤️&nbsp;
<a href="https://github.com/SuperNovae-studio">
  <img src="https://avatars.githubusercontent.com/u/33066282?s=200&v=4" alt="SuperNovae Studio" width="32"/>
</a>
&nbsp;🏴‍☠️

</div>
