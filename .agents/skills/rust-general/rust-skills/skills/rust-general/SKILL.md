---
name: rust-general
description: >
  Production-grade Rust coding skill for agentic tasks. Use this skill whenever
  the user asks to write, refactor, debug, or review any Rust code — including
  CLI tools, libraries, async servers, data pipelines, FFI bindings, or workspace
  projects. Triggers on any mention of Rust, Cargo, crate, tokio, serde, anyhow,
  trait, lifetime, borrow checker, or .rs files. Apply even for short Rust
  snippets or questions about Rust idioms — this skill ensures idiomatic,
  compiler-happy, production-ready output every time.
---

# Rust General — Agent Coding Skill

You are an expert Rust engineer. Write idiomatic, safe, performant Rust that
compiles on the first attempt. Every piece of code you produce must meet the
standards in this skill before being shown to the user.

Read the relevant reference file before writing non-trivial code:
- Project setup / Cargo → `references/cargo-patterns.md`
- Error handling → `references/error-handling.md`
- Async / tokio → `references/async-patterns.md`
- Traits & generics → `references/traits-generics.md`
- Testing → `references/testing.md`

---

## Core Principles

### 1. Make the compiler your ally
- Resolve all `cargo check` errors and `cargo clippy -- -D warnings` warnings
  before presenting code.
- Prefer the type system over runtime checks: encode invariants in types, not
  in `assert!` calls scattered through business logic.
- `unwrap()` and `expect()` are **only** acceptable in tests and in `main()`
  for unrecoverable startup failures. Everywhere else: propagate with `?`.

### 2. Ownership-first design
Think about ownership before writing a single line:

| Question | Answer → |
|----------|----------|
| Who owns this value? | The function that creates it, unless moved |
| Does the callee need to own it? | Pass by value |
| Does the callee just read it? | Pass `&T` |
| Does the callee need to mutate? | Pass `&mut T` |
| Multiple owners needed? | `Arc<T>` (thread-safe) or `Rc<T>` (single-thread) |
| Interior mutability needed? | `Mutex<T>` / `RwLock<T>` / `Cell<T>` / `RefCell<T>` |

Never reach for `clone()` as a first resort to satisfy the borrow checker.
Cloning is fine for small `Copy`-able data; for large data, reconsider ownership.

### 3. Idiomatic Rust over clever Rust
- Use iterators and combinators (`map`, `filter`, `fold`, `chain`, `flat_map`)
  instead of manual loops when it improves clarity.
- `match` exhaustively; never use `_ =>` as a lazy catch-all when the compiler
  would warn about a missing variant.
- Prefer `Option` / `Result` combinators (`map`, `and_then`, `unwrap_or_else`,
  `ok_or_else`) over nested `if let` / `match` chains.
- Name things clearly: `snake_case` for values/functions, `PascalCase` for
  types, `SCREAMING_SNAKE_CASE` for constants.

### 4. Zero-cost abstractions — use them
- Prefer generics with trait bounds over `dyn Trait` for hot paths.
- Use `impl Trait` in function signatures to avoid unnecessary boxing.
- Reserve `Box<dyn Trait>` for heterogeneous collections or when you need
  runtime polymorphism across an API boundary.

---

## Project Structure

Follow this layout for every new project/crate:

```
my-project/
├── Cargo.toml
├── Cargo.lock          # commit this for binaries, .gitignore for libraries
├── src/
│   ├── main.rs         # thin: parse args, call lib, handle top-level errors
│   ├── lib.rs          # all real logic lives here, exposed as a library
│   ├── error.rs        # unified error types (thiserror)
│   ├── config.rs       # configuration structs (serde + clap or envy)
│   └── <module>/
│       ├── mod.rs
│       └── *.rs
├── tests/
│   └── integration_test.rs
├── benches/
│   └── bench.rs        # criterion, only if perf matters
└── examples/
    └── basic.rs
```

**Workspace** (multiple crates):
```
workspace/
├── Cargo.toml          # [workspace] members = [...]
├── crates/
│   ├── core/           # shared types, traits, errors
│   ├── platform/       # OS-specific implementations
│   └── app/            # binary crate, thin shell
└── Cargo.lock
```

---

## Mandatory Cargo.toml Practices

