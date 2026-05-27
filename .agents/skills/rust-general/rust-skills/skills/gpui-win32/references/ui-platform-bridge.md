# UI ↔ Platform Bridge Reference

## 핵심 원칙

- **플랫폼 스레드**는 Win32 API를 호출한다. GPUI를 건드리지 않는다.
- **UI 스레드**는 GPUI `cx`로만 렌더링한다. Win32를 직접 호출하지 않는다.
- 둘은 `tokio::sync::mpsc` 채널로만 통신한다.
- `AppContext::spawn` 안에서 채널 수신 → `Model::update` → `cx.notify()`.

## 전체 브릿지 구조

```
AppModel (GPUI Model)
    │
    ├── cmd_tx: mpsc::Sender<UiCommand>       ─────────────────► Platform Worker
    │                                                                │
    └── (event loop via cx.spawn)                                   │
            ▲                                                        │
            │ event_rx: mpsc::Receiver<PlatformEvent>  ◄────────────┘
```

## 이벤트 타입 정의

```rust
// crates/ui/src/event.rs
use serde::{Deserialize, Serialize};

/// 플랫폼 백엔드 → UI
#[derive(Debug, Clone)]
pub enum PlatformEvent {
    // 광고 윈도우 숨김 성공
    AdHidden { hwnd: usize, process: String },
    // 감시 중인 윈도우가 다시 표시됨 (재차단 필요)
    AdReappeared { hwnd: usize },
    // 타겟 프로세스 시작/종료 감지
    ProcessStarted { name: String },
    ProcessExited  { name: String },
    // 스캔 주기 완료
    ScanComplete { found: usize, hidden: usize },
    // 에러
    PlatformError { context: String, message: String },
}

/// UI → 플랫폼 백엔드
#[derive(Debug, Clone)]
pub enum UiCommand {
    /// 해당 프로세스의 광고 윈도우 탐색 및 차단 시작
    StartBlocking { process_name: String },
    /// 차단 중지 및 원래 크기 복원
    StopBlocking  { process_name: String },
    /// 즉시 스캔 실행
    ForceScan,
    /// 애플리케이션 종료
    Shutdown,
}
```

## AppModel 초기화 (채널 포함)

```rust
// crates/ui/src/state.rs
use tokio::sync::mpsc;
use crate::event::{PlatformEvent, UiCommand};

pub struct AppModel {
    pub state: AppState,
    pub cmd_tx: mpsc::Sender<UiCommand>,
    // event_rx는 cx.spawn에서 소비되므로 Option으로 한 번만 꺼냄
    event_rx: Option<mpsc::Receiver<PlatformEvent>>,
}

impl AppModel {
    pub fn new(_cx: &mut Context<Self>) -> (Self, mpsc::Receiver<PlatformEvent>) {
        let (cmd_tx, cmd_rx) = mpsc::channel::<UiCommand>(32);
        let (event_tx, event_rx) = mpsc::channel::<PlatformEvent>(64);

        // 플랫폼 워커 시작
        Self::spawn_platform_worker(cmd_rx, event_tx);

        let model = Self {
            state: AppState::load_or_default(),
            cmd_tx,
            event_rx: None,
        };
        (model, event_rx)
    }

    fn spawn_platform_worker(
        mut cmd_rx: mpsc::Receiver<UiCommand>,
        event_tx: mpsc::Sender<PlatformEvent>,
    ) {
        tokio::spawn(async move {
            use platform::WindowsPlatform;
            let platform = WindowsPlatform::new();

            loop {
                match cmd_rx.recv().await {
                    Some(UiCommand::StartBlocking { process_name }) => {
                        match platform.find_ad_window(&process_name) {
                            Ok(Some(hwnd)) => {
                                platform.hide_ad(hwnd).ok();
                                event_tx.send(PlatformEvent::AdHidden {
                                    hwnd,
                                    process: process_name,
                                }).await.ok();
                            }
                            Ok(None) => { /* 아직 없음 — 다음 스캔에서 재시도 */ }
                            Err(e) => {
                                event_tx.send(PlatformEvent::PlatformError {
                                    context: format!("find_ad_window({})", process_name),
                                    message: e.to_string(),
                                }).await.ok();
                            }
                        }
                    }
                    Some(UiCommand::ForceScan) => {
                        // 모든 활성 타겟에 대해 스캔
                        // (실제 구현에서는 상태를 공유하거나 cmd에 targets 포함)
                    }
                    Some(UiCommand::Shutdown) | None => break,
                    _ => {}
                }
            }
        });
    }
}
```

