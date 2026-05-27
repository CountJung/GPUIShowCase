# Cargo Patterns Reference

## Workspace Setup (Multi-Crate)

```toml
# workspace/Cargo.toml
[workspace]
resolver = "2"
members = [
    "crates/core",
    "crates/platform",
    "crates/app",
]

# Shared dependency versions — single source of truth
[workspace.dependencies]
anyhow      = "1"
thiserror   = "1"
serde       = { version = "1", features = ["derive"] }
serde_json  = "1"
tokio       = { version = "1", features = ["full"] }
log         = "0.4"
env_logger  = "0.11"

# Workspace metadata for all crates
[workspace.package]
version   = "0.1.0"
edition   = "2021"
rust-version = "1.75"
authors   = ["Your Name <you@example.com>"]
license   = "MIT"
```

```toml
# crates/core/Cargo.toml
[package]
name    = "my-core"
version.workspace    = true
edition.workspace    = true
rust-version.workspace = true

[dependencies]
anyhow    = { workspace = true }
thiserror = { workspace = true }
serde     = { workspace = true }
```

## Feature Flags

```toml
[features]
default  = ["json"]
json     = ["dep:serde_json"]
metrics  = ["dep:prometheus"]
full     = ["json", "metrics"]

[dependencies]
serde_json = { version = "1", optional = true }
prometheus = { version = "0.13", optional = true }
```

```rust
// In code: gate behind feature
#[cfg(feature = "json")]
pub fn to_json<T: serde::Serialize>(val: &T) -> anyhow::Result<String> {
    serde_json::to_string(val).map_err(Into::into)
}
```

## Build Profiles

```toml
# Fast dev builds with some optimisation
[profile.dev]
opt-level     = 1
debug         = true
incremental   = true

# Tests: full debug info for backtraces
[profile.test]
opt-level = 1
debug     = true

# Release: maximum optimisation
[profile.release]
opt-level     = 3
lto           = "thin"      # "fat" for final shipping binary
codegen-units = 1
strip         = "symbols"
panic         = "abort"     # smaller binary, no unwinding overhead

# Profiling: release + debug info
[profile.profiling]
inherits  = "release"
debug     = true
strip     = "none"
```

## Platform-Specific Dependencies

```toml
[target.'cfg(target_os = "windows")'.dependencies]
windows-sys = { version = "0.52", features = [
    "Win32_Foundation",
    "Win32_UI_WindowsAndMessaging",
    "Win32_System_Threading",
] }

[target.'cfg(target_os = "macos")'.dependencies]
core-foundation = "0.9"

[target.'cfg(unix)'.dependencies]
nix = { version = "0.27", features = ["signal"] }
```

## Build Scripts (build.rs)

```rust
// build.rs — runs before compilation
fn main() {
    // Re-run only if these files change
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=assets/");

    // Emit compile-time env var available via env!()
    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", chrono_now());

    // Link a native library
    println!("cargo:rustc-link-lib=static=mylib");
    println!("cargo:rustc-link-search=native=/usr/local/lib");

    // Windows: embed manifest / icon
    #[cfg(target_os = "windows")]
    {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/app.ico");
        res.compile().unwrap();
    }
}
```

## Useful Cargo Commands

```bash
# Check without producing binaries (fastest feedback loop)
cargo check --all-targets --all-features

# Lint (treat warnings as errors in CI)
cargo clippy --all-targets --all-features -- -D warnings

# Format check (CI) / fix (local)
cargo fmt --all --check
cargo fmt --all

# Run a specific example
cargo run --example basic

# Run tests with output
cargo test -- --nocapture

# Expand macros (requires cargo-expand)
cargo expand src/main.rs

# Dependency audit
cargo audit

# Unused dependencies
cargo machete

# Show dependency tree
cargo tree --duplicates    # find duplicate transitive deps
```
