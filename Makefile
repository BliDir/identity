.PHONY: run test check coverage

run:
	docker compose up --build

test:
	cargo test --all-targets --all-features

check:
	cargo fmt --check
	cargo clippy --all-targets --all-features -- -D warnings
	cargo test --all-targets --all-features

coverage:
	cargo llvm-cov --all-targets --all-features --ignore-filename-regex 'src/main.rs' --fail-under-lines 80
