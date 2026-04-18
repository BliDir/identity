# Contributing

Thanks for taking the time to improve this project.

## Local Checks

Before opening a pull request, run:

```sh
make check
```

This runs formatting, linting, and tests:

```sh
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

## Development Guidelines

- Keep changes focused on one problem at a time.
- Put reusable logic in `src/lib.rs` or modules under `src/`.
- Keep `src/main.rs` small and focused on wiring the binary entry point.
- Add or update tests when behavior changes.
- Use integration tests in `tests/` for public behavior.
- Use examples in `examples/` to demonstrate public API usage.
- Avoid unrelated formatting or refactoring in the same change.

## Pull Request Checklist

- `make check` passes locally.
- New behavior has tests or a clear reason tests are not needed.
- Public API changes are reflected in examples or documentation when useful.
- The change is small enough to review without unrelated edits.
