---
name: gpui-win32
description: >
  Specialized Rust coding skill for GPUI desktop applications with Win32/Windows
  API platform integration. Use this skill whenever the user is building a GPUI
  app, working with gpui-component, creating native Windows UI, calling Win32
  APIs from Rust, writing platform-specific window management code, or combining
  GPUI rendering with native OS hooks (SetWinEventHook, SetWindowPos,
  EnumChildWindows, etc.). Also triggers for cross-platform GPUI apps that need
  Windows-specific backend layers. Apply this skill for any .rs file that imports
  gpui, gpui_component, or windows_sys. Always read references/win32-ffi.md
  before writing any unsafe Win32 bindings.
---

# GPUI + Win32 — Agent Coding Skill

You are building production-quality desktop applications with GPUI (the GPU-
accelerated UI framework from Zed) and gpui-component (the shadcn-style
component library). This skill covers the full stack: GPUI rendering, the
gpui-component library, Win32 platform integration, and the async bridge between
them.

Before writing non-trivial code, load the relevant reference:
- GPUI core patterns → `references/gpui-core.md`
- gpui-component usage → `references/gpui-component.md`
- Win32 FFI & safety → `references/win32-ffi.md`
- UI ↔ Platform bridge → `references/ui-platform-bridge.md`

---

## Project Architecture

```
adblocker/                          # workspace root
├── Cargo.toml                      # workspace + shared deps
├── crates/
│   ├── app/                        # binary crate — thin entrypoint
│   │   ├── Cargo.toml
│   │   ├── build.rs                # Windows resource embedding
│   │   └── src/
│   │       └── main.rs
│   ├── ui/                         # GPUI views, models, AppState
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── app.rs              # Application root, cx.open_window()
│   │       ├── state.rs            # AppModel, AppState structs
│   │       ├── event.rs            # PlatformEvent, UiCommand enums
│   │       └── views/
│   │           ├── mod.rs
│   │           ├── dashboard.rs
│   │           ├── target_list.rs
│   │           └── settings.rs
│   └── platform/                   # OS-specific backend, no GPUI imports
│       └── src/
│           ├── lib.rs
│           ├── traits.rs           # Platform trait
│           ├── windows/
│           │   ├── mod.rs
│           │   ├── finder.rs       # EnumWindows / EnumChildWindows
│           │   ├── manipulator.rs  # SetWindowPos, ShowWindow
│           │   └── hook.rs         # SetWinEventHook
│           └── macos/
│               └── mod.rs          # stub / Accessibility API
└── assets/
    └── icons/                      # Lucide SVG files
```

**Rule**: `platform` crate must never import `gpui` or `gpui-component`.
`ui` crate must never call Win32 APIs directly.
Communication only through `crates/ui/src/event.rs` types.

---

## Workspace Cargo.toml

```toml
[workspace]
resolver = "2"
members  = ["crates/app", "crates/ui", "crates/platform"]

[workspace.package]
version      = "0.1.0"
edition      = "2021"
rust-version = "1.75"

[workspace.dependencies]
# UI
gpui           = "0.2.2"
gpui-component = "0.5.1"

# Async & concurrency
tokio = { version = "1", features = ["full"] }

# Error handling
anyhow    = "1"
thiserror = "1"

# Serialization
serde      = { version = "1", features = ["derive"] }
serde_json = "1"

# Utilities
log        = "0.4"
env_logger = "0.11"
dirs       = "5"

[workspace.dependencies.windows-sys]
version  = "0.52"
features = [
    "Win32_Foundation",
    "Win32_UI_WindowsAndMessaging",
    "Win32_UI_Accessibility",
    "Win32_System_Threading",
    "Win32_System_ProcessStatus",
    "Win32_Security",
]
```

---

## GPUI Application Skeleton

```rust
// crates/app/src/main.rs
use anyhow::Result;
use gpui::*;
use gpui_component::Root;
use ui::{AppModel, RootView};

fn main() -> Result<()> {
    env_logger::init();

    let app = Application::new();
    app.run(move |cx: &mut AppContext| {
        // 1. Register gpui-component
        gpui_component::init(cx);

        // 2. Build the application model (holds AppState)
        let model = cx.new(|cx| AppModel::new(cx));

        // 3. Open the main window
        cx.spawn({
            let model = model.clone();
            async move |mut cx| {
                cx.open_window(
                    WindowOptions {
                        window_bounds: Some(WindowBounds::Windowed(
                            Bounds::centered(None, size(px(960.0), px(640.0)), &cx),
                        )),
                        titlebar: Some(TitlebarOptions {
                            title: Some("AdBlocker".into()),
                            appears_transparent: true,
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    |window, cx| {
                        let view = cx.new(|cx| RootView::new(model, window, cx));
                        cx.new(|cx| Root::new(view.into(), window, cx))
                    },
                )?;
                Ok::<_, anyhow::Error>(())
            }
        })
        .detach();
    });

    Ok(())
}
```

