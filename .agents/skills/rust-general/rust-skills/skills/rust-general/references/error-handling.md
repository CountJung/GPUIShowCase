# Error Handling Reference

## Decision Tree

```
Is this a library crate?
├─ YES → thiserror: define typed enum, each variant = one failure mode
└─ NO (binary/application)
   ├─ Simple script → anyhow::Result<()> everywhere, done
   └─ Complex app  → thiserror for domain errors + anyhow at boundaries
```

## thiserror — Complete Patterns

```rust
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    // Simple message
    #[error("configuration is empty")]
    Empty,

    // With field interpolation
    #[error("key `{key}` not found in config")]
    MissingKey { key: String },

    // Wrapping another error — preserves source chain
    #[error("failed to read file `{path}`")]
    ReadFailed {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    // #[from] auto-implements From<X> for this variant
    #[error("JSON parse error")]
    Json(#[from] serde_json::Error),

    // #[transparent] delegates both Display and source to the inner error
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
```

**When to use `#[from]` vs `#[source]`**:
- `#[from]`: when you want automatic `From` conversion AND the variant has
  only that one field.
- `#[source]`: when the variant has additional context fields alongside the
  wrapped error. Implement `From` manually if needed.

## anyhow — Context Chaining

The goal is an error chain that reads like a stack trace in English:

```
Error: loading application
  caused by: reading config from /etc/app/config.json
  caused by: No such file or directory (os error 2)
```

```rust
use anyhow::{bail, ensure, Context, Result};

fn load_app() -> Result<App> {
    let config = load_config(Path::new("/etc/app/config.json"))
        .context("reading config")?;

    let db = connect_db(&config.database_url)
        .with_context(|| format!("connecting to {}", config.database_url))?;

    Ok(App { config, db })
}

fn load_config(path: &Path) -> Result<Config> {
    // context() is lazy only with with_context(); use it for allocating strings
    let text = fs::read_to_string(path)
        .with_context(|| format!("reading config from {}", path.display()))?;

    serde_json::from_str(&text).context("parsing config JSON")
}

fn validate(value: i32) -> Result<()> {
    // ensure! = if !cond { bail!(...) }
    ensure!(value > 0, "value must be positive, got {value}");

    // bail! = return Err(anyhow!(...))
    if value > 1000 {
        bail!("value {value} exceeds maximum of 1000");
    }
    Ok(())
}
```

## Converting Between Error Types at Boundaries

```rust
// Library returns typed ConfigError; app converts to anyhow at the call site
fn main() -> anyhow::Result<()> {
    let config = load_config()
        .map_err(|e| anyhow::anyhow!("config error: {e}"))?;
    // OR simply — anyhow implements From<E: std::error::Error>:
    let config = load_config()?;
    Ok(())
}
```

## Error Reporting to Users vs Developers

```rust
use anyhow::Result;

fn run() -> Result<()> {
    // ... work ...
    Ok(())
}

fn main() {
    // For CLI tools: pretty-print the chain
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        for cause in e.chain().skip(1) {
            eprintln!("  caused by: {cause}");
        }
        std::process::exit(1);
    }
}
```

## Anti-patterns to Avoid

```rust
// ❌ Never: silently discarding errors
let _ = do_something();

// ❌ Never: unwrap outside tests
let val = risky_op().unwrap();

// ❌ Never: string-typed errors in library APIs
fn load() -> Result<Config, String> { ... }

// ❌ Avoid: catching and rethrowing without context
fn load() -> Result<Config> {
    do_thing().map_err(|e| anyhow!("{e}"))?; // loses the chain
    // Better:
    do_thing().context("loading thing")?;
    Ok(Config::default())
}

// ✅ Correct patterns shown above
```
