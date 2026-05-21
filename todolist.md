# GPUIShowCase 운영 체크리스트

이 문서는 장기 phase 설명 대신 현재 운영 상태와 다음 재진입 지점만 남기는 간략 운영 문서다.
상세 이력은 `docs/implementation-history.md`에서 관리한다.

## 현재 상태

- [x] Phase 0 ~ Phase 9 산출물 정리 완료
- [x] canonical master plan을 `docs/master-plan.md`로 이관
- [x] phase별 상세 문서를 이력 문서로 통합
- [x] native desktop + independent web mode 디버그 경로 정리
- [x] 2026-05-21 Showcase-first shell 재작성 및 GitHub Dark 기본 테마 적용

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
| GitHub Dark default | DONE | `src/main.rs` — `Theme::change(Dark)` + `themes/github.json` |
| 53 ComponentId variants | DONE | `src/shared/state/mod.rs` |
| Examples sidebar nav | DONE | `src/app/layout/nav_panel.rs` — category/component example index |
| Showcase workbench layout | DONE | `src/app/root.rs`, `src/features/components/mod.rs` |
| Component showcase panel | DONE | `src/features/components/mod.rs` — live preview/table/inspector |

## Open Backlog

- [ ] BG-001 Linux native smoke verification
- [ ] BG-002 macOS native smoke verification
- [ ] BG-003 upstream GPUI web target 재검토
- [ ] BG-004 desktop + web CI 자동화

## Re-entry Rule

- 새 기능을 추가할 때는 이 문서에 phase를 다시 늘리지 않고, `docs/master-plan.md`와 `docs/backlog.md`, `docs/project-map.md`를 먼저 갱신한다.
