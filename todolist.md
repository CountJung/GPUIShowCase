# GPUIShowCase 운영 체크리스트

이 문서는 장기 phase 설명 대신 현재 운영 상태와 다음 재진입 지점만 남기는 간략 운영 문서다.
상세 이력은 `docs/implementation-history.md`에서 관리한다.

## 현재 상태

- [x] Phase 0 ~ Phase 9 산출물 정리 완료
- [x] canonical master plan을 `docs/master-plan.md`로 이관
- [x] phase별 상세 문서를 이력 문서로 통합
- [x] native desktop + independent web mode 디버그 경로 정리

## Canonical Documents

- `docs/master-plan.md`
- `docs/implementation-history.md`
- `docs/project-map.md`
- `docs/testing-strategy.md`
- `docs/code-tour.md`
- `docs/backlog.md`
- `docs/debugging.md`

## Validation Baseline

- [x] `cargo fmt --check`
- [x] `cargo check`
- [x] `cargo clippy --all-targets --all-features -- -D warnings`
- [x] `cargo test`
- [x] web mode smoke and current project map review

## Current Completion Matrix

| 영역 | 상태 | 기준 문서 |
| --- | --- | --- |
| Native app shell and routes | DONE | `docs/master-plan.md` |
| Logging and settings | DONE | `docs/project-map.md` |
| Quality and testing docs | DONE | `docs/testing-strategy.md`, `quality/` |
| VS Code debugging setup | DONE | `docs/debugging.md`, `.vscode/` |
| Independent web mode | DONE | `docs/debugging.md`, `web/`, `src/bin/web_mode.rs` |
| Light theme default | DONE | `src/main.rs` — `Theme::change(Light)` |
| 53 ComponentId variants | DONE | `src/shared/state/mod.rs` |
| Hierarchical sidebar nav | DONE | `src/app/layout/sidebar.rs` — `Sidebar<SidebarMenu>` |
| Resizable layout | DONE | `src/app/root.rs` — `h_resizable` + `resizable_panel` |
| Component showcase panel | DONE | `src/features/components/mod.rs` — `render_component_showcase` |

## Open Backlog

- [ ] BG-001 Linux native smoke verification
- [ ] BG-002 macOS native smoke verification
- [ ] BG-003 upstream GPUI web target 재검토
- [ ] BG-004 desktop + web CI 자동화

## Re-entry Rule

- 새 기능을 추가할 때는 이 문서에 phase를 다시 늘리지 않고, `docs/master-plan.md`와 `docs/backlog.md`, `docs/project-map.md`를 먼저 갱신한다.
