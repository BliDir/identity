# Identity Service

Identity service for managing identity-related application workflows.

## Layout

```text
.
├── Cargo.toml              # Package metadata and dependency declarations
├── src/
│   ├── lib.rs              # Reusable library API
│   └── main.rs             # Binary entry point
├── tests/                  # Integration tests
├── examples/               # Runnable examples
├── benches/                # Benchmark placeholders
├── .cargo/config.toml      # Local Cargo defaults
├── rustfmt.toml            # Formatting policy
├── clippy.toml             # Lint configuration
├── Dockerfile              # Production-style container build
├── docker-compose.yml      # Local app and PostgreSQL stack
└── .github/workflows/ci.yml
```

## Common Commands

```sh
make help    # Show available local commands
make fmt     # Check Rust formatting
make lint    # Run Clippy with warnings as errors
make test    # Run all tests
make build   # Build the project
make run     # Run the binary
make check   # Run fmt, lint, and test
make doc     # Build documentation
make clean   # Remove build artifacts
make docker-build # Build the Docker image
make docker-up    # Start the app and PostgreSQL in the background
make docker-down  # Stop the app and PostgreSQL
make docker-logs  # Follow Compose logs
make docker-clean # Stop Compose and remove volumes
```

The `Makefile` wraps the equivalent Cargo commands used by CI:

```sh
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

## Docker

The local Docker stack runs the Rust service with PostgreSQL:

```sh
make docker-up
```

Follow container logs with:

```sh
make docker-logs
```

PostgreSQL is exposed on `localhost:5432` with these development credentials:

```text
database: identity
username: identity
password: identity
```

The application container receives:

```text
DATABASE_URL=postgres://identity:identity@postgres:5432/identity
```

The Docker setup pins current stable images:

```text
rust:1.95.0-slim-trixie
postgres:18.3-trixie
debian:trixie-slim
```

The crate declares Rust `1.95.0` as its minimum supported toolchain to match the
current stable Rust image used by Docker.

PostgreSQL 18 stores versioned database data under `/var/lib/postgresql`, so the
Compose volume is mounted there instead of directly at `/var/lib/postgresql/data`.

## Automation

CI runs `make check` and `make build` on every push and pull request.

When changes are merged into `main`, GitHub Actions creates or updates a draft
release using the current semantic Cargo crate version, such as `v0.1.0`.
Published releases are not modified automatically.

## Development

Keep domain logic in `src/lib.rs` or modules under `src/`, and keep `src/main.rs`
small. Integration tests belong in `tests/`, while examples that demonstrate
public API usage belong in `examples/`.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for the full local workflow and pull
request checklist.

Before opening a pull request, run the full local check:

```sh
make check
```

Use focused commits, keep changes scoped to the problem being solved, and add or
update tests when behavior changes.
