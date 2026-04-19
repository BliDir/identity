.PHONY: run test check

run:
	docker compose up --build

test:
	cargo test --all-targets --all-features

check:
	cargo fmt --check
	cargo clippy --all-targets --all-features -- -D warnings
	cargo test --all-targets --all-features
