## Git Workflow

### Purpose

This document defines how Atronsa uses Git for version control.

### Branches

#### Main Branch

`main` is the single source of truth. It is always deployable. Never commit directly to `main`.

#### Feature Branches

Create from `main`. Merge back to `main` when complete.

#### Branch Naming
```
feat/{module}/{description}
fix/{description}
docs/{description}
chore/{description}
```

#### Examples
```
feat/cards/link-card
feat/payments/tap-to-pay
feat/wallet/balance-topup
fix/auth/refresh-token-401
fix/nfc-reader/timeout
docs/api/payment-endpoints
chore/deps/update-sqlx
```

### Commits

#### Format
type(scope): description

#### Types

| Type | When to Use |
|------|-------------|
| `feat` | New feature or functionality |
| `fix` | Bug fix |
| `refactor` | Code change, no feature or fix |
| `docs` | Documentation only |
| `test` | Adding or updating tests |
| `chore` | Build, CI, dependencies, config |
| `style` | Formatting, no logic change |

#### Scope

Use the module or app name: `auth`, `payments`, `cards`, `web`, `wallet`.

#### Examples
```
feat(cards): add card linking endpoint
feat(payments): process tap-to-pay transaction
fix(auth): refresh token returns 401 after expiry
refactor(wallets): extract balance calculation to service
docs(api): document payment endpoints
test(payments): add integration tests for payment flow
chore(deps): update sqlx to 0.8
```

#### Rules

- Use imperative mood: "add" not "added"
- Keep the first line under 72 characters
- Separate subject from body with blank line if more detail is needed
- Commit frequently, squash before merging

### Pull Requests

#### Rules

- One feature or fix per PR
- Must pass CI (lint, test, build) before merging
- Squash merge to `main` (one commit per PR)
- Delete branch after merging

#### PR Title

Use the same format as commits:
feat(payments): process tap-to-pay transaction

### What Not to Commit

- `.env` files
- IDE config (`.vscode/`, `.idea/`)
- Binary files and build artifacts
- Database dumps
- Credentials, API keys, secrets
- Node modules, Rust target directory

Keep these in `.gitignore`.

### Git Hygiene

- Pull before starting new work
- Keep branches short-lived (hours, not weeks)
- Delete merged branches
- Don't commit commented-out code—delete it
- Don't commit debug prints or console logs
- Rebase on `main` before creating PR if there are conflicts

### .gitignore Essentials
```  
.env
.env.local
.env.*.local
/target
node_modules
.vscode
.idea
*.log
dist
build
```
