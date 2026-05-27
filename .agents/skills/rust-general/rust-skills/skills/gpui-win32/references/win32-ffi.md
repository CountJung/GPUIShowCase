# Win32 FFI Safety Reference

## 기본 원칙

Win32 API 호출은 모두 `unsafe`다. 모든 `unsafe` 블록에는 반드시
`// SAFETY:` 주석으로 안전성 보장 근거를 명시한다.

```rust
// SAFETY: hwnd is a valid window handle obtained from EnumWindows callback,
//         which only fires for existing top-level windows.
unsafe { SetWindowPos(hwnd, HWND_TOP, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOZORDER) };
```

## HWND 스레드 안전성

`HWND`는 raw pointer이므로 Rust 스레드 경계를 넘길 수 없다.
대신 `usize`로 변환해서 전달한다.

```rust
// 플랫폼 스레드에서: HWND → usize
let hwnd_raw: usize = hwnd.0 as usize;
event_tx.send(PlatformEvent::AdBlocked { hwnd: hwnd_raw }).await.ok();

// 다시 사용할 때: usize → HWND
use windows_sys::Win32::Foundation::HWND;
let hwnd = HWND(hwnd_raw as isize);
```

## 프로세스 탐색

```rust
use windows_sys::Win32::{
    Foundation::{BOOL, HWND, LPARAM, TRUE},
    UI::WindowsAndMessaging::{EnumWindows, GetWindowThreadProcessId},
    System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ},
    System::ProcessStatus::GetProcessImageFileNameW,
};

/// 특정 프로세스명의 최상위 윈도우 핸들을 반환한다.
pub fn find_window_by_process(target_name: &str) -> Option<HWND> {
    struct SearchState {
        target: String,
        result: Option<HWND>,
    }

    unsafe extern "system" fn enum_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let state = &mut *(lparam as *mut SearchState);

        let mut pid: u32 = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));
        if pid == 0 { return TRUE; } // continue

        let process = OpenProcess(
            PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
            FALSE,
            pid,
        );
        if process.is_null() { return TRUE; }

        let mut name_buf = [0u16; 260];
        let len = GetProcessImageFileNameW(process, &mut name_buf);
        CloseHandle(process);

        if len == 0 { return TRUE; }

        let name = String::from_utf16_lossy(&name_buf[..len as usize]);
        // GetProcessImageFileNameW returns full path; extract filename
        let exe = name.rsplit(['\\', '/']).next().unwrap_or("");

        if exe.eq_ignore_ascii_case(&state.target) {
            state.result = Some(hwnd);
            return FALSE; // stop enumeration
        }
        TRUE
    }

    let mut state = SearchState {
        target: target_name.to_string(),
        result: None,
    };

    // SAFETY: callback is valid for the duration of EnumWindows call.
    //         lparam points to stack-allocated SearchState — safe because
    //         EnumWindows is synchronous and returns before this function exits.
    unsafe {
        EnumWindows(Some(enum_callback), &mut state as *mut _ as LPARAM);
    }

    state.result
}
```

## 자식 윈도우 탐색 (WebView2 식별)

```rust
use windows_sys::Win32::UI::WindowsAndMessaging::{
    EnumChildWindows, GetClassNameW, GetWindowTextW,
};

/// WebView2 광고 컨트롤을 찾는다.
/// WebView2는 "Chrome_WidgetWin_1" 또는 "Chrome_RenderWidgetHostHWND" 클래스를 사용한다.
pub fn find_webview2_child(parent: HWND) -> Option<HWND> {
    const WEBVIEW2_CLASSES: &[&str] = &[
        "Chrome_WidgetWin_1",
        "Chrome_RenderWidgetHostHWND",
    ];

    struct ChildSearch {
        targets: &'static [&'static str],
        result: Option<HWND>,
    }

    unsafe extern "system" fn child_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let search = &mut *(lparam as *mut ChildSearch);

        let mut class_buf = [0u16; 256];
        let len = GetClassNameW(hwnd, &mut class_buf);
        if len == 0 { return TRUE; }

        let class_name = String::from_utf16_lossy(&class_buf[..len as usize]);

        for target in search.targets {
            if class_name.contains(target) {
                search.result = Some(hwnd);
                return FALSE; // found — stop
            }
        }
        TRUE
    }

    let mut search = ChildSearch {
        targets: WEBVIEW2_CLASSES,
        result: None,
    };

    // SAFETY: parent is a valid HWND; callback lifetime is bounded by this call.
    unsafe {
        EnumChildWindows(
            parent,
            Some(child_callback),
            &mut search as *mut _ as LPARAM,
        );
    }

    search.result
}
```

## 윈도우 크기 조절 (광고 숨김)

