## Coding Standards

### Purpose

This document defines naming, formatting, and style rules across all languages in Atronsa.

### Rust (Backend, Hardware)

#### Naming

| Element | Convention | Example |
|---------|-----------|---------|
| Files | snake_case | `payment_service.rs` |
| Structs/Enums | PascalCase | `CreatePaymentDto` |
| Functions/Methods | snake_case | `process_payment()` |
| Constants | UPPER_SNAKE_CASE | `MAX_RETRY_COUNT` |
| Variables | snake_case | `payment_id` |
| Modules | Plural nouns | `users`, `payments` |

### Imports

Order imports in three blocks separated by blank lines:

1. std library
2. External crates
3. Internal modules

### Errors

Use `AppError` from `crates/core/`. Don't create per-module error types.

Map external errors at the boundary:
- Repository returns `sqlx::Error`
- Service maps to `AppError`
- Handler returns `AppError` as HTTP response

### Database Access

Repository functions take `&PgPool` as first parameter.
Return `Result<T, sqlx::Error>` from repositories.
Services handle mapping to `AppError`.

### TypeScript (Web App)

### Naming

| Element | Convention | Example |
|---------|-----------|---------|
| Files | kebab-case | `payment-form.tsx` |
| Components | PascalCase | `PaymentForm` |
| Functions | camelCase | `formatCurrency()` |
| Variables | camelCase | `transactionId` |
| Constants | UPPER_SNAKE_CASE | `API_BASE_URL` |
| Types/Interfaces | PascalCase | `PaymentResponse` |

#### API Client

Define types where they are used. Don't create a shared types package until duplication appears in at least two apps.

### Dart (Flutter Wallet)

#### Naming

| Element | Convention | Example |
|---------|-----------|---------|
| Files | snake_case | `payment_screen.dart` |
| Classes | PascalCase | `PaymentScreen` |
| Functions | camelCase | `processPayment()` |
| Variables | camelCase | `transactionId` |
| Constants | camelCase | `apiBaseUrl` |

#### Project Structure
```
lib/
├── features/ # One folder per feature
│ ├── cards/
│ ├── wallet/
│ └── payments/
└── core/ # Shared utilities
├── api_client.dart
└── theme.dart
```

### General Rules (All Languages)

#### Comments

Comment why, not what.

- **Good:** "Compensate for PN532 off-by-one bug in UID length"
- **Bad:** "Increment the counter"

No TODO comments without a ticket reference.

#### Logging

Use structured logging with levels:

| Level | When |
|-------|------|
| error | Something broke, needs attention |
| warn | Unexpected but handled |
| info | Significant events (payment, login) |
| debug | Useful during development |

Never log: passwords, tokens, full card UIDs, PINs.

#### Formatting

- Rust: `cargo fmt`
- TypeScript: Prettier
- Dart: `dart format`

Format on save. Never commit unformatted code.

#### Dead Code

Delete it. Don't comment it out. Git remembers.

#### File Length

Keep files under 300 lines. If longer, split into smaller files within the same module.

#### Function Length

Keep functions under 30 lines. If longer, extract helper functions.

### Language-Specific Rules

#### Rust

- Use `#[derive]` for boilerplate (Serialize, Deserialize, FromRow)
- Prefer `&str` over `String` for function parameters
- Use `Option<T>` for nullable values, avoid sentinel values
- Use `.map_err()` to convert errors at boundaries

#### TypeScript

- Prefer `interface` over `type` for object shapes
- Use optional chaining (`?.`) for nested access
- Avoid `any`—use `unknown` and narrow the type

#### Dart

- Use `final` by default, `var` only when reassigning
- Prefer named parameters for functions with 3+ arguments
- Use `const` constructors when possible
