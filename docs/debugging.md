# GPUIShowCase Debugging Guide

## VS Code Profiles

- Native desktop debug: `.vscode/launch.json`의 `Desktop: GPUIShowCase`
- Native desktop logs route debug: `.vscode/launch.json`의 `Desktop: Logs Route`
- Independent web mode: `.vscode/launch.json`의 `Web Mode: Browser`

## Required Extension

- Rust native debug: `vadimcn.vscode-lldb`
- Browser debug: VS Code 기본 JavaScript debugger (`pwa-chrome`)

## Platform Notes

### Windows

- 현재 workspace에서 native 빌드와 런타임 검증을 수행했다.
- PowerShell helper script를 제공한다.

### Linux

- Bash helper script와 cross-platform cargo task를 제공한다.
- 실제 native smoke는 이 workspace에서 검증하지 못했다.

### macOS

- Bash helper script와 cross-platform cargo task를 제공한다.
- GPUI 의존성으로 Xcode/Metal 계열 준비가 필요하다.
- 실제 native smoke는 이 workspace에서 검증하지 못했다.

## Helper Scripts

- `scripts/run-desktop.ps1`
- `scripts/run-desktop.sh`
- `scripts/run-web-mode.ps1`
- `scripts/run-web-mode.sh`

## Independent Web Mode

- server binary: `cargo run --bin web_mode --target-dir target-agent -- --host 127.0.0.1 --port 4173`
- browser URL: `http://127.0.0.1:4173`
- health check: `http://127.0.0.1:4173/health`

## Validation Gap

- launch configuration JSON은 workspace에 반영했지만, Linux/macOS 디버그 세션은 이 Windows 환경에서 실제 attach/run 검증을 할 수 없다.