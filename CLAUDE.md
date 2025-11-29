# Super Pixeled

> Panneau LED RGB programmable open-source

---

## 🎯 Projet

| Aspect | Valeur |
|:-------|:-------|
| **Stack** | C++17 (renderer) • Python 3.11+ (API) • TypeScript (web) |
| **Hardware** | Raspberry Pi 5 • Waveshare P2.5 64×32 • HUB75 |
| **Architecture** | Johnny Decimal (`00-docs/`, `10-firmware/`, etc.) |

---

## 📁 Structure

```
00-docs/     → Documentation, plans, specs
10-firmware/ → C++ renderer (rpi-rgb-led-matrix)
20-server/   → Python FastAPI backend
30-mcp/      → Claude MCP Server integration
40-web/      → Frontend dashboard
```

---

## 🔴 Règles Critiques

1. **TDD obligatoire** — Test AVANT implémentation
2. **80% coverage minimum** — pytest (Python), GoogleTest (C++)
3. **Pre-commit DOIT passer** — ruff, clang-format, tests
4. **Jamais de push sans CI verte**

---

## 💻 Commandes

```bash
# Python (20-server/)
uv run pytest                    # Tests
uv run pytest --cov --cov-report=term-missing  # Coverage
uv run ruff check .              # Lint
uv run ruff format .             # Format

# C++ (10-firmware/)
cmake -B build && cmake --build build  # Build
ctest --test-dir build           # Tests
clang-format -i src/*.cpp        # Format

# Pre-commit (root)
pre-commit run --all-files       # Tous les hooks
```

---

## 📝 Conventions

| Type | Convention |
|:-----|:-----------|
| **Commits** | `type(scope): description` — Conventional Commits |
| **Branches** | `feature/xxx`, `fix/xxx`, `docs/xxx` |
| **Python** | PEP 8, ruff, type hints obligatoires |
| **C++** | Google Style, clang-format |
| **Nommage** | `snake_case` (Python/C++), `camelCase` (JS/TS) |

---

## 🚫 Ne Pas Faire

- Skip les tests "pour aller plus vite"
- Commit sans pre-commit
- Modifier `10-firmware/third_party/` (submodule)
- Push sur `main` directement (PR obligatoire)
