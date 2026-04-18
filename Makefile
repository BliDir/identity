.PHONY: help fmt lint test build run check clean doc docker-build docker-up docker-down docker-logs docker-clean

.DEFAULT_GOAL := help

CARGO ?= cargo

help:
	@printf "Available targets:\n"
	@printf "  make fmt     Check Rust formatting\n"
	@printf "  make lint    Run Clippy with warnings as errors\n"
	@printf "  make test    Run all tests\n"
	@printf "  make build   Build the project\n"
	@printf "  make run     Run the binary\n"
	@printf "  make check   Run fmt, lint, and test\n"
	@printf "  make doc     Build documentation\n"
	@printf "  make clean   Remove build artifacts\n"
	@printf "  make docker-build Build the Docker image\n"
	@printf "  make docker-up    Start the app and PostgreSQL\n"
	@printf "  make docker-down  Stop the app and PostgreSQL\n"
	@printf "  make docker-logs  Follow Compose logs\n"
	@printf "  make docker-clean Stop Compose and remove volumes\n"

fmt:
	$(CARGO) fmt --check

lint:
	$(CARGO) clippy --all-targets --all-features -- -D warnings

test:
	$(CARGO) test --all-targets --all-features

build:
	$(CARGO) build --all-targets --all-features

run:
	$(CARGO) run

check: fmt lint test

doc:
	$(CARGO) doc --no-deps --all-features

clean:
	$(CARGO) clean

docker-build:
	docker compose build

docker-up:
	docker compose up --build --detach

docker-down:
	docker compose down

docker-logs:
	docker compose logs -f

docker-clean:
	docker compose down --volumes --remove-orphans
