## API Conventions

### Purpose

This document defines how Atronsa APIs are designed, structured, and formatted. Every endpoint follows these rules.

### Base URL

All endpoints are prefixed with: `/api/v1/`

### URL Structure

#### Pattern
`/api/v1/{module}/{resource}`

#### Examples
```
GET /api/v1/users
GET /api/v1/users/:id
POST /api/v1/users
PATCH /api/v1/users/:id
DELETE /api/v1/users/:id

POST /api/v1/auth/login
POST /api/v1/auth/register
POST /api/v1/auth/refresh

POST /api/v1/payments
GET /api/v1/payments/:id
GET /api/v1/payments
```

### URL Rules

- Use plural nouns: `/users`, `/cards`, `/payments`
- Actions are HTTP methods, not URL verbs
- Use kebab-case for multi-word resources: `/merchant-accounts`
- Keep nesting to one level: `/users/:id/cards` is fine
- Avoid deep nesting: `/users/:id/cards/:id/transactions` is too deep

### Request Format

All requests use JSON:

Content-Type: application/json

### Example

```json
POST /api/v1/users

{
    "email": "user@example.com",
    "name": "John Doe",
    "phone": "+251911234567"
}
Response Format
Single Item Success
json
{
    "success": true,
    "data": {
        "id": "abc-123",
        "email": "user@example.com"
    }
}
List Success
json
{
    "success": true,
    "data": [
        { "id": "abc-123" },
        { "id": "def-456" }
    ],
    "meta": {
        "total": 100,
        "page": 1,
        "per_page": 20
    }
}
Error
json
{
    "success": false,
    "message": "User not found",
    "error_code": "USER_NOT_FOUND"
}
```
