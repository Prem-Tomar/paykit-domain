# Release Validation

Run these checks before tagging or publishing `paykit-domain`.

## Required Checks

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo doc --no-deps
cargo package --allow-dirty --list
```

Full package verification with `cargo package --allow-dirty` requires the compatible
`paykit-money` version declared in `Cargo.toml` to be available from the selected package source.

## Package Contract

- The package must include `LICENSE`, `NOTICE`, `CHANGELOG.md`, and `README.md`.
- `Cargo.toml` must keep the approved `Apache-2.0` license identifier.
- The `paykit-money` dependency must keep both local sibling development and the compatible
  version requirement.
- Payment lifecycle behavior must remain independent of transport, persistence, processors,
  ledgers, Serde, REST, and async runtime concerns.
