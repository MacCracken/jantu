# Contributing to Jantu

Thank you for your interest in contributing to Jantu.

## Development Setup

```bash
# Clone
git clone https://github.com/MacCracken/jantu.git
cd jantu

# Verify toolchain (stable, MSRV 1.89)
rustup show

# Run the full check suite
make check
```

## Development Process

All changes follow the work loop defined in `CLAUDE.md`:

1. Implement the change
2. Run cleanliness checks: `cargo fmt --check`, `cargo clippy --all-features --all-targets -- -D warnings`, `cargo audit`, `cargo deny check`, `RUSTDOCFLAGS="-D warnings" cargo doc --all-features --no-deps`
3. Add tests (unit + serde roundtrip) and benchmarks for new code
4. Run benchmarks: `./scripts/bench-history.sh`
5. Internal review for correctness, performance, and safety

## Code Standards

- `#[non_exhaustive]` on all public enums
- `#[must_use]` on all pure functions
- All types must implement `Serialize` + `Deserialize`
- Zero `unwrap`/`panic` in library code
- Every type must have serde roundtrip tests
- Every public function must have a doc-test

## Adding a New Module

1. Create `src/module_name.rs` with doc comment explaining the biological basis
2. Register in `src/lib.rs`
3. Add serde roundtrip tests for all types
4. Add criterion benchmarks in `benches/benchmarks.rs`
5. Update `CHANGELOG.md`, `README.md`, and `docs/architecture/overview.md`

## Commit Messages

Use concise messages that focus on *why*, not *what*. Reference biological models or papers where applicable.

## Code of Conduct

This project follows the [Contributor Covenant v2.1](https://www.contributor-covenant.org/version/2/1/code_of_conduct/).
