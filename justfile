# Format all files
fmt:
    @cargo fmt --all

# Lint with clippy
lint:
    @cargo clippy --all-targets --all-features -- -D warnings

# Run basic tests
test:
    @cargo test

# Run integration tests (postgresql must be setup and running) WARNING! this clears the database afterwards
itest:
    @diesel migration redo && cargo test --features integration_tests && diesel migration redo

# Clean the project
clean:
    @cargo clean

# Open Docs
doc:
    @cargo doc --open

# Builds a debug version
build-debug:
    @cargo build --all-features

# Builds a release version
build-release:
    @cargo build  --all-features --release

# Formats, lints and builds debug
dev: fmt lint build-debug

# Formats, lints and builds release
release: fmt lint build-release
