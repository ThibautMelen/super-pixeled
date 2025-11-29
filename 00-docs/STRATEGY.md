# Super LED Panel - Strategie Projet

> **Panneau LED programmable open-source style iPixel Colors**
> Projet initie le 2025-11-29

---

## Vision du Projet

```mermaid
%%{init: {'theme': 'base', 'themeVariables': { 'primaryColor': '#0ea5e9', 'primaryTextColor': '#f1f5f9', 'primaryBorderColor': '#38bdf8', 'lineColor': '#64748b', 'secondaryColor': '#a855f7', 'tertiaryColor': '#0f172a', 'background': '#020617', 'mainBkg': '#0f172a', 'nodeBorder': '#38bdf8', 'clusterBkg': '#1e293b', 'titleColor': '#f8fafc', 'edgeLabelBackground': '#1e293b', 'nodeTextColor': '#f1f5f9'}}}%%
mindmap
  root((🎯 Super LED<br/>Panel))
    🔓 Open Source
      100%<br/>Hackable
      MIT<br/>License
      GitHub<br/>Public
    🎮 Controlable
      ⚡ C++<br/>Renderer
      🐍 Python<br/>API
      🌐 Web<br/>Interface
      🤖 Claude<br/>MCP
    🔌 Integrations
      📨 Webhooks<br/>HTTP
      🏠 Home<br/>Assistant
      📡 MQTT<br/>IoT
    ✨ Effets
      🔥 Fire<br/>Flammes
      💻 Matrix<br/>Rain
      🌊 Waves<br/>Vagues
      🎨 Custom<br/>Scripts
```

### Objectif

Creer un panneau LED RGB programmable **100% open-source**, controlable via:

| Stack | Role | Langages |
|:------|:-----|:---------|
| **Renderer** | Rendu bas niveau, effets, animations | `C++` |
| **Backend** | API, webhooks, MCP integration | `Python` |
| **Frontend** | Dashboard, controle a distance | `HTML/JS` |

> **Inspiration**: Panneaux iPixel Colors mais entierement hackable et integrable dans des SaaS.

---

## Architecture Technique

```mermaid
%%{init: {'theme': 'base', 'themeVariables': {'lineColor': '#64748b'}}}%%
flowchart TB
    accTitle: Architecture Super LED Panel
    accDescr: Diagramme montrant le flux de controle externe vers le Raspberry Pi et le hardware LED

    classDef external fill:#06b6d4,stroke:#0891b2,stroke-width:2px,color:#fff
    classDef process fill:#3b82f6,stroke:#2563eb,stroke-width:2px,color:#fff
    classDef data fill:#a855f7,stroke:#9333ea,stroke-width:2px,color:#fff
    classDef success fill:#22c55e,stroke:#16a34a,stroke-width:2px,color:#fff

    subgraph CONTROL["🖥️ CONTROLE EXTERNE"]
        direction TB
        MAC["💻 Mac<br/>Site Web"]:::external
        MCP["🤖 MCP Server<br/>Claude Code<br/>AI Control"]:::external
        WEBHOOK["📨 Webhooks<br/>Zapier • n8n<br/>GitHub Actions"]:::external
    end

    subgraph PI["🍓 RASPBERRY PI 5<br/>8GB RAM • Quad-Core"]
        direction TB
        subgraph PYTHON["🐍 Python Server"]
            direction LR
            FASTAPI["⚡ FastAPI<br/>Async"]:::process
            WS["🔌 WebSocket<br/>Realtime"]:::process
            REST["📡 REST<br/>API"]:::process
        end
        subgraph CPP["⚙️ C++ Renderer"]
            direction LR
            FB["🖼️ Frame<br/>buffer"]:::data
            FX["✨ Effects<br/>Engine"]:::data
            DITHER["🎨 Floyd<br/>Dithering"]:::data
        end
        PYTHON <--->|"JSON<br/>Socket IPC"| CPP
    end

    subgraph HW["🔌 HARDWARE"]
        direction TB
        HUB75["📟 Adaptateur<br/>HUB75<br/>Xicoolee"]:::success
        PANEL["💡 Panneau LED<br/>Waveshare P2.5<br/>64×32 px<br/>2048 LEDs"]:::success
        PSU["🔋 Alimentation<br/>LEICKE<br/>5V 10A • 50W"]:::success
    end

    MAC -->|"HTTP"| FASTAPI
    MCP -->|"WebSocket"| WS
    WEBHOOK -->|"POST"| REST

    CPP -->|"GPIO 40 pins"| HUB75
    HUB75 -->|"IDC 16 broches"| PANEL
    PSU -->|"5V DC"| PANEL

    style CONTROL fill:#dbeafe,stroke:#3b82f6,stroke-width:2px,color:#1e40af
    style PI fill:#e0e7ff,stroke:#6366f1,stroke-width:2px,color:#3730a3
    style HW fill:#dcfce7,stroke:#22c55e,stroke-width:2px,color:#166534
    style PYTHON fill:#dbeafe,stroke:#3b82f6,stroke-width:2px,color:#1e40af
    style CPP fill:#f3e8ff,stroke:#a855f7,stroke-width:2px,color:#6b21a8
```