```rust
use windows_sys::Win32::UI::WindowsAndMessaging::{
    GetWindowRect, SetWindowPos,
    SWP_NOMOVE, SWP_NOZORDER, SWP_NOACTIVATE,
    HWND_TOP,
};
use windows_sys::Win32::Foundation::RECT;

/// 이전 크기를 저장하고 윈도우를 height=0으로 축소한다.
pub fn hide_window(hwnd: HWND) -> anyhow::Result<RECT> {
    let mut rect = RECT { left: 0, top: 0, right: 0, bottom: 0 };

    // SAFETY: hwnd is a valid window handle. RECT is initialized above.
    let ok = unsafe { GetWindowRect(hwnd, &mut rect) };
    anyhow::ensure!(ok != 0, "GetWindowRect failed: hwnd={:?}", hwnd);

    let width = rect.right - rect.left;

    // SAFETY: hwnd is valid; flags are well-defined Win32 constants.
    let ok = unsafe {
        SetWindowPos(
            hwnd,
            HWND_TOP,
            0, 0,
            width, 0,           // height = 0
            SWP_NOMOVE | SWP_NOZORDER | SWP_NOACTIVATE,
        )
    };
    anyhow::ensure!(ok != 0, "SetWindowPos failed");

    Ok(rect) // caller stores this for restore
}

/// 숨겨진 윈도우를 원래 크기로 복원한다.
pub fn restore_window(hwnd: HWND, original: RECT) -> anyhow::Result<()> {
    let width  = original.right  - original.left;
    let height = original.bottom - original.top;

    // SAFETY: hwnd is valid; original rect was obtained from GetWindowRect.
    let ok = unsafe {
        SetWindowPos(
            hwnd,
            HWND_TOP,
            0, 0,
            width, height,
            SWP_NOMOVE | SWP_NOZORDER | SWP_NOACTIVATE,
        )
    };
    anyhow::ensure!(ok != 0, "SetWindowPos (restore) failed");
    Ok(())
}
```

## WinEventHook — 광고 재표시 감지

```rust
use windows_sys::Win32::UI::Accessibility::{
    SetWinEventHook, UnhookWinEvent, HWINEVENTHOOK,
    WINEVENT_OUTOFCONTEXT, EVENT_OBJECT_SHOW,
};

static HOOK_CALLBACK: std::sync::atomic::AtomicUsize
    = std::sync::atomic::AtomicUsize::new(0);

pub struct EventHook(HWINEVENTHOOK);

impl Drop for EventHook {
    fn drop(&mut self) {
        if !self.0.is_null() {
            // SAFETY: self.0 is a valid hook handle from SetWinEventHook.
            unsafe { UnhookWinEvent(self.0); }
        }
    }
}

unsafe impl Send for EventHook {}

pub fn watch_window_show(
    hwnd: HWND,
    on_shown: Box<dyn Fn(HWND) + Send>,
) -> anyhow::Result<EventHook> {
    // Store callback in a thread-local or static; WinEventHook requires
    // a bare fn pointer — use a global slot for simplicity.
    // For production: use a registry map keyed by HWND.
    let cb = Box::new(on_shown);
    let cb_ptr = Box::into_raw(Box::new(cb));
    HOOK_CALLBACK.store(cb_ptr as usize, std::sync::atomic::Ordering::SeqCst);

    unsafe extern "system" fn hook_proc(
        _hook: HWINEVENTHOOK,
        event: u32,
        hwnd: HWND,
        _id_object: i32,
        _id_child: i32,
        _thread: u32,
        _time: u32,
    ) {
        if event == EVENT_OBJECT_SHOW {
            let ptr = HOOK_CALLBACK.load(std::sync::atomic::Ordering::SeqCst);
            if ptr != 0 {
                // SAFETY: ptr is valid as long as the hook is registered.
                let cb = &*(ptr as *const Box<dyn Fn(HWND) + Send>);
                cb(hwnd);
            }
        }
    }

    // SAFETY: hook_proc has the correct signature; process is long-lived.
    let hook = unsafe {
        SetWinEventHook(
            EVENT_OBJECT_SHOW,
            EVENT_OBJECT_SHOW,
            std::ptr::null_mut(),
            Some(hook_proc),
            0,     // all processes
            0,     // all threads
            WINEVENT_OUTOFCONTEXT,
        )
    };

    anyhow::ensure!(!hook.is_null(), "SetWinEventHook failed");
    Ok(EventHook(hook))
}
```

## 자주 발생하는 실수

```rust
// ❌ HWND를 직접 스레드 경계로 넘기기 → 컴파일 에러 또는 UB
std::thread::spawn(move || use_hwnd(hwnd)); // HWND는 !Send

// ✅ usize로 변환해서 전달
let raw = hwnd.0 as usize;
std::thread::spawn(move || {
    let hwnd = HWND(raw as isize);
    use_hwnd(hwnd);
});

// ❌ WinAPI 반환값 확인 안 하기
SetWindowPos(hwnd, HWND_TOP, 0, 0, 0, 0, flags); // silent failure

// ✅ 반환값으로 에러 처리
let ok = unsafe { SetWindowPos(hwnd, HWND_TOP, 0, 0, 0, 0, flags) };
anyhow::ensure!(ok != 0, "SetWindowPos failed: {}", GetLastError());

// ❌ SAFETY 주석 없는 unsafe 블록
unsafe { CloseHandle(handle); }

// ✅ 항상 SAFETY 주석
// SAFETY: handle was obtained from OpenProcess and not yet closed.
unsafe { CloseHandle(handle); }
```