```toml
[package]
name = "my-crate"
version = "0.1.0"
edition = "2021"        # always 2021
rust-version = "1.75"   # set MSRV explicitly

[dependencies]
# Pin major versions; use workspace deps in workspaces
anyhow  = "1"           # app-level error handling
log     = "0.4"         # logging facade (not a backend)

[dev-dependencies]
# Keep test-only deps here, never in [dependencies]

[profile.release]
opt-level     = 3
lto           = "thin"  # "fat" only for final prod binaries
codegen-units = 1
strip         = "symbols"

[profile.dev]
opt-level = 1           # faster dev builds without full debug overhead
```

Workspace-level shared deps:
```toml
# workspace Cargo.toml
[workspace.dependencies]
tokio    = { version = "1", features = ["full"] }
serde    = { version = "1", features = ["derive"] }
anyhow   = "1"
thiserror = "1"

# crate Cargo.toml
[dependencies]
tokio     = { workspace = true }
serde     = { workspace = true }
```

---

## Error Handling — Quick Reference

**Library crates**: use `thiserror` — typed, descriptive, implementable.
```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("config file not found: {path}")]
    ConfigNotFound { path: PathBuf },

    #[error("network request failed")]
    Network(#[from] reqwest::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
```

**Application / binary crates**: use `anyhow` — ergonomic context chaining.
```rust
use anyhow::{Context, Result};

fn load_config(path: &Path) -> Result<Config> {
    let text = fs::read_to_string(path)
        .with_context(|| format!("reading config from {}", path.display()))?;
    serde_json::from_str(&text)
        .with_context(|| "parsing config JSON")
}
```

Full guide → `references/error-handling.md`

---

## Async — Quick Reference

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Build a runtime explicitly for anything beyond simple scripts:
    // let rt = tokio::runtime::Builder::new_multi_thread()
    //     .worker_threads(4)
    //     .enable_all()
    //     .build()?;

    run().await
}

async fn run() -> anyhow::Result<()> {
    // Fan-out concurrent tasks:
    let (a, b) = tokio::try_join!(task_a(), task_b())?;

    // Spawn independent tasks:
    let handle = tokio::spawn(async move { heavy_work().await });
    let result = handle.await??;  // outer ? = JoinError, inner ? = task error

    Ok(())
}
```

Rules:
- Never call blocking code inside an async fn — use `tokio::task::spawn_blocking`.
- Share state between tasks with `Arc<Mutex<T>>`, not global statics.
- Prefer channels (`tokio::sync::mpsc`) over shared mutable state where possible.

Full guide → `references/async-patterns.md`

---

## Clippy Configuration

Add to your project root:

```toml
# .clippy.toml or clippy.toml
msrv = "1.75"
```

Add to `src/lib.rs` or `src/main.rs`:
```rust
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    missing_docs,           // for library crates
)]
#![allow(
    clippy::module_name_repetitions,  // common false positive
    clippy::must_use_candidate,       // noisy for builders
)]
```

Run before every commit:
```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
```

---

## Common Patterns

### Builder pattern
```rust
#[derive(Default)]
pub struct ClientBuilder {
    timeout: Option<Duration>,
    retries: u32,
    base_url: String,
}

impl ClientBuilder {
    pub fn timeout(mut self, d: Duration) -> Self { self.timeout = Some(d); self }
    pub fn retries(mut self, n: u32) -> Self { self.retries = n; self }
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into(); self
    }
    pub fn build(self) -> anyhow::Result<Client> {
        anyhow::ensure!(!self.base_url.is_empty(), "base_url must be set");
        Ok(Client { timeout: self.timeout, retries: self.retries, base_url: self.base_url })
    }
}
```

### Newtype for domain safety
```rust
// Prevent mixing up IDs of different types at compile time
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserId(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrderId(u64);
// UserId and OrderId are now incompatible — the compiler enforces it
```

### Config from environment
```rust
#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
    #[serde(default = "default_workers")]
    pub workers: usize,
}

fn default_workers() -> usize { num_cpus::get() }

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        envy::from_env().context("loading config from environment")
    }
}
```

---

## Code Review Checklist

Before showing any code to the user, verify:

- [ ] `cargo check` passes (mentally or actually)
- [ ] No `unwrap()`/`expect()` outside tests or startup
- [ ] All `Result`/`Option` are handled — no silent ignoring
- [ ] No unnecessary `clone()` — ownership is deliberate
- [ ] Public API has doc comments (`///`)
- [ ] Error messages are lowercase, no trailing period (Rust convention)
- [ ] Lifetimes are explicit only when the compiler can't infer them
- [ ] Async functions don't call blocking code without `spawn_blocking`
- [ ] Sensitive data (passwords, keys) uses types that `impl Drop` to zeroize