### Stack Logiciel

```mermaid
%%{init: {'theme': 'base', 'themeVariables': {'lineColor': '#64748b'}}}%%
flowchart LR
    accTitle: Stack Logiciel
    accDescr: Langages et bibliotheques utilisees pour le projet LED Panel

    classDef process fill:#3b82f6,stroke:#2563eb,stroke-width:2px,color:#fff
    classDef data fill:#a855f7,stroke:#9333ea,stroke-width:2px,color:#fff
    classDef decision fill:#f59e0b,stroke:#d97706,stroke-width:2px,color:#fff

    subgraph LANG["📚 Langages"]
        direction TB
        PY["🐍 Python<br/>3.11+<br/>Async/Await"]:::decision
        CPP2["⚡ C++17<br/>Modern<br/>Standard"]:::decision
    end

    subgraph PYLIBS["🐍 Python Stack"]
        direction TB
        FASTAPI2["FastAPI<br/>Framework"]:::process
        PYDANTIC["Pydantic v2<br/>Validation"]:::process
        UVICORN["Uvicorn<br/>ASGI Server"]:::process
        WEBSOCKETS["WebSockets<br/>Realtime"]:::process
    end

    subgraph CPPLIBS["⚡ C++ Stack"]
        direction TB
        RGBMATRIX["rpi-rgb-led-matrix<br/>hzeller/lib"]:::data
        CMAKE["CMake 3.20+<br/>Build System"]:::data
        JSON["nlohmann/json<br/>Serialization"]:::data
    end

    PY --> PYLIBS
    CPP2 --> CPPLIBS

    style LANG fill:#fef3c7,stroke:#f59e0b,stroke-width:2px,color:#92400e
    style PYLIBS fill:#dbeafe,stroke:#3b82f6,stroke-width:2px,color:#1e40af
    style CPPLIBS fill:#f3e8ff,stroke:#a855f7,stroke-width:2px,color:#6b21a8
```

---

## Liste d'Achats Validee

### Composants Essentiels

