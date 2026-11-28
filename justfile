# Run tests
test:
    DYLINT_RUSTFLAGS="-D warnings" cargo test

fmt:
    cargo fmt --all -- --check

# Run clippy with all linters
lint:
    cargo clippy --all-targets --all-features -- -W clippy::all
    cargo dylint --all

tools:
    cargo install --locked --version 6.4.0 cargo-dylint dylint-link

# Run both test and lint
check: test lint
