## Testing Strategy

### Purpose

This document defines what to test, how to test it, and what not to test in Atronsa.

### Test Types

| Type | Location | Purpose | Speed |
|------|----------|---------|-------|
| Unit | Inside each module | Test service logic in isolation | Fast |
| Integration | `tests/integration/` | Test API endpoints with real DB | Medium |
| E2E | `tests/e2e/` | Test full user flows across apps | Slow |

### Unit Tests

#### What to test

- Service functions (business logic)
- Validation logic in DTOs
- Error mapping and handling
- Edge cases in calculations

#### What not to test

- Repository functions (tested via integration)
- Handler functions (tested via integration)
- Framework code (Axum routing, middleware plumbing)
- Trivial getters and setters

### Mocking

Mock repositories when testing services. Don't hit a real database in unit tests.

### Integration Tests

#### What to test

- Complete API endpoint behavior
- Request validation at the HTTP boundary
- Authentication and authorization
- Database interactions with real test database
- Response format matches API conventions

#### What not to test

- External services (mock the notification service, payment gateway)
- NFC hardware (mock the reader HTTP calls)
- Third-party APIs

#### Setup

Use a separate test database:

```ATRONSA_DATABASE_URL=postgres://atronsa:password@localhost:5432/atronsa_test```

Run migrations before tests. Each test seeds its own data and cleans up after itself.

#### Tools

- `reqwest` for making HTTP requests to the test server
- `sqlx` for seeding test data
- Test server started per test or per test suite

### E2E Tests

#### What to test

- Critical user journeys only:
  - User registers and makes a payment
  - Card is linked and used for tap-to-pay
  - Wallet balance updates correctly

#### When to write

Only after the feature is stable. E2E tests are expensive to maintain.
Write them for payment flows, not for profile page rendering.

### Coverage Targets

| Module | Target |
|--------|--------|
| Payments | 90%+ |
| Auth | 80%+ |
| Wallets | 80%+ |
| Other modules | 70%+ |

Don't obsess over 100%. Test behavior, not lines of code.

### What to Always Test

- Payment processing (credit, debit, balance never negative)
- Card linking and unlinking
- Authentication (login success, login failure, token refresh)
- Balance calculations (precision, rounding, zero edge case)
- Input validation (invalid email, missing required fields)

### What to Never Test

- Framework internals (Axum, SQLx, serde)
- Standard library functionality
- External services (mock them instead)
- Database queries directly (test through service or integration)

### Running Tests

```bash
# Unit tests
cargo test

# Integration tests
cargo test --test '*'

# Specific test
cargo test test_create_payment_success

# With output
cargo test -- --nocapture
```
