## Security Guidelines

### Purpose

This document defines the security standards for Atronsa. These rules are non-negotiable.

### Authentication

#### Password Handling

- Hash all passwords with Argon2id
- Never store plaintext passwords
- Never log passwords or password hashes
- Minimum password length: 8 characters

#### Tokens

- JWT access tokens expire after 24 hours
- Refresh tokens expire after 30 days
- Store refresh tokens securely on client
- Invalidate tokens on password change

#### Rate Limiting

- Login: 5 attempts per minute per IP
- Account locked for 30 minutes after 10 failed attempts
- Rate limit on all auth endpoints

### API Security

#### Authentication Requirements

Every endpoint requires authentication except:
- Login
- Register
- Health check

Return 401 for missing tokens, 403 for insufficient permissions.

#### Token Format

Authorization: Bearer <token>

Validate tokens in middleware. Never validate tokens manually in handlers.

### Data Protection

#### What Never to Expose

- Password hashes in API responses
- Internal database IDs (use UUIDs already)
- Full card UIDs in logs or responses
- PINs or security codes

#### Logging Rules

- Log every payment attempt (success and failure)
- Mask card UIDs in logs: show first 4 characters only
- Never log: passwords, tokens, full card UIDs, PINs, security codes

#### Sensitive Data at Rest

If storing card numbers or sensitive data in the future:
- Encrypt before storage
- Use separate encryption key from application secrets
- Rotate encryption keys regularly

### Input Validation

#### Rules

- Validate every input at the API boundary
- Reject unknown fields in request bodies
- Trim whitespace from strings
- Enforce maximum lengths
- Validate formats (email, phone, UUID)
- Reject invalid data before it reaches business logic

#### Common Validations

| Field | Rule |
|-------|------|
| Email | Valid format, max 255 chars |
| Phone | Valid format, max 20 chars |
| Name | Max 255 chars, not empty |
| UUID | Valid UUID format |
| Amount | Positive integer, max value defined per currency |

### Payment Security

#### Critical Rules

- Balance updates must be atomic (use database transactions)
- Never allow negative balance from race conditions
- Idempotency key for payment creation (prevent double charges)
- Card UID alone is not authentication—require PIN for wallet access
- Log every payment with: amount, timestamp, status, initiator

#### Idempotency

Clients send an idempotency key with payment requests:
Idempotency-Key: <unique-uuid>

Backend rejects duplicate keys within 24 hours.

### Hardware Security

#### Reader Authentication

- Reader authenticates to backend with pre-shared key
- Each reader has a registered ID
- Backend rejects unknown reader IDs
- Reader communication over HTTPS in production

#### Physical Security

- Reader runs on dedicated Pi with no other services
- Pi OS is minimal (Raspberry Pi OS Lite)
- SSH access restricted to local network
- Default passwords changed on first boot

### Dependencies

#### Before Every Release

- Run `cargo audit` to check for vulnerabilities
- Review direct dependency changes
- Pin all dependency versions (no wildcards)

### Secrets Management

#### Rules

- Never commit secrets to Git
- Use `.env` for local development
- Use `.env.example` with placeholder values for documentation
- Production secrets via environment variables or secret manager
- Rotate JWT secret every 90 days

#### What is a Secret

- Database URLs and credentials
- JWT signing keys
- API keys
- Reader pre-shared keys
- Encryption keys
- Any third-party service credentials

### Pre-Release Checklist

Before deploying to production:

- [ ] `cargo audit` passes with no critical vulnerabilities
- [ ] No secrets in committed code
- [ ] All endpoints require auth (except public ones)
- [ ] Passwords hashed with Argon2id
- [ ] Input validation on all endpoints
- [ ] Payment operations use transactions
- [ ] Rate limiting enabled on login
- [ ] Logging rules followed (no sensitive data in logs)
