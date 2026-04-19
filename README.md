# Identity Service

Identity service for managing identity-related application workflows.

## Prerequisites

- Rust `1.95.0`
- `make`
- Docker and Docker Compose, for running PostgreSQL locally

## Check Locally

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

Run the service and PostgreSQL with Docker Compose:

```sh
make run
```

Press `Ctrl-C` to stop the local stack.

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
