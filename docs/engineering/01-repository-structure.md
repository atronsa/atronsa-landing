## Repository Structure

### Purpose

This document maps the Atronsa monorepo. Anyone should understand where everything lives within five minutes.

### Top-Level
```
atronsa/
├── apps/                 # User-facing applications
├── backend/              # Rust API server
├── hardware/             # NFC reader daemon + setup
├── infrastructure/       # Docker, Postgres, deployment configs
├── tests/                # E2E and integration tests
├── docs/                 # All documentation
├── scripts/              # Development and build scripts
├── Cargo.toml            # Rust workspace root
├── pnpm-workspace.yaml
└── README.md
```

#### `apps/` — Applications
```
apps/
├── admin-desktop/        # Tauri — Admin panel for merchant and card management
├── landing-website/      # Next.js — Landing page
└── mobile-wallet/        # Flutter — Customer wallet mobile app
```
Each app is self-contained with its own build system and dependencies.

#### `backend/` — API Server
```
backend/
├── Cargo.toml
├── crates/
│ ├── api/                # HTTP server, router, middleware, entry point
│ └── core/               # Shared kernel: DB pool, auth utils, error types
└── modules/
├── auth/                 # Login, register, token management
├── users/                # User profiles and accounts
├── cards/                # Physical NFC cards + virtual cards
├── wallets/              # Balances, top-ups
├── payments/             # Payment processing
├── merchants/            # Merchant accounts
└── notifications/        # Push, email, SMS
```
##### Module Rules

- Each module is a Rust crate
- Each module owns its model, dto, service, handler, repository, routes
- Modules communicate through services, never by importing handlers
- No module imports another module's repository directly

### `hardware/` — NFC Reader
```
hardware/
├── reader-v1/            # PN532 daemon (runs on Raspberry Pi)
├── setup/                # Pi setup scripts
└── docs/                 # Wiring diagrams, pinout references
```
#### Key Rule

Hardware code never appears in `apps/` or `backend/`.

The reader communicates with the backend via HTTP. Applications never
import SPI, GPIO, or PN532 dependencies directly.

### `infrastructure/` — Ops
```
infrastructure/
├── docker/
│ ├── compose.dev.yml
│ └── Dockerfile.backend
└── postgres/
│ └── migrations/
```

### `tests/` — Testing
```
tests/
├── e2e/                  # Full user flows
└── integration/          # API endpoint tests
```

### `docs/` — Documentation
```
docs/
├── engineering/          # This handbook
├── architecture.md       # System design overview
├── hardware.md           # Physical setup guide
├── api.md                # API reference
└── roadmap.md            # Product direction
```

Don't create structure you don't need yet.