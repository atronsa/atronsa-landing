## Deployment

### Purpose

This document defines how Atronsa components are built and deployed.

### Components Overview

| Component | Runs On | Packaging |
|-----------|---------|-----------|
| Backend API | VPS or Cloud | Docker container |
| Database | VPS or Cloud | Managed or self-hosted PostgreSQL |
| NFC Reader | Raspberry Pi 3B+ | Rust binary (bare metal) |
| Web App | Vercel or Netlify | Static export |
| Admin Desktop | Local machine | Tauri native binary |
| Wallet App | App Stores | Flutter APK/IPA |

### Development

Start everything locally:

```bash
./scripts/dev.sh
```

#### Default ports

Backend: https://localhost:3000

Web: https://localhost:5000

Postgres: localhost:5432

### Production Builds

#### Backend
```bash
docker build -f infrastructure/docker/Dockerfile.backend -t atronsa-api .
docker run -d -p 3000:3000 --env-file .env.production atronsa-api
```
Migrations run automatically on startup.

#### Web App
```bash
cd apps/web
pnpm build
```
Deploy the output directory to Vercel or Netlify.

#### Admin Desktop
```bash
cd apps/admin-desktop
cargo tauri build
```
Output: src-tauri/target/release/bundle/

#### Wallet App
```bash
cd apps/wallet
flutter build apk    # Android
flutter build ios    # iOS
```

#### NFC Reader (Raspberry Pi)
```bash
cd hardware/reader-v1
cargo build --release
```

Copy binary to Pi and run:

```bash
scp target/release/reader-v1 pi@raspberrypi:~
ssh pi@raspberrypi ./reader-v1
```

### Environment Variables

#### Backend
```
DATABASE_URL=postgres://user:pass@host:5432/atronsa
JWT_SECRET=<random-64-char-string>
API_PORT=3000
```

#### NFC Reader
```
BACKEND_URL=http://backend-host:3000
READER_ID=reader-01
READER_SECRET=<pre-shared-key>
```

#### Web App
```
NEXT_PUBLIC_API_URL=https://api.atronsa.com
```

#### Database
Migrations run automatically when the backend starts. No manual step required in production.

For manual migration (development only):

```bash
cd backend
cargo run --bin api -- migrate
```

#### Health Check
Monitor backend health at:

```
GET /api/v1/health
```
Returns 200 if the server and database are reachable.

### Raspberry Pi Setup
- Flash Raspberry Pi OS Lite to SD card
- Enable SPI via raspi-config
- Install dependencies: ./hardware/setup/install-deps.sh
- Wire PN532 HAT following hardware/docs/wiring.md
- Copy reader binary and run

### Rollback
To rollback the backend:

- Stop the running container
- Start the previous image tag
- Verify health check passes

Migrations are forward-only. Test migrations thoroughly before deploying.
