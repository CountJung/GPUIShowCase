# Async Patterns Reference (tokio)

## Runtime Setup

```rust
// Simple: macro on main
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run().await
}

// Explicit: more control (use for libraries, tests, or when you need handles)
fn main() -> anyhow::Result<()> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .enable_all()
        .thread_name("my-app-worker")
        .build()?;
    rt.block_on(run())
}
```

## Concurrency Patterns

```rust
// Run two futures concurrently, fail fast if either errors
let (users, orders) = tokio::try_join!(
    fetch_users(db),
    fetch_orders(db),
)?;

// Run N futures concurrently with FuturesUnordered
use futures::StreamExt;
let handles: FuturesUnordered<_> = ids
    .iter()
    .map(|id| fetch_item(*id))
    .collect();
let results: Vec<_> = handles.collect().await; // Vec<Result<Item>>

// Spawn a detached background task
let handle = tokio::spawn(async move {
    background_work().await
});
// Await and double-unwrap: outer = JoinError (panic), inner = task error
let result = handle.await??;

// Fire and forget (detach)
tokio::spawn(log_event(event)).await.ok();
```

## Shared State Between Tasks

```rust
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

// Prefer RwLock when reads >> writes
#[derive(Clone)]
struct AppState {
    cache: Arc<RwLock<HashMap<String, String>>>,
    counter: Arc<std::sync::atomic::AtomicU64>,
}

impl AppState {
    async fn get(&self, key: &str) -> Option<String> {
        self.cache.read().await.get(key).cloned()
    }

    async fn set(&self, key: String, value: String) {
        self.cache.write().await.insert(key, value);
    }
}

// Pattern: clone the Arc, move into task
let state = AppState { ... };
tokio::spawn({
    let state = state.clone();
    async move { state.set("k".into(), "v".into()).await }
});
```

## Channels

```rust
use tokio::sync::{mpsc, oneshot, broadcast};

// mpsc: task → task pipeline (most common)
let (tx, mut rx) = mpsc::channel::<Event>(32); // bounded: backpressure
let (tx, mut rx) = mpsc::unbounded_channel::<Event>(); // unbounded: careful

tokio::spawn(async move {
    while let Some(event) = rx.recv().await {
        process(event).await;
    }
});
tx.send(Event::Start).await?;

// oneshot: request/response pattern
let (resp_tx, resp_rx) = oneshot::channel::<String>();
tx.send(Command::Query { resp: resp_tx }).await?;
let response = resp_rx.await?;

// broadcast: fan-out (e.g., shutdown signals)
let (shutdown_tx, _) = broadcast::channel::<()>(1);
let mut shutdown_rx = shutdown_tx.subscribe();
tokio::select! {
    _ = do_work() => {},
    _ = shutdown_rx.recv() => { /* graceful shutdown */ },
}
```

## Blocking Code in Async Context

```rust
// NEVER call blocking code directly in an async fn:
// ❌ std::thread::sleep, std::fs::read, heavy CPU work

// ✅ Offload blocking I/O
let contents = tokio::task::spawn_blocking(move || {
    std::fs::read_to_string(&path) // blocking call — OK inside spawn_blocking
})
.await??;

// ✅ Use async versions when available
let contents = tokio::fs::read_to_string(&path).await?; // preferred

// ✅ CPU-bound: rayon inside spawn_blocking
let result = tokio::task::spawn_blocking(move || {
    data.par_iter().map(crunch).sum::<f64>() // rayon parallel iter
})
.await?;
```

## Timeouts and Cancellation

```rust
use tokio::time::{timeout, Duration};

// Timeout a future
match timeout(Duration::from_secs(5), fetch_data()).await {
    Ok(Ok(data))  => process(data),
    Ok(Err(e))    => eprintln!("fetch error: {e}"),
    Err(_elapsed) => eprintln!("timed out after 5s"),
}

// Select — race multiple futures
tokio::select! {
    result = primary_task() => handle(result),
    result = fallback_task() => handle(result),
    _ = tokio::time::sleep(Duration::from_secs(30)) => {
        eprintln!("deadline exceeded");
    }
}
```

## Graceful Shutdown Pattern

```rust
use tokio::signal;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (shutdown_tx, shutdown_rx) = tokio::sync::watch::channel(false);

    let server = tokio::spawn(run_server(shutdown_rx));

    // Wait for Ctrl+C
    signal::ctrl_c().await?;
    eprintln!("shutdown signal received");

    // Signal all tasks
    shutdown_tx.send(true)?;

    // Wait with timeout
    tokio::time::timeout(
        std::time::Duration::from_secs(30),
        server,
    ).await???;

    Ok(())
}

async fn run_server(mut shutdown: tokio::sync::watch::Receiver<bool>) {
    loop {
        tokio::select! {
            _ = shutdown.changed() => break,
            _ = handle_request() => {},
        }
    }
    // cleanup here
}
```
