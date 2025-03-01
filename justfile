# Format all files
fmt:
    @cargo fmt --all

# Lint with clippy
lint:
    @cargo clippy --all-targets --all-features -- -D warnings

test:
    @cargo test --workspace

# Clean the project
clean:
    @cargo clean

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
