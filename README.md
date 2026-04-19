# Identity Service

Identity service for managing identity-related application workflows.

## Prerequisites

- Rust `1.95.0`
- `make`
- Docker and Docker Compose, for running PostgreSQL locally

## Build Locally

Build all targets and features:

```sh
make build
```

Run the full local verification gate:

```sh
make check
```

`make check` runs:

```sh
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

## Run Locally

Run the Rust binary directly:

```sh
make run
```

The service listens on port `3000` by default. Check service health with:

```sh
curl http://localhost:3000/health
```

Run the service with PostgreSQL through Docker Compose:

```sh
make docker-up
```

Follow container logs:

```sh
make docker-logs
```

Stop the local stack:

```sh
make docker-down
```

Stop the local stack and remove PostgreSQL data:

```sh
make docker-clean
```

PostgreSQL is exposed locally with these development credentials:

```text
host: localhost
port: 5432
database: identity
username: identity
password: identity
```

The application container receives:

```text
DATABASE_URL=postgres://identity:identity@postgres:5432/identity
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for the full local workflow and pull
request checklist.

Before opening a pull request, run:

```sh
make check
```

Use focused commits, keep changes scoped to the problem being solved, and add or
update tests when behavior changes.
