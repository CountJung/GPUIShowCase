# Testing Reference

## Unit Tests

```rust
// Place unit tests in the same file as the code they test
// This gives access to private functions
pub fn add(a: i32, b: i32) -> i32 { a + b }

fn internal_helper(x: i32) -> i32 { x * 2 }

#[cfg(test)]
mod tests {
    use super::*;  // imports both pub and private items

    #[test]
    fn test_add_positive() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-1, -2), -3);
    }

    #[test]
    fn test_internal_helper() {
        assert_eq!(internal_helper(5), 10); // can access private fn
    }

    #[test]
    #[should_panic(expected = "overflow")]
    fn test_panics_on_overflow() {
        let _ = i32::MAX.checked_add(1).expect("overflow");
    }

    // Async tests
    #[tokio::test]
    async fn test_async_fetch() {
        let result = fetch_data("http://example.com").await;
        assert!(result.is_ok());
    }
}
```

## Integration Tests (tests/)

```rust
// tests/integration_test.rs — tests the public API as an external crate would
use my_crate::{Config, Client};

#[test]
fn test_client_builds_with_valid_config() {
    let config = Config::builder()
        .base_url("http://localhost:8080")
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .expect("valid config should build");

    assert_eq!(config.base_url(), "http://localhost:8080");
}
```

## Test Organisation Patterns

```rust
// Group related tests with descriptive submodules
#[cfg(test)]
mod config_tests {
    use super::*;

    mod validation {
        use super::*;

        #[test]
        fn rejects_empty_url() { ... }

        #[test]
        fn rejects_negative_timeout() { ... }
    }

    mod serialization {
        use super::*;

        #[test]
        fn roundtrips_json() { ... }
    }
}
```

## Test Helpers and Fixtures

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::OnceLock;

    // Expensive setup done once
    fn test_config() -> &'static Config {
        static CONFIG: OnceLock<Config> = OnceLock::new();
        CONFIG.get_or_init(|| Config {
            base_url: "http://localhost".into(),
            timeout: Duration::from_secs(1),
        })
    }

    // Temp dir that auto-cleans on drop
    fn temp_dir() -> tempfile::TempDir {
        tempfile::tempdir().expect("temp dir")
    }

    #[test]
    fn test_with_fixture() {
        let dir = temp_dir();
        let path = dir.path().join("test.json");
        std::fs::write(&path, r#"{"key": "value"}"#).unwrap();
        let result = load_from_file(&path).unwrap();
        assert_eq!(result.key, "value");
        // dir drops here → deleted automatically
    }
}
```

## Property-Based Testing (proptest)

```rust
// Cargo.toml: proptest = { version = "1", default-features = false, features = ["std"] }
use proptest::prelude::*;

proptest! {
    #[test]
    fn parse_roundtrip(s in "[a-zA-Z0-9]{1,64}") {
        let encoded = encode(&s);
        let decoded = decode(&encoded).expect("decode");
        prop_assert_eq!(s, decoded);
    }

    #[test]
    fn add_commutative(a in 0i32..1000, b in 0i32..1000) {
        prop_assert_eq!(add(a, b), add(b, a));
    }
}
```

## Mocking with mockall

```rust
// Cargo.toml: mockall = { version = "0.12", default-features = false }
use mockall::{automock, predicate::*};

#[automock]
pub trait Database {
    fn get_user(&self, id: u64) -> Option<User>;
    fn save_user(&mut self, user: User) -> anyhow::Result<()>;
}

#[test]
fn test_service_with_mock_db() {
    let mut mock = MockDatabase::new();
    mock.expect_get_user()
        .with(eq(42u64))
        .times(1)
        .returning(|_| Some(User { id: 42, name: "Alice".into() }));

    let service = UserService::new(mock);
    let user = service.find(42).unwrap();
    assert_eq!(user.name, "Alice");
}
```

## Snapshot Testing (insta)

```rust
// Cargo.toml (dev): insta = "1"
// Run `cargo insta review` to accept new snapshots
use insta::assert_snapshot;

#[test]
fn test_error_message_format() {
    let err = AppError::MissingKey { key: "database_url".into() };
    assert_snapshot!(err.to_string());
    // Creates/compares: snapshots/test_error_message_format.snap
}
```

## Coverage

```bash
# Install: cargo install cargo-tarpaulin (Linux) or cargo-llvm-cov
cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info
cargo llvm-cov report --html   # HTML report in target/llvm-cov/html/
```

## CI Test Matrix Example (GitHub Actions)

```yaml
test:
  strategy:
    matrix:
      os: [ubuntu-latest, windows-latest, macos-latest]
      rust: [stable, 1.75]  # MSRV
  runs-on: ${{ matrix.os }}
  steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@master
      with: { toolchain: "${{ matrix.rust }}", components: clippy, rustfmt }
    - run: cargo fmt --all --check
    - run: cargo clippy --all-targets --all-features -- -D warnings
    - run: cargo test --all-features
```