## GPUI 이벤트 루프 연결

```rust
// crates/ui/src/app.rs
use gpui::*;

pub fn setup_event_loop(
    model: Model<AppModel>,
    mut event_rx: mpsc::Receiver<PlatformEvent>,
    cx: &mut AppContext,
) {
    cx.spawn(async move |mut cx| {
        while let Some(event) = event_rx.recv().await {
            // Model::update는 GPUI UI 스레드에서만 호출 가능
            model.update(&mut cx, |m, cx| {
                match event {
                    PlatformEvent::AdHidden { process, .. } => {
                        m.state.blocked_count += 1;
                        log::info!("Ad hidden for {process}");
                        cx.notify();
                    }
                    PlatformEvent::AdReappeared { hwnd } => {
                        // 재차단 커맨드 발송
                        let tx = m.cmd_tx.clone();
                        cx.spawn(async move |_cx| {
                            tx.send(UiCommand::StartBlocking {
                                process_name: "KakaoTalk.exe".into()
                            }).await.ok();
                        }).detach();
                    }
                    PlatformEvent::PlatformError { context, message } => {
                        log::error!("[{context}] {message}");
                        // UI에 에러 토스트 표시
                        cx.push_notification(
                            gpui_component::notification::Notification::new(
                                format!("Error: {message}")
                            )
                        );
                        cx.notify();
                    }
                    PlatformEvent::ProcessStarted { name } => {
                        log::info!("Target started: {name}");
                        // 자동 차단 시작
                        let tx = m.cmd_tx.clone();
                        cx.spawn(async move |_cx| {
                            tx.send(UiCommand::StartBlocking { process_name: name })
                              .await.ok();
                        }).detach();
                    }
                    PlatformEvent::ProcessExited { name } => {
                        log::info!("Target exited: {name}");
                        cx.notify();
                    }
                    _ => {}
                }
            }).ok(); // model이 dropped되면 ok()로 무시
        }
    }).detach();
}
```

## 뷰에서 커맨드 발송

```rust
// View 내부에서 cmd_tx를 통해 플랫폼에 명령 전달
impl DashboardView {
    fn render_action_bar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let cmd_tx = self.model.read(cx).cmd_tx.clone();

        Button::new("activate")
            .label("Activate")
            .on_click(cx.listener(move |_this, _ev, cx| {
                let tx = cmd_tx.clone();
                cx.spawn(async move |_cx| {
                    tx.send(UiCommand::StartBlocking {
                        process_name: "KakaoTalk.exe".into(),
                    }).await.ok();
                }).detach();
            }))
    }
}
```

## 주기적 스캔 (Polling)

```rust
// 1초마다 광고 윈도우 스캔
fn spawn_periodic_scan(
    cmd_tx: mpsc::Sender<UiCommand>,
    interval: Duration,
) {
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(interval);
        loop {
            ticker.tick().await;
            if cmd_tx.send(UiCommand::ForceScan).await.is_err() {
                break; // receiver dropped — 앱 종료
            }
        }
    });
}
```

## 종료 시퀀스

```rust
// GPUI window close 이벤트 핸들링
cx.on_window_closed(|cx| {
    // 플랫폼 워커에 종료 신호
    let tx = model.read(cx).cmd_tx.clone();
    cx.spawn(async move |_| {
        tx.send(UiCommand::Shutdown).await.ok();
        // 워커가 정리 후 종료할 때까지 잠시 대기
        tokio::time::sleep(Duration::from_millis(500)).await;
    }).detach();
});
```
