## Database Conventions

### Purpose

This document defines how Atronsa designs, names, and interacts with the PostgreSQL database.

### Stack

| Component | Technology |
|-----------|------------|
| Engine | PostgreSQL 16 |
| Driver | SQLx (Rust) |
| Migrations | Raw SQL files |
| ORM | None |

### Naming

#### Tables

- Plural: `users`, `cards`, `transactions`
- snake_case: `merchant_accounts`, `payment_methods`
- Junction tables: `user_roles`, `card_transactions`

#### Columns

- snake_case: `created_at`, `user_id`, `amount_cents`
- Primary key: always `id`
- Foreign key: `{table}_id` → `user_id`, `card_id`, `merchant_id`
- Timestamps: `created_at`, `updated_at`

#### Indexes

- Format: `idx_{table}_{column}`
- Examples: `idx_users_email`, `idx_transactions_created_at`
- Foreign key columns always indexed

### Required Columns

Every table must include:

```sql
id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
updated_at    TIMESTAMPTZ NOT NULL DEFAULT now()
```

### Foreign Keys
Always define foreign keys explicitly:

```sql
user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE
```

Specify delete behavior explicitly:
- ON DELETE CASCADE when child records should be removed
- ON DELETE RESTRICT when child records must be deleted first
- ON DELETE SET NULL when child records should become orphaned

### Migrations

#### Rules

- Sequential numbering: 001_users.sql, 002_cards.sql, 003_transactions.sql
- Forward-only: no down migrations
- Never modify an existing migration file
- Each migration does one thing

#### Location
```
infrastructure/postgres/migrations/
├── 001_users.sql
├── 002_cards.sql
├── 003_transactions.sql
└── ...
```

#### Naming Convention
```
{sequence}_{description}.sql

001_users.sql
002_cards.sql
003_transactions.sql
004_add_card_status.sql
005_create_wallets.sql
```

#### No ORM

Use raw SQL with SQLx. No ORM. No query builders that generate SQL.

All queries are written by hand and compile-time checked by SQLx macros.

#### Connection Pool

Single pool created in `crates/core/`. All modules receive a reference
to this pool. No module creates its own connection.
