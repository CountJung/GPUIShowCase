# GPUIShowCase Testing Strategy

## 목적

- 현재 구현 범위를 빠르게 회귀 확인할 수 있는 최소 검증 세트를 유지한다.
- 네이티브 앱과 독립 웹 모드를 분리해 검증한다.

## Baseline Commands

- `cargo fmt --check`
- `cargo check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test`

## Native Feature Checks

| 영역 | 자동 검증 | 수동 또는 스모크 검증 |
| --- | --- | --- |
| Bootstrap | `cargo check`, `cargo test` | PM-001 |
| Shell/Layout | 상태 모델 테스트 간접 확인 | PM-002 |
| Routing/Theme | `shared/state` 테스트 | PM-003, PM-004 |
| Dashboard | 모델 테스트 | PM-005 |
| Showcase Workbench | registry 테스트 + examples 선택 수동 확인 | PM-006 |
| Playground | 상태 전이 테스트 | PM-007 |
| Data Showcase | dataset 테스트 | PM-008, PM-010 |
| Settings/Logging | 상태/로그 테스트 | PM-009 |

## Independent Web Mode Checks

| 항목 | 검증 방식 | 기대 결과 |
| --- | --- | --- |
| Static server boot | `cargo run --bin web_mode --target-dir target-agent -- --host 127.0.0.1 --port 4173` | `Listening on http://127.0.0.1:4173` 출력 |
| Health endpoint | 브라우저 또는 HTTP 요청 | `/health`가 `ok` 반환 |
| Page render | 브라우저 로드 | Hero, route cards, harness summary가 렌더링됨 |

## Platform Review Notes

- Windows: 현재 저장소에서 실제 빌드/실행 검증 완료
- Linux: VS Code task와 script는 제공하지만, 이 Windows workspace에서는 런타임 검증을 수행하지 못했다
- macOS: VS Code task와 script는 제공하지만, 이 Windows workspace에서는 런타임 검증을 수행하지 못했다

## 2026-05-21 Rewrite Checks

- `cargo fmt --check`: PASS
- `cargo clippy --all-targets --all-features -- -D warnings`: PASS
- `cargo test`: PASS
- 수동 확인 대상: 앱 실행 후 기본 화면이 Showcase이고 GitHub Dark 테마로 시작하는지, Settings에서 GitHub Light/Dark 및 다른 프리셋 전환이 즉시 반영되는지 확인한다.

## Re-entry Conditions

- Linux/macOS 실기기 또는 runner가 생기면 native smoke를 다시 수행한다.
- upstream GPUI web target이 안정화되면 independent web mode를 native browser target으로 대체할지 재검토한다.
