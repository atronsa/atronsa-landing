## Backend Architecture

## Purpose

This document defines how the Atronsa backend is structured, how modules communicate, and what rules each layer must follow.

### Stack

| Component | Technology |
|-----------|------------|
| Language | Rust |
| HTTP Framework | Axum |
| Database | PostgreSQL 18 |
| Database Driver | SQLx |
| Authentication | JWT |
| Password Hashing | Argon2id |

### Architecture Pattern

**Vertical Slice Architecture.**

Each business module is self-contained. All code for "payments" lives in `backend/modules/payments/`. No splitting by technical layer across folders.

### Module Structure

Every module follows this exact file layout:
```
modules/<name>/
├── Cargo.toml
└── src/
├── lib.rs          # Re-exports
├── model.rs        # Database entity structs
├── dto.rs          # Request and response shapes
├── repository.rs   # Database queries
├── service.rs      # Business logic
├── handler.rs      # Axum handler functions
└── routes.rs       # Router definition
```
Implement from the inside out: model → dto → repository → service → handler → routes.

### Layer Rules

#### Model
- Database entity structs only
- Fields match database columns exactly
- No business logic, no validation

#### DTO
- Request validation and response serialization structs
- API shape is separate from database shape
- May differ from model fields intentionally

#### Repository
- Database queries only
- Receives pool reference as first argument
- Returns database-level errors only
- Never contains business logic
- Never calls other repositories

#### Service
- All business logic lives here
- Calls repositories for data access
- May call other module services
- Returns `AppError`, never database errors
- Never touches HTTP types directly

#### Handler
- Parses HTTP request (params, query, body)
- Calls the service with extracted data
- Maps result to HTTP response format
- Never contains business logic
- Never accesses database directly

#### Routes
- Defines the module's router
- Maps paths and HTTP methods to handlers


### Cross-Module Communication

When a module needs another module's data, inject that module's service.

Never import another module's repository or handler directly.

### Error Handling

Single `AppError` enum in `crates/core/`. Variants cover:

- `NotFound` — Resource doesn't exist
- `Unauthorized` — Missing or invalid credentials
- `ValidationError` — Input failed validation
- `Conflict` — Duplicate or state conflict
- `Internal` — Unexpected failures

All modules use this same error type. Map database and external errors before returning from services.

### App State

Shared state lives in `crates/api/` and contains:

- Database connection pool
- JWT secret
- Any other global configuration

Modules access state via Axum's `State` extractor in handlers.

### Middleware

Middleware lives in `crates/api/src/middleware/`:

- `auth.rs` — JWT token validation
- Additional middleware added as needed

Auth middleware extracts the user from the token. Handlers receive the authenticated user via a custom extractor, not by parsing tokens manually.