---

## GPUI View Pattern

Every UI component follows this pattern — no exceptions:

```rust
use gpui::*;
use gpui_component::{button::Button, label::Label};

pub struct DashboardView {
    state: Model<AppState>,
}

impl DashboardView {
    pub fn new(state: Model<AppState>, cx: &mut Context<Self>) -> Self {
        // Subscribe to state changes → auto re-render
        cx.observe(&state, |_this, _model, cx| cx.notify()).detach();
        Self { state }
    }
}

impl Render for DashboardView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.state.read(cx);

        div()
            .flex()
            .flex_col()
            .gap_4()
            .p_6()
            .size_full()
            .child(self.render_status_card(state, cx))
            .child(self.render_action_bar(cx))
    }
}

impl DashboardView {
    fn render_status_card(&self, state: &AppState, cx: &Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap_3()
            .p_4()
            .rounded_lg()
            .bg(cx.theme().colors().surface_background)
            .child(Label::new(format!("Blocked: {}", state.blocked_count)))
    }

    fn render_action_bar(&self, cx: &Context<Self>) -> impl IntoElement {
        let is_active = self.state.read(cx).is_active;

        Button::new("toggle")
            .label(if is_active { "Deactivate" } else { "Activate" })
            .on_click(cx.listener(|this, _event, cx| {
                this.state.update(cx, |s, cx| {
                    s.is_active = !s.is_active;
                    cx.notify();
                });
            }))
    }
}
```

**Rules for views**:
- `Render::render` must be pure — no side effects, no blocking, no async.
- All state lives in a `Model<T>`, never in the view struct directly.
- Use `cx.listener()` for event handlers — it provides `&mut Self` safely.
- Call `cx.notify()` after any state mutation to trigger re-render.
- Use `cx.observe()` to reactively watch external models.

---

## State Management

```rust
// crates/ui/src/state.rs
use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub is_active: bool,
    pub blocked_count: u64,
    pub targets: Vec<TargetApp>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetApp {
    pub process_name: String,
    pub display_name: String,
    pub enabled: bool,
    pub ad_window_class: String,
}

pub struct AppModel {
    pub state: AppState,
}

impl AppModel {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        let state = Self::load_persisted().unwrap_or_default();
        Self { state }
    }

    fn load_persisted() -> Option<AppState> {
        let path = config_path();
        let text = std::fs::read_to_string(path).ok()?;
        serde_json::from_str(&text).ok()
    }

    pub fn persist(&self) {
        if let Ok(json) = serde_json::to_string_pretty(&self.state) {
            let path = config_path();
            std::fs::create_dir_all(path.parent().unwrap()).ok();
            std::fs::write(path, json).ok();
        }
    }
}

fn config_path() -> std::path::PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("adblocker")
        .join("config.json")
}
```

---

## Platform Trait

```rust
// crates/platform/src/traits.rs
use anyhow::Result;

/// Abstracts OS-level window manipulation.
/// Implementations: WindowsPlatform, MacosPlatform.
/// This trait is Send + Sync — safe to share across threads.
pub trait Platform: Send + Sync + 'static {
    /// Returns true if the named process is currently running.
    fn is_target_running(&self, process_name: &str) -> bool;

    /// Finds the advertisement WebView window handle for a target process.
    /// Returns None if the process isn't running or no ad window is found.
    fn find_ad_window(&self, process_name: &str) -> Result<Option<usize>>;

    /// Collapses the window to zero height (hides the ad).
    fn hide_ad(&self, hwnd: usize) -> Result<()>;

    /// Restores the window to its original size.
    fn restore_ad(&self, hwnd: usize) -> Result<()>;

    /// Registers a WinEvent hook that fires when a previously hidden
    /// window becomes visible again, so we can re-hide it.
    fn watch_ad_window(
        &self,
        hwnd: usize,
        on_shown: Box<dyn Fn(usize) + Send + 'static>,
    ) -> Result<WatchHandle>;
}

/// Dropping this handle unregisters the event hook.
pub struct WatchHandle(pub Box<dyn FnOnce() + Send>);

impl Drop for WatchHandle {
    fn drop(&mut self) {
        // Box<dyn FnOnce> needs a trick to call
        let f = std::mem::replace(&mut self.0, Box::new(|| {}));
        f();
    }
}
```

Full Win32 implementation → `references/win32-ffi.md`

---

## UI ↔ Platform Bridge

