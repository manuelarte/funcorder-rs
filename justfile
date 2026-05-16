# Run tests
test:
    cargo test

fmt:
    cargo fmt --all -- --check

# Run clippy with all linters
lint:
    cargo clippy --all-targets --all-features -- -W clippy::all
    cargo dylint --all

tools:
    cargo install cargo-dylint dylint-link

# Run both test and lint
check: test lint

