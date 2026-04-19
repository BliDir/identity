# Identity Service

A multitenant identity service written in Rust, implementing OAuth 2.0 (RFC 6749/6750), OpenID Connect Core 1.0, and RFC-compliant token introspection/revocation. Exposes a fully documented OpenAPI 3.1 spec. Designed for Docker/Kubernetes deployment.

## Stack

- **Language**: Rust (edition 2021, stable toolchain)
- **Web framework**: Axum + Tokio async runtime
- **Database**: PostgreSQL via `sqlx` (compile-time checked queries, migrations)
- **Auth standards**: OAuth 2.0, OpenID Connect Core 1.0
- **Token format**: JWT (RS256) via `jsonwebtoken`; JWKS endpoint for public key discovery
- **OpenAPI**: `utoipa` for spec generation, `utoipa-swagger-ui` for UI
- **Config**: `config` crate + environment variables (12-factor)
- **Observability**: `tracing` + `tracing-subscriber` (JSON in prod, pretty in dev)
- **Containerization**: Docker multi-stage builds, Kubernetes manifests in `/deploy/k8s/`

## Architecture

```
/
├── src/
│   ├── main.rs               # Entrypoint, server bootstrap
│   ├── config.rs             # Config loading (env + file)
│   ├── db.rs                 # DB pool setup
│   ├── error.rs              # Unified AppError -> HTTP response
│   ├── tenant/               # Tenant resolution middleware + model
│   ├── oauth2/               # OAuth 2.0 flows (auth code, client creds, refresh)
│   ├── oidc/                 # OIDC discovery, userinfo, ID token claims
│   ├── token/                # JWT issuance, validation, introspection, revocation
│   ├── jwks/                 # JWKS endpoint + key rotation
│   ├── users/                # User CRUD, password hashing (argon2)
│   ├── clients/              # OAuth client registration + management
│   └── openapi.rs            # Aggregated OpenAPI spec via utoipa
├── migrations/               # sqlx migration files (numbered, sequential)
├── tests/                    # Integration tests (spin up test DB)
├── deploy/
│   ├── Dockerfile
│   └── k8s/                  # Deployment, Service, ConfigMap, Secrets
└── docs/
    └── rfcs.md               # Notes on RFC compliance decisions
```

## Commands

All commands are wrapped in `make`. A `Makefile` must exist at the project root.

- `make build` — compile (debug)
- `make release` — compile (release, used in Docker)
- `make run` — run locally (reads `.env`)
- `make test` — run unit + integration tests
- `make lint` — run `cargo clippy -- -D warnings`
- `make fmt` — run `cargo fmt`
- `make fmt-check` — check formatting without modifying files
- `make migrate` — apply pending DB migrations (`sqlx migrate run`)
- `make migrate-add name=<n>` — create a new migration file
- `make up` — start service + PostgreSQL via Docker Compose
- `make down` — stop Docker Compose stack
- `make check` — run `lint` + `fmt-check` + `test` (CI gate, must pass before committing)
- `make keys` — generate RS256 keypair into `./keys/` (dev helper)

When adding new developer tasks, always add a `make` target first — never instruct to run raw `cargo` or `sqlx` commands directly.

## Environment Variables

Always load from environment; never hardcode secrets. Required vars:

```
DATABASE_URL=postgres://user:pass@localhost:5432/identity
JWT_PRIVATE_KEY_PATH=./keys/private.pem
JWT_PUBLIC_KEY_PATH=./keys/public.pem
BASE_URL=https://auth.example.com
PORT=8080
RUST_LOG=info
```

## Multitenancy

- Every request is resolved to a **tenant** via subdomain or `X-Tenant-ID` header
- Tenant ID is a UUID stored in a `tenant_id` column on all tables
- All DB queries **must** include `WHERE tenant_id = $n` — never query across tenants
- Tenant isolation is enforced at the middleware layer (`src/tenant/`)
- Each tenant has its own: client registry, user pool, token signing keys, OIDC issuer URL

## OAuth 2.0 / OIDC Standards Compliance

Follow these RFCs exactly — do not invent behavior:

- **RFC 6749** — OAuth 2.0 Authorization Framework (auth code + client credentials flows)
- **RFC 6750** — Bearer Token Usage
- **RFC 7009** — Token Revocation (`POST /oauth/revoke`)
- **RFC 7662** — Token Introspection (`POST /oauth/introspect`)
- **RFC 7517** — JSON Web Key (JWKS at `GET /.well-known/jwks.json`)
- **RFC 7519** — JSON Web Tokens (JWT claims structure)
- **RFC 8414** — Authorization Server Metadata (`GET /.well-known/oauth-authorization-server`)
- **OIDC Core 1.0** — Discovery at `GET /.well-known/openid-configuration`, userinfo at `GET /userinfo`

Error responses must use RFC 6749 §5.2 error codes (`invalid_client`, `invalid_grant`, etc.) as JSON.

## OpenAPI

- All routes must have `#[utoipa::path(...)]` annotations
- Schemas derived via `#[derive(ToSchema)]` — do not hand-write schema JSON
- Spec served at `GET /openapi.json`
- Swagger UI served at `GET /docs`
- Keep operationIds stable — they are part of the public API contract

## Code Style

- No `unwrap()` or `expect()` in non-test code — use `?` and `AppError`
- All DB queries use `sqlx::query!` or `sqlx::query_as!` macros (compile-time checked)
- Errors: define variants in `src/error.rs`, implement `IntoResponse` once
- Secrets (keys, passwords) must use `secrecy::Secret<T>` — never log them
- Use `uuid::Uuid` for all IDs — no auto-increment integers as public identifiers
- Timestamps: `chrono::DateTime<Utc>` stored as `TIMESTAMPTZ` in Postgres

## Workflow

1. **Plan before coding** — for new endpoints or flows, outline the RFC section being implemented and the DB schema change before writing any code
2. **Migrations first** — always write and apply the migration before touching Rust model structs
3. **Test RFC edge cases** — error paths (expired tokens, wrong `client_secret`, missing scopes) must have integration tests
4. **Clippy must pass** — run `cargo clippy -- -D warnings` before considering any task done
5. Ask before making changes that affect the public OpenAPI surface or token format

## Important Notes

- NEVER log token values, client secrets, or private keys — use `tracing` spans with redacted fields
- PKCE (`code_challenge` / `code_verifier`) is **required** for authorization code flow — do not make it optional
- Token expiry and clock skew: validate `exp`, `nbf`, `iat` strictly; allow 30s clock skew max
- Refresh tokens must be **rotated** on each use (one-time use); store hashed in DB, not plaintext
- `client_secret` stored as argon2 hash — never store raw secrets
- All endpoints must return `Cache-Control: no-store` on token responses (RFC 6749 §5.1)
- The `/.well-known/openid-configuration` issuer URL must exactly match `BASE_URL` including scheme

## Git & Branch Naming (Gitflow)

Use Gitflow prefixes for all branch names. Any branch may be merged directly into `main`/`master`.

| Branch type | Pattern | Example |
|---|---|---|
| Feature | `feature/<short-description>` | `feature/oidc-userinfo-endpoint` |
| Bug fix | `bugfix/<short-description>` | `bugfix/token-expiry-overflow` |
| Release | `release/<version>` | `release/1.2.0` |
| Hotfix | `hotfix/<short-description>` | `hotfix/jwks-rotation-crash` |
| Chore | `chore/<short-description>` | `chore/update-sqlx-dependency` |

Branch name rules:
- Lowercase, hyphen-separated — no underscores, no extra slashes
- Keep the description short: 2–5 words
- Never commit directly to `main`/`master`