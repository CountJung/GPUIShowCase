# GPUIShowCase Implementation History

이 문서는 기존 phase별 상세 문서를 요약한 이력 문서다.
Phase 0부터 Phase 9까지의 결과와 검증 흔적만 남기고, 장황한 단계별 설명은 정리했다.

## Phase Summary

| Phase | 결과 | 핵심 산출물 | 검증 요약 |
| --- | --- | --- | --- |
| 0 | DONE | `.github/`, `quality/`, `docs/project-map.md`, `.gitignore` | 작업 기준 문서와 품질 규칙 생성 |
| 1 | DONE | Cargo crate, `src/main.rs`, app/features/shared 구조 | PM-001 PASS |
| 2 | DONE | Sidebar, Header, Content shell | PM-002 PASS |
| 3 | DONE | `AppState`, routing, theme flow | PM-003, PM-004 PASS |
| 4 | DONE | Dashboard MVP | PM-005 PASS |
| 5 | DONE | Component Explorer | PM-006 PASS |
| 6 | DONE | Playground state/event surface | PM-007 PASS |
| 7 | DONE | Data Showcase shared dataset flow | PM-008, PM-010 PASS |
| 8 | DONE | Settings, rolling file logs, UI logs | PM-009 PASS |
| 9 | DONE with noted gaps | 테스트 전략, 코드 투어, backlog, VS Code debug, independent web mode | PM-011 PASS, Linux/macOS smoke는 backlog 유지 |

## Consolidation Note

- 기존 `docs/phase-0-foundation.md`부터 `docs/phase-8-settings-logging.md`까지의 상세 phase 문서는 이 문서로 통합했다.
- phase별 상세 산출물 추적은 더 이상 별도 문서로 유지하지 않고, 현재 상태는 `docs/master-plan.md`, `todolist.md`, `docs/project-map.md`에서 관리한다.

## Key Milestones

- UI 가시성 규칙을 앱 설계 전반에 반영했다.
- 로그 시스템을 file + UI dual-surface로 정리했다.
- Data Showcase 범위를 stock/trading이 아닌 generic operational data로 고정했다.
- VS Code 디버그 경로를 desktop native와 independent web mode로 분리했다.

## Verification Record

- PM-001 ~ PM-012는 `docs/project-map.md`에 PASS 증거를 유지한다.
- Linux/macOS native smoke gap은 `docs/backlog.md`에서 계속 추적한다.