```rust
// crates/ui/src/event.rs

/// Events sent from the platform thread to the UI
#[derive(Debug, Clone)]
pub enum PlatformEvent {
    AdBlocked  { hwnd: usize, process: String },
    AdRestored { hwnd: usize },
    TargetStarted { process: String },
    TargetExited  { process: String },
    Error { message: String },
}

/// Commands sent from the UI to the platform thread
#[derive(Debug, Clone)]
pub enum UiCommand {
    StartBlocking { process_name: String },
    StopBlocking  { process_name: String },
    Shutdown,
}
```

```rust
// Bridge setup in app startup
use tokio::sync::mpsc;

let (cmd_tx,  mut cmd_rx)  = mpsc::channel::<UiCommand>(32);
let (event_tx, mut event_rx) = mpsc::channel::<PlatformEvent>(64);

// Platform thread — runs entirely outside GPUI
tokio::spawn({
    let platform = Arc::new(WindowsPlatform::new());
    let event_tx = event_tx.clone();
    async move {
        loop {
            match cmd_rx.recv().await {
                Some(UiCommand::StartBlocking { process_name }) => {
                    match platform.find_ad_window(&process_name) {
                        Ok(Some(hwnd)) => {
                            platform.hide_ad(hwnd).ok();
                            event_tx.send(PlatformEvent::AdBlocked {
                                hwnd,
                                process: process_name,
                            }).await.ok();
                        }
                        Ok(None) => {}
                        Err(e) => {
                            event_tx.send(PlatformEvent::Error {
                                message: e.to_string(),
                            }).await.ok();
                        }
                    }
                }
                Some(UiCommand::Shutdown) | None => break,
                _ => {}
            }
        }
    }
});

// GPUI event pump — runs on the UI thread via cx.spawn
cx.spawn(async move |cx| {
    while let Some(event) = event_rx.recv().await {
        model.update(&mut cx, |m, cx| {
            match event {
                PlatformEvent::AdBlocked { .. } => {
                    m.state.blocked_count += 1;
                    cx.notify();
                }
                PlatformEvent::Error { message } => {
                    log::error!("platform error: {message}");
                }
                _ => {}
            }
        }).ok();
    }
}).detach();
```

Full bridge patterns → `references/ui-platform-bridge.md`

---

## gpui-component Quick Reference

| Component | Import | Notes |
|-----------|--------|-------|
| `Button` | `gpui_component::button::Button` | `.label()`, `.on_click()`, `.variant()` |
| `Label` | `gpui_component::label::Label` | `.text_color()`, `.text_sm()` |
| `Switch` | `gpui_component::switch::Switch` | Controlled: `.checked(bool)` |
| `Input` | `gpui_component::input::TextInput` | Requires `InputState` model |
| `Dropdown` | `gpui_component::dropdown::Dropdown` | Requires `DropdownState` |
| `Table` | `gpui_component::table::Table` | Virtual rows, column sorting |
| `Toast` | `gpui_component::toast::*` | `cx.push_notification(...)` |
| `Dock` | `gpui_component::dock::*` | Sidebar + panel layout |
| `Icon` | `gpui_component::icon::Icon` | Needs SVG in assets/icons/ |
| `Badge` | `gpui_component::badge::Badge` | Status indicators |

Full component guide → `references/gpui-component.md`

---

## Asset Setup (Icons)

gpui-component does not bundle icons. You must:

1. Download Lucide SVG files to `assets/icons/`
2. The filename must match the `IconName` enum variant (snake_case).
3. Register in `build.rs`:

```rust
// build.rs
fn main() {
    println!("cargo:rerun-if-changed=assets/");
    // No extra steps needed — GPUI finds assets via CARGO_MANIFEST_DIR
}
```

```rust
// Access at runtime
use gpui_component::icon::{Icon, IconName};

Icon::new(IconName::Play)        // assets/icons/play.svg
Icon::new(IconName::ShieldCheck) // assets/icons/shield_check.svg
```

---

## Code Checklist (GPUI-specific)

Before presenting any GPUI code, verify:

- [ ] `Render::render` is pure — no blocking, no async, no side effects
- [ ] State lives in `Model<T>`, not in view struct fields
- [ ] All event handlers use `cx.listener()` — never capture `&mut self`
- [ ] `cx.notify()` called after every state mutation
- [ ] `cx.observe()` used for cross-model reactivity, `.detach()` called
- [ ] Win32 calls are in the `platform` crate behind the `Platform` trait
- [ ] All `unsafe` blocks have a `// SAFETY:` comment explaining invariants
- [ ] `HWND` values passed as `usize` across thread boundaries (not raw pointer)
- [ ] Channel sends use `.await.ok()` — don't panic if receiver dropped
- [ ] Persisted config uses `serde_json`, saved on state changes
