# Identity Service

Multitenant OAuth 2.0 + OpenID Connect identity service in Rust. Follows RFC 6749, 6750, 7009, 7662, 7517, 7519, 8414 and OIDC Core 1.0. OpenAPI 3.1 spec. Docker/Kubernetes deployment.

## Commands

Use `make` for everything — never raw `cargo` or `sqlx` directly.

- `make run` — start locally (reads `.env`)
- `make test` — unit + integration tests
- `make coverage` — generate coverage report (via `cargo-llvm-cov`); minimum 80% required
- `make check` — lint + fmt-check + test + coverage (CI gate)
- `make migrate` — apply DB migrations
- `make migrate-add name=<n>` — new migration file
- `make up` / `make down` — Docker Compose stack
- `make keys` — generate RS256 keypair into `./keys/`

## Environment Variables

```
DATABASE_URL=postgres://user:pass@localhost:5432/identity
JWT_PRIVATE_KEY_PATH=./keys/private.pem
JWT_PUBLIC_KEY_PATH=./keys/public.pem
BASE_URL=https://auth.example.com
PORT=8080
RUST_LOG=info
```

## Multitenancy

All DB queries must include `WHERE tenant_id = $n` — never cross-tenant. Tenant resolved from subdomain or `X-Tenant-ID` header. Each tenant owns its keys, clients, users, and OIDC issuer.

## Hard Rules

- Follow RFCs exactly — do not invent behavior
- PKCE required on auth code flow, no exceptions
- Refresh tokens: rotate on use, store hashed
- `client_secret`: store as argon2 hash
- Secrets use `secrecy::Secret<T>` — never log them (skip in `#[tracing::instrument]`)
- Token responses: `Cache-Control: no-store`
- Issuer URL must exactly match `BASE_URL`
- `make check` must pass before every commit (includes 80% coverage gate)

## Observability

Every request span: `method`, `path`, `tenant_id`, `status`, `latency_ms`.

Prometheus at `GET /metrics` (internal only). Required metrics:

| Metric | Type | Labels |
|---|---|---|
| `http_requests_total` | Counter | `method`, `path`, `status`, `tenant_id` |
| `http_request_duration_seconds` | Histogram | `method`, `path`, `tenant_id` |
| `token_issued_total` | Counter | `grant_type`, `tenant_id` |
| `auth_failure_total` | Counter | `reason`, `tenant_id` |

No user-supplied label values (cardinality).

## Branches

`feature/`, `bugfix/`, `hotfix/`, `release/`, `chore/` — lowercase, hyphenated, 2–5 words. Any branch may merge to `main`. No direct commits to `main`.