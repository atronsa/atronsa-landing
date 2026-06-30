## Engineering Principles

### Purpose

This document defines the engineering principles used throughout Atronsa.
These principles guide every technical decision, code review, and architectural choice.

When in doubt, return here.


### Principles

#### 1. Simple over clever

Write code a tired person can understand at 3 AM.

If two solutions work, pick the one with fewer moving parts. Save cleverness
for problems that actually need it.

#### 2. Vertical slices

All code for a feature lives in one module folder.

Models, handlers, services, and repositories sit together. When you need to
change "payments," you open one folder not five.

#### 3. Business logic never lives in handlers

Handlers parse input, call the service, and return a response. Nothing more.

Services contain all business rules. This keeps logic testable without HTTP.

#### 4. Database access only through repositories

Only repository files touch SQL. Services call repositories. Handlers never
touch the database directly.

#### 5. Validate at the boundary

Every API input is validated before it enters the system. Invalid data should
never reach business logic.

#### 6. Fail loudly

Don't swallow errors silently. Log them. Return clear error messages.

A bug you don't know about is worse than a bug you see in logs.

#### 7. Don't build abstractions for one implementation

Wait for the second use case before extracting shared code.

One implementation = inline it. Two implementations = consider it. Three = extract it.

#### 8. Hardware code stays in `hardware/`

NFC reader logic, SPI communication, and GPIO code live only in `hardware/`.

Applications never import hardware dependencies directly. The reader
communicates via HTTP to the backend.

#### 9. Security is not optional

- Hash passwords
- Validate tokens
- Log payments
- Never trust the client
- Never commit secrets

Security is a default requirement, not an afterthought.

#### 10. Document decisions

Write down why you chose something, not just what you chose.

Six months from now, you'll need to know why the reader is a separate binary,
not a library.


### Application

These principles are not suggestions. They are the standard.

If you find yourself violating one, either:
- You have a very good reason (document it in `decisions-log.md`)
- You're taking a shortcut (don't)

Code reviews check for principle violations before anything else.