| # | Produit | Image | Prix | Status |
|:-:|:--------|:-----:|-----:|:------:|
| 1 | **Kit Raspberry Pi 5 8GB**<br/>Boitier metal • NVMe • 27W • SD 64GB<br/>[B0CRMQCYXH](https://www.amazon.fr/dp/B0CRMQCYXH) | ![Pi5](https://m.media-amazon.com/images/I/71OyrTU4FhL._AC_SL1200_.jpg) | **158,90€** | ✅ |
| 2 | **Panneau Waveshare P2.5 64×32**<br/>2048 LEDs • Cable IDC inclus<br/>[B0BQYDLHY9](https://www.amazon.fr/dp/B0BQYDLHY9) | ![Panel](https://www.waveshare.com/media/catalog/product/cache/1/image/560x560/9df78eab33525d08d6e5fb8d27136e95/r/g/rgb-matrix-p2.5-64x32-1.jpg) | **29,99€** | ✅ |
| 3 | **Adaptateur HUB75 Xicoolee**<br/>GPIO→HUB75 • Compatible Pi 5<br/>[B0BC8Y447G](https://www.amazon.fr/dp/B0BC8Y447G) | ![HUB75](https://m.media-amazon.com/images/I/61Ux+GFTS6L._AC_SL1001_.jpg) | **19,90€** | ✅ |
| 4 | **Alimentation LEICKE 5V 10A**<br/>50W • Pour panneau LED<br/>[B07YVBHH6K](https://www.amazon.fr/dp/B07YVBHH6K) | ![PSU](https://m.media-amazon.com/images/I/61QoN8vBa0L._AC_SL1001_.jpg) | **29,99€** | ✅ |

### Composants Optionnels (Recommandes)

| # | Produit | Image | Prix | Verdict |
|:-:|:--------|:-----:|-----:|:--------|
| 5 | **Kit Freenove 132 projets**<br/>860 pages • Python/C/Java<br/>[B092V1BPBC](https://www.amazon.fr/dp/B092V1BPBC) | ![Freenove](https://m.media-amazon.com/images/I/81w-2uXZx4L._AC_SL1500_.jpg) | **64,95€** | ✅ Meme stack! |
| 6 | **Ecran LCD I2C 16×2**<br/>Debug • Status<br/>[B0B76YGDV4](https://www.amazon.fr/dp/B0B76YGDV4) | ![LCD](https://m.media-amazon.com/images/I/61DuFvDZYxL._AC_SL1200_.jpg) | **8,95€** | 🤔 Optionnel |

---

### Kit Freenove - Synergie avec le Projet

```mermaid
%%{init: {'theme': 'base', 'themeVariables': {'lineColor': '#64748b'}}}%%
flowchart LR
    accTitle: Integration Kit Freenove
    accDescr: Comment les capteurs Freenove s'integrent au projet LED Panel

    classDef success fill:#22c55e,stroke:#16a34a,stroke-width:2px,color:#fff
    classDef process fill:#3b82f6,stroke:#2563eb,stroke-width:2px,color:#fff
    classDef data fill:#a855f7,stroke:#9333ea,stroke-width:2px,color:#fff

    subgraph FREENOVE["📦 Kit Freenove<br/>132 Projets Python/C"]
        direction TB
        MOTION["👁️ PIR<br/>Mouvement<br/>Detection"]:::success
        ULTRA["📏 Ultrason<br/>HC-SR04<br/>Distance"]:::success
        RFID2["💳 RFID<br/>RC522<br/>Badges NFC"]:::success
        ACCEL["📐 MPU6050<br/>Accelerometre<br/>Gyroscope"]:::success
        KEYPAD["🔢 Keypad<br/>4×4 Matrix<br/>16 touches"]:::success
    end

    subgraph PROJECT["🎯 Super LED Panel"]
        direction TB
        API2["🐍 FastAPI<br/>REST + WS<br/>Server"]:::process
        RENDER["⚡ C++<br/>Effects<br/>Renderer"]:::data
        LED2["💡 Panneau<br/>64×32 px<br/>2048 LEDs"]:::process
    end

    MOTION -->|"Detecte<br/>→ Animation"| API2
    ULTRA -->|"Distance<br/>→ Intensite"| API2
    RFID2 -->|"Badge ID<br/>→ Theme"| API2
    ACCEL -->|"Tilt XYZ<br/>→ Effet"| API2
    KEYPAD -->|"Touche<br/>→ Mode"| API2

    API2 --> RENDER --> LED2

    style FREENOVE fill:#dcfce7,stroke:#22c55e,stroke-width:2px,color:#166534
    style PROJECT fill:#dbeafe,stroke:#3b82f6,stroke-width:2px,color:#1e40af
```

| Capteur Freenove | Integration LED Panel |
|:-----------------|:----------------------|
| 👁️ PIR Motion | Declencheur d'animation automatique |
| 📏 Ultrason | Effet selon distance (proche = intense) |
| 💳 RFID | Animation personnalisee par badge |
| 📐 Accelerometre | Effet qui suit l'inclinaison |
| 🔢 Keypad | Selection manuelle des effets |

---

### A NE PAS ACHETER ❌

| Produit | ASIN | Raison |
|:--------|:-----|:-------|
| ~~Boitier ElectroCookie~~ | B0CV7GN22G | **DOUBLON** - Kit Pi5 inclut boitier metal |
| ~~Blocs 5V 2A~~ | B0F485M8SG | **INSUFFISANT** - 2A < 2.5A requis |

---

### Recapitulatif Budget

```mermaid
%%{init: {'theme': 'base', 'themeVariables': { 'pieOuterStrokeWidth': '2px', 'pieOpacity': '1', 'pieLegendTextColor': '#f1f5f9', 'pieSectionTextColor': '#0f172a', 'pie1': '#0ea5e9', 'pie2': '#22c55e', 'pie3': '#a855f7', 'pie4': '#f59e0b', 'pie5': '#ef4444', 'pie6': '#ec4899'}}}%%
pie showData
    title 💰 Budget Setup Complet (~312€)
    "🍓 Pi 5 Kit" : 159
    "💡 Panneau LED" : 30
    "📟 Adaptateur" : 20
    "🔋 Alimentation" : 30
    "📦 Freenove" : 65
    "📺 LCD" : 9
```

| Configuration | Composants | Prix |
|:--------------|:-----------|-----:|
| **Setup Minimal** | Pi5 + Panel + HUB75 + PSU | **239€** |
| **Setup Complet** | + Freenove + LCD | **312€** |

> **Recommandation**: Le **Setup Complet** est ideal car Freenove utilise le meme stack Python/C et permet d'enrichir le projet avec des capteurs.

---

## Specifications Techniques

### Panneau Waveshare RGB-Matrix-P2.5-64x32

```mermaid
%%{init: {'theme': 'base', 'themeVariables': {'lineColor': '#64748b'}}}%%
flowchart LR
    accTitle: Specifications Waveshare P2.5
    accDescr: Specifications techniques du panneau LED RGB 64x32

    classDef success fill:#22c55e,stroke:#16a34a,stroke-width:2px,color:#fff
    classDef decision fill:#f59e0b,stroke:#d97706,stroke-width:2px,color:#fff
    classDef data fill:#a855f7,stroke:#9333ea,stroke-width:2px,color:#fff

    subgraph SPECS["📐 Specifications<br/>Panneau"]
        direction TB
        RES["📺 Resolution<br/>64×32 pixels<br/>2048 LEDs"]:::success
        PITCH["📏 Pitch<br/>2.5mm<br/>Haute densite"]:::success
        SIZE["📐 Dimensions<br/>160×80mm<br/>Compact"]:::success
        ANGLE["👁️ Vision<br/>≥140°<br/>Large angle"]:::success
    end

    subgraph POWER["⚡ Alimentation<br/>Requise"]
        direction TB
        VOLT["🔌 Tension<br/>5V DC<br/>Stable"]:::decision
        AMP["⚡ Courant<br/>2.5A min<br/>4A optimal"]:::decision
        WATT["💡 Puissance<br/>≤12W<br/>Max"]:::decision
    end

    subgraph CONNECT["🔗 Connexion<br/>HUB75"]
        direction TB
        HUB["📟 Interface<br/>HUB75<br/>IN + OUT"]:::data
        SCAN["🔄 Scan<br/>1/16<br/>Multiplexe"]:::data
        CHAIN["🔗 Chainable<br/>Oui<br/>Multi-panels"]:::data
    end

    style SPECS fill:#dcfce7,stroke:#22c55e,stroke-width:2px,color:#166534
    style POWER fill:#fef3c7,stroke:#f59e0b,stroke-width:2px,color:#92400e
    style CONNECT fill:#f3e8ff,stroke:#a855f7,stroke-width:2px,color:#6b21a8
```

| Spec | Valeur |
|:-----|:-------|
| Resolution | 64×32 = **2048 pixels** |
| Pitch | 2.5mm |
| Dimensions | 160mm × 80mm |
| Angle de vision | ≥140° |
| Type de scan | 1/16 scan |
| Interface | HUB75 (IN + OUT chainable) |
| Alimentation | **5V / 2.5A min** (4A recommande) |
| Puissance | ≤12W |
| Contenu | Panneau + Adaptateur + Cable IDC 16P |

### Raspberry Pi 5 (Kit B0CRMQCYXH)

| Spec | Valeur |
|:-----|:-------|
| CPU | Quad-core ARM Cortex-A76 @ **2.4GHz** |
| RAM | **8GB** LPDDR4X |
| Stockage | microSD 64GB + NVMe PCIe |
| Alimentation | USB-C 27W incluse |
| Connectivite | WiFi • Bluetooth • Gigabit |
| GPIO | 40 pins |

### Adaptateur HUB75 Xicoolee

| Spec | Valeur |
|:-----|:-------|
| Compatibilite | Tous Raspberry Pi + Pico |
| Entree alim | USB-C 5V/4A **ou** DC 5V/8A |
| Sortie | HUB75 + Borniers VH-4P |
| Inclus | Cables • Visserie • Entretoises |

---

## Workflow de Montage

```mermaid
%%{init: {'theme': 'base', 'themeVariables': { 'primaryColor': '#0ea5e9', 'primaryTextColor': '#f1f5f9', 'background': '#020617', 'mainBkg': '#0f172a', 'clusterBkg': '#1e293b'}}}%%
flowchart LR
    subgraph P1["📁 PHASE 1<br/>Preparation<br/>Avant Reception"]
        direction TB
        REPO["1️⃣ Creer repo<br/>GitHub Public<br/>super-led-panel"]
        STRUCT["2️⃣ Structure<br/>firmware/ server/<br/>web/ docs/"]
        SUBMOD["3️⃣ Submodule<br/>rpi-rgb-led-matrix<br/>hzeller lib"]
        REPO --> STRUCT --> SUBMOD
    end

    subgraph P2["🔧 PHASE 2<br/>Hardware<br/>Montage Physique"]
        direction TB
        PI_SETUP["1️⃣ Pi 5<br/>SD + Boitier<br/>Boot Test"]
        HUB_INSTALL["2️⃣ HUB75<br/>GPIO 40 pins<br/>Verifier orientation"]
        PANEL_CONNECT["3️⃣ Panneau<br/>Cable IDC 16P<br/>Waveshare"]
        POWER2["4️⃣ Allumage<br/>Pi → Attendre<br/>→ 5V Panel"]
        PI_SETUP --> HUB_INSTALL --> PANEL_CONNECT --> POWER2
    end

    subgraph P3["💻 PHASE 3<br/>Software<br/>Code + Test"]
        direction TB
        SSH["1️⃣ SSH<br/>pi@raspberrypi<br/>.local"]
        CLONE["2️⃣ Clone<br/>git clone<br/>super-led-panel"]
        DEPS["3️⃣ Deps<br/>apt + pip<br/>cmake build"]
        COMPILE2["4️⃣ Compile<br/>make<br/>rpi-rgb-led-matrix"]
        TEST2["5️⃣ Test<br/>./demo -D0<br/>First Light!"]
        SSH --> CLONE --> DEPS --> COMPILE2 --> TEST2
    end

    P1 ==> P2 ==> P3

    style P1 fill:#0f172a,stroke:#0ea5e9,stroke-width:4px,color:#f1f5f9
    style P2 fill:#7c2d12,stroke:#f59e0b,stroke-width:4px,color:#f1f5f9
    style P3 fill:#052e16,stroke:#22c55e,stroke-width:4px,color:#f1f5f9
```

### Phase 1: Preparation (Avant reception)

```bash
# Creer le repo GitHub
git init super-led-panel
cd super-led-panel

# Structure du projet
mkdir -p firmware/{include,src,third_party}
mkdir -p server/{app,scripts}
mkdir -p web docs

# Ajouter la lib LED comme submodule
git submodule add \
  https://github.com/hzeller/rpi-rgb-led-matrix \
  firmware/third_party/rpi-rgb-led-matrix
```

### Phase 2: Montage Hardware

```mermaid
%%{init: {'theme': 'base', 'themeVariables': { 'primaryColor': '#f59e0b', 'primaryTextColor': '#f1f5f9', 'background': '#020617'}}}%%
sequenceDiagram
    participant SD as 💾 microSD
    participant PI as 🍓 Pi 5
    participant HUB as 📟 HUB75
    participant LED as 💡 Panel
    participant PSU as 🔋 5V 10A

    Note over SD,PI: 1. Preparer Pi 5
    SD->>PI: Inserer SD (OS preinstalle)
    Note over PI: Monter dans boitier

    Note over PI,HUB: 2. Installer HUB75
    HUB->>PI: Enficher sur GPIO 40 pins
    Note over HUB: Verifier orientation

    Note over HUB,LED: 3. Connecter panneau
    HUB->>LED: Cable IDC 16 broches

    Note over LED,PSU: 4. Sequence allumage
    Note over PI: ⚡ Brancher USB-C Pi
    Note over PI: ⏳ Attendre boot complet
    PSU->>LED: ⚡ Brancher 5V panneau
```

### Phase 3: Installation Software

```bash
# Sur le Raspberry Pi (via SSH)
ssh pi@raspberrypi.local

# Cloner le repo
git clone https://github.com/ThibautMelen/super-led-panel.git
cd super-led-panel

# Installer dependances
sudo apt update
sudo apt install -y build-essential cmake python3-pip python3-venv

# Compiler la lib LED
cd firmware/third_party/rpi-rgb-led-matrix
make

# Tester avec un exemple
cd examples-api-use
make
sudo ./demo -D0 --led-no-hardware-pulse --led-cols=64 --led-rows=32
```

---

## Structure du Repository

```mermaid
%%{init: {'theme': 'base', 'themeVariables': { 'primaryColor': '#a855f7', 'primaryTextColor': '#f1f5f9', 'background': '#020617'}}}%%
flowchart TB
    subgraph ROOT["📁 super-led-panel/<br/>Repository Root"]
        direction TB
        subgraph FIRMWARE["⚡ firmware/<br/>C++ Renderer"]
            direction LR
            INC["include/<br/>Headers<br/>*.hpp"]
            SRC["src/<br/>Sources<br/>*.cpp"]
            THIRD["third_party/<br/>rpi-rgb-led-matrix<br/>Submodule"]
        end

        subgraph SERVER["🐍 server/<br/>Python Backend"]
            direction LR
            APP["app/<br/>FastAPI<br/>main.py"]
            SCRIPTS["scripts/<br/>CLI tools<br/>Utilities"]
        end

        subgraph WEB["🌐 web/<br/>Frontend UI"]
            direction LR
            HTML["index.html<br/>Dashboard<br/>Layout"]
            JS["app.js<br/>Controls<br/>WebSocket"]
        end

        subgraph DOCS["📚 docs/<br/>Documentation"]
            direction LR
            STRAT["STRATEGY.md<br/>Vision<br/>Planning"]
            HW["hardware.md<br/>Montage<br/>Wiring"]
            PROTO["protocol.md<br/>JSON API<br/>Spec"]
        end
    end

    style ROOT fill:#0f172a,stroke:#64748b,stroke-width:3px,color:#f1f5f9
    style FIRMWARE fill:#312e81,stroke:#818cf8,stroke-width:3px,color:#f1f5f9
    style SERVER fill:#1e3a5f,stroke:#3b82f6,stroke-width:3px,color:#f1f5f9
    style WEB fill:#7c2d12,stroke:#f59e0b,stroke-width:3px,color:#f1f5f9
    style DOCS fill:#052e16,stroke:#22c55e,stroke-width:3px,color:#f1f5f9
```

```
super-led-panel/
├── firmware/                    # ⚡ C++ - Moteur de rendu
│   ├── include/
│   │   ├── framebuffer.hpp
│   │   ├── effects.hpp
│   │   ├── protocol.hpp
│   │   └── renderer.hpp
│   ├── src/
│   │   ├── main.cpp
│   │   ├── effects_fire.cpp
│   │   ├── effects_waves.cpp
│   │   └── effects_matrix.cpp
│   └── third_party/
│       └── rpi-rgb-led-matrix/
│
├── server/                      # 🐍 Python - API
│   ├── app/
│   │   ├── main.py              # FastAPI app
│   │   ├── api.py               # REST endpoints
│   │   ├── websocket.py         # WS handler
│   │   └── models.py            # Pydantic
│   └── requirements.txt
│
├── web/                         # 🌐 Frontend
│   ├── index.html
│   └── app.js
│
└── docs/                        # 📚 Documentation
    └── STRATEGY.md
```

---

## Protocole de Communication

### Format JSON (Python → C++)

```json
{
  "type": "text|image|effect|raw",
  "payload": {
    "text": "Hello World",
    "color": [255, 0, 0],
    "x": 0,
    "y": 0,
    "scroll": true,
    "speed": 50
  }
}
```

### Endpoints API

```mermaid
%%{init: {'theme': 'base', 'themeVariables': { 'primaryColor': '#3b82f6', 'primaryTextColor': '#f1f5f9', 'background': '#020617'}}}%%
flowchart LR
    subgraph API["🐍 FastAPI<br/>REST + WebSocket"]
        direction TB
        POST1["📝 POST<br/>/api/text<br/>Afficher texte"]
        POST2["🖼️ POST<br/>/api/image<br/>PNG → Panel"]
        POST3["✨ POST<br/>/api/effect<br/>Lancer animation"]
        POST4["🎨 POST<br/>/api/raw<br/>Framebuffer direct"]
        GET1["📊 GET<br/>/api/status<br/>Etat systeme"]
        WS2["🔌 WS<br/>/ws<br/>Realtime bidirectionnel"]
    end

    style API fill:#1e3a5f,stroke:#3b82f6,stroke-width:3px,color:#f1f5f9
```

| Methode | Endpoint | Description |
|:--------|:---------|:------------|
| `POST` | `/api/text` | Afficher du texte |
| `POST` | `/api/image` | Afficher une image |
| `POST` | `/api/effect` | Lancer un effet |
| `POST` | `/api/raw` | Envoyer framebuffer |
| `GET` | `/api/status` | Etat du panneau |
| `WS` | `/ws` | WebSocket temps reel |

---

## Effets a Implementer

```mermaid
%%{init: {'theme': 'base', 'themeVariables': { 'primaryColor': '#ef4444', 'primaryTextColor': '#f1f5f9', 'background': '#020617', 'clusterBkg': '#1e293b'}}}%%
flowchart TB
    subgraph P1["🔴 PRIORITE 1<br/>MVP • Must Have"]
        direction LR
        FIRE["🔥 fire<br/>Flammes<br/>Realistes"]
        WAVES["🌊 waves<br/>Vagues<br/>RGB fluides"]
        MATRIX["💻 matrix_rain<br/>Code vert<br/>Tombant"]
        SCROLL["📜 scroll_text<br/>Texte<br/>Defilant"]
    end

    subgraph P2["🟡 PRIORITE 2<br/>Nice to Have"]
        direction LR
        PLASMA["🎨 plasma<br/>Sinus<br/>Psychedelique"]
        CLOCK["🕐 clock<br/>Horloge<br/>Digitale"]
        SPECTRUM["📊 spectrum<br/>Audio FFT<br/>Visualizer"]
    end

    subgraph P3["🟢 PRIORITE 3<br/>Fun • Bonus"]
        direction LR
        GOL["🧬 gameoflife<br/>Conway<br/>Cellular"]
        SNAKE["🐍 snake<br/>Classic<br/>Arcade"]
    end

    style P1 fill:#7f1d1d,stroke:#ef4444,stroke-width:4px,color:#f1f5f9
    style P2 fill:#713f12,stroke:#f59e0b,stroke-width:4px,color:#f1f5f9
    style P3 fill:#14532d,stroke:#22c55e,stroke-width:4px,color:#f1f5f9
```

| Effet | Priorite | Description |
|:------|:--------:|:------------|
| `fire` | 🔴 P1 | Flammes animees realistes |
| `waves` | 🔴 P1 | Vagues de couleurs fluides |
| `matrix_rain` | 🔴 P1 | Code Matrix tombant |
| `scroll_text` | 🔴 P1 | Texte defilant horizontal |
| `plasma` | 🟡 P2 | Effet plasma psychedelique |
| `clock` | 🟡 P2 | Horloge numerique |
| `spectrum` | 🟡 P2 | Visualiseur audio |
| `gameoflife` | 🟢 P3 | Jeu de la vie Conway |
| `snake` | 🟢 P3 | Jeu Snake classique |

---

## Integrations Futures

```mermaid
%%{init: {'theme': 'base', 'themeVariables': { 'primaryColor': '#8b5cf6', 'primaryTextColor': '#f1f5f9', 'background': '#020617', 'lineColor': '#64748b'}}}%%
flowchart TB
    subgraph EXTERNAL["🌐 Services Externes<br/>Cloud + Automation"]
        direction TB
        ZAPIER["⚡ Zapier<br/>No-code<br/>Workflows"]
        N8N["🔄 n8n<br/>Self-hosted<br/>Automation"]
        GITHUB2["🐙 GitHub<br/>Actions<br/>CI/CD"]
        HA["🏠 Home<br/>Assistant<br/>Domotique"]
        MQTT_SRV["📡 MQTT<br/>Broker<br/>Eclipse Mosquitto"]
    end

    subgraph SENSORS["📦 Capteurs Freenove<br/>GPIO + SPI"]
        direction TB
        PIR["👁️ PIR<br/>HC-SR501<br/>Motion"]
        RFID3["💳 RFID<br/>RC522<br/>13.56MHz"]
        SONIC["📏 Ultrason<br/>HC-SR04<br/>2-400cm"]
    end

    subgraph PANEL_SYS["🎯 Super LED Panel<br/>Core System"]
        direction TB
        API_INT["🐍 FastAPI<br/>REST Server<br/>Port 8000"]
        MCP_INT["🤖 MCP<br/>Server<br/>Claude Integration"]
        MQTT_INT["📡 MQTT<br/>Client<br/>Pub/Sub"]
        GPIO_INT["🔌 GPIO<br/>Handler<br/>RPi.GPIO"]
    end

    subgraph OUTPUT["💡 Affichage<br/>LED Matrix"]
        LED3["Waveshare<br/>P2.5 64×32<br/>2048 LEDs"]
    end

    ZAPIER -->|"HTTP<br/>Webhook"| API_INT
    N8N -->|"HTTP<br/>Webhook"| API_INT
    GITHUB2 -->|"HTTP<br/>Webhook"| API_INT
    HA -->|"REST<br/>API"| API_INT
    MQTT_SRV <-->|"Pub/Sub<br/>QoS"| MQTT_INT

    PIR -->|"GPIO<br/>Digital"| GPIO_INT
    RFID3 -->|"SPI<br/>Bus"| GPIO_INT
    SONIC -->|"GPIO<br/>Trigger/Echo"| GPIO_INT

    API_INT --> LED3
    MCP_INT --> LED3
    MQTT_INT --> LED3
    GPIO_INT --> LED3

    style EXTERNAL fill:#1e1b4b,stroke:#8b5cf6,stroke-width:3px,color:#f1f5f9
    style SENSORS fill:#052e16,stroke:#22c55e,stroke-width:3px,color:#f1f5f9
    style PANEL_SYS fill:#0f172a,stroke:#0ea5e9,stroke-width:3px,color:#f1f5f9
    style OUTPUT fill:#7c2d12,stroke:#f59e0b,stroke-width:3px,color:#f1f5f9
```

| Integration | Protocol | Use Case |
|:------------|:---------|:---------|
| **Webhooks** | HTTP POST | Notifications Zapier, n8n, GitHub |
| **MCP Server** | WebSocket | Controle via Claude Code |
| **Home Assistant** | REST API | Automatisation domotique |
| **MQTT** | Pub/Sub | Messages IoT temps reel |
| **Freenove GPIO** | GPIO/SPI | Capteurs physiques |

---

## Checklist Pre-Commande

### ✅ Panier Valide (333,60€)

- [x] 🍓 Kit Raspberry Pi 5 8GB (B0CRMQCYXH) — **158,90€**
- [x] 💡 Panneau Waveshare P2.5 64×32 (B0BQYDLHY9) — **29,99€**
- [x] 📟 Adaptateur HUB75 Xicoolee (B0BC8Y447G) — **19,90€**
- [x] 🔋 Alimentation LEICKE 5V 10A (B07YVBHH6K) — **29,99€**
- [x] 📦 Kit Freenove 132 projets (B092V1BPBC) — **64,95€**
- [x] 📺 Ecran LCD I2C 16×2 (B0B76YGDV4) — **8,95€**
- [ ] 📖 Livre "The Creative Act" — 20,92€ *(optionnel)*

### ❌ Retires du Panier

- [x] ~~Boitier ElectroCookie~~ — DOUBLON
- [x] ~~Blocs 5V 2A~~ — INSUFFISANT

---

## Ressources

| Type | Lien |
|:-----|:-----|
| 📚 Lib LED | [rpi-rgb-led-matrix](https://github.com/hzeller/rpi-rgb-led-matrix) |
| 📖 Wiki Waveshare | [RGB-Matrix-P2.5-64x32](https://www.waveshare.com/wiki/RGB-Matrix-P2.5-64x32) |
| 🍓 PioMatter Pi5 | [Adafruit Blinka](https://github.com/adafruit/Adafruit_Blinka_Raspberry_Pi5_Piomatter) |
| 🐍 FastAPI | [Documentation](https://fastapi.tiangolo.com/) |
| 📦 Freenove | [GitHub](https://github.com/Freenove/Freenove_Projects_Kit_for_Raspberry_Pi) |

---

> *Document genere le 2025-11-29 par Claude Code*
> *Theme: Solarized Dark + Tailwind CSS Colors*
