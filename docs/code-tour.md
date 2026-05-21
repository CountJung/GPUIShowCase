# GPUIShowCase Code Tour

## 1. Entry Points

- `src/main.rs`: desktop app bootstrap, launch override parsing, logging init
- `src/bin/web_mode.rs`: independent web mode static server

## 2. Native App Flow

1. `main`이 logging runtime을 초기화한다.
2. launch override를 읽어 초기 `AppState`를 구성한다.
3. `app::root::AppRoot`가 왼쪽 레일, examples 사이드바, Header, Content를 조합한다.
4. route/theme/component/settings 액션이 `AppState`를 갱신한다.
5. UI 로그와 파일 로그가 동일 이벤트 계열을 기록한다.

## 3. Shared State

- `src/shared/state/mod.rs`
- 핵심 타입:
  - `AppRoute`
  - `AppState`
  - `PlaygroundState`
  - `SettingsState`
  - `UiLogEntry`

## 4. Logging

- `src/shared/logger/mod.rs`
- 역할:
  - rolling file initialization
  - runtime log level update
  - event emission helper

## 5. Feature Surfaces

- `src/app/layout/sidebar.rs`: Showcase/Settings/Logs 전환용 icon rail
- `src/app/layout/nav_panel.rs`: gpui-component examples 선택 사이드바
- `src/features/components/mod.rs`: registry-driven Showcase workbench
- `src/features/playground/mod.rs`: action log + state trace
- `src/features/data_showcase/mod.rs`: shared dataset 기반 시각화
- `src/features/settings/mod.rs`: GitHub 기본 테마 및 JSON 프리셋 선택 + logs page

## 6. Debug Paths

- VS Code native debug: `.vscode/launch.json`의 LLDB profile
- VS Code web debug: `.vscode/launch.json`의 browser profile + `.vscode/tasks.json` background task
- CLI helper scripts: `scripts/`

## 7. Documents To Read First

1. `docs/master-plan.md`
2. `todolist.md`
3. `docs/project-map.md`
4. `docs/testing-strategy.md`
5. `docs/backlog.md`
