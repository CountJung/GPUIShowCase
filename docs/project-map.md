# GPUIShowCase 프로젝트 맵

이 문서는 기능별 검증 경로를 추적한다.
현재는 Phase 0 기준의 초기 항목 등록 상태이며, 실제 구현이 시작되면 명령 결과와 증거 위치를 채운다.

## Coverage Rules

- 모든 주요 기능은 최소 1개의 프로젝트 맵 항목을 가져야 한다.
- Phase 완료 시 관련 항목 상태를 갱신한다.
- 실패한 검증은 삭제하지 않고 `BLOCKED` 또는 `FAIL`로 남긴다.
- 데이터 시나리오는 중립적 예제 데이터 기준으로 유지한다.

## Status Legend

- `TODO`: 아직 실행 전
- `READY`: 검증 경로와 기대 결과가 정의됨
- `PASS`: 최근 검증이 성공함
- `FAIL`: 최근 검증이 실패했으며 재작업 필요
- `BLOCKED`: 전제조건 부족으로 실행 불가

## Project Map Table

| 항목 ID | 대상 기능 | 계층 | 검증 방식 | 실행 트리거 또는 명령 | 기대 결과 | 증거 위치 | 상태 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| PM-001 | 앱 부트스트랩 | bootstrap | smoke | `cargo run` | 기본 창이 열리고 즉시 종료되지 않음 | `cargo run` 완료 후 `target-agent\debug\gpuishowcase.exe` 실행 확인. 화면 캡처 시각 `2026-04-17T06:03:37Z` | PASS |
| PM-002 | 루트 레이아웃 | layout | manual | 앱 실행 후 셸 확인 | 왼쪽 레일, examples 사이드바, Header, Content, Inspector 구조가 유지됨 | 2026-05-21 전면 재작성. `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test` 통과 | PASS |
| PM-003 | 라우터 전환 | state | manual | 왼쪽 레일 및 examples 선택 | Showcase/Settings/Logs 전환과 선택 컴포넌트 상태가 일치함 | 2026-05-21 `AppRoute::Components`를 `Showcase` 기본 화면으로 변경. 상태 테스트 갱신 및 `cargo test` 통과 | PASS |
| PM-004 | 테마 전환 | theme | manual | Settings → Theme Presets에서 테마 카드 선택 | GitHub Dark가 기본값이고, 선택한 JSON 테마가 앱 전체에 즉시 적용됨 | 2026-05-21 Settings 재설계. GitHub Light/Dark 및 10+ JSON 프리셋 선택 가능 | PASS |
| PM-005 | Dashboard 렌더링 | feature | smoke | Dashboard 진입 | 요약 카드와 보조 시각화가 안전하게 렌더링됨 | `cargo run` 후 Dashboard 화면 캡처 시각 `2026-04-17T06:39:33Z`. System, Component, Render Performance, Theme, Chart Placeholder 카드 확인 | PASS |
| PM-006 | Showcase Workbench | feature | manual | examples 사이드바에서 컴포넌트 선택 및 전환 | 중앙 Live Preview, 하단 Props Table, 우측 Inspector가 동기화됨 | 2026-05-21 `src/features/components/mod.rs`, `src/app/layout/nav_panel.rs`, `src/app/layout/sidebar.rs` 재작성. `cargo test` 통과 | PASS |
| PM-007 | Playground 이벤트 로그 | feature | integration | Playground 상호작용 | 이벤트 로그와 상태 전이가 누락 없이 기록됨 | `target-agent\debug\gpuishowcase.exe --route playground --playground-script preview-badge,toggle-selected,increase-intensity,toggle-loading` 실행 후 화면 캡처 시각 `2026-04-17T07:35:44Z`. Real-time Composition, User Event Log, State Change Trace 동시 노출 확인 | PASS |
| PM-008 | Data Showcase | data | integration | 공통 샘플 데이터 로딩 | 표, 차트, 리스트가 동일 데이터 기준으로 갱신됨 | `target-agent\debug\gpuishowcase.exe --route data-showcase` 실행 후 화면 캡처 시각 `2026-04-17T07:50:35Z`. Shared Dataset Overview, DataTable Sample, Trend Chart, File Explorer List 동시 노출 확인 | PASS |
| PM-009 | 파일 로깅 및 UI 로그 뷰 | logging | integration | `target-agent\debug\gpuishowcase.exe --route logs` 실행 후 Logs 화면과 `logs/` 디렉터리 확인 | `logs/` 아래에 파일 로그가 생성되고, UI 로그 페이지 또는 패널에서 동일 계열 로그를 확인할 수 있음 | `target-agent\debug\gpuishowcase.exe --route logs` 실행 후 화면 캡처 시각 `2026-04-17T08:10:57Z`. `logs\app.log.2026-04-17` 생성 및 초기화 로그 기록 확인 | PASS |
| PM-010 | 성능 기본선 | performance | smoke | 대용량 리스트 또는 테이블 렌더링 | UI 프리즈 없이 상호작용 가능함 | `target-agent\debug\gpuishowcase.exe --route data-showcase` 실행 후 화면 캡처 시각 `2026-04-17T07:50:35Z`. 2048 row capacity hint와 동시 렌더링 화면이 즉시 표시됨 | PASS |
| PM-011 | Independent Web Mode | feature | integration | `cargo run --bin web_mode --target-dir target-agent -- --host 127.0.0.1 --port 4173` 실행 후 `/health`와 브라우저 렌더 확인 | 독립 웹 모드가 브라우저에서 로드되고 health endpoint가 응답함 | `Listening on http://127.0.0.1:4173` 출력 확인, `/health` 응답 `ok`, 브라우저 snapshot에서 Hero, Routes, Project Map, Platforms, Launch Profiles 섹션 동시 확인 | PASS |
| PM-012 | JSON 프리셋 테마 선택 | theme | manual | Settings → 테마 카드 클릭 | 10+ 테마 카드 표시, 클릭 시 앱 전체 테마 즉시 변경 | 2026-05-21 구현 완료. Catppuccin, Tokyo Night, Solarized, Nord, Dracula, Gruvbox, One Dark, GitHub, Material, Everforest, Monokai 11종 프리셋 | PASS |

## Notes

- PM-001과 PM-002는 2026-04-17 수동 스모크와 스크린 캡처 기준으로 갱신했다.
- PM-003은 2026-04-17 런타임 클릭 검증과 스크린 캡처 기준으로 갱신했다.
- PM-004는 2026-05-21 JSON 프리셋 테마 선택 기능으로 대체 구현. Settings 페이지에서 11종 테마를 카드 UI로 선택하면 즉시 적용된다.
- PM-005는 2026-04-17 Dashboard 런타임 화면 캡처 기준으로 갱신했다.
- PM-006은 2026-04-17 Explorer 런타임 화면 캡처 기준으로 갱신했다.
- PM-007은 2026-04-17 Playground scripted runtime 화면 캡처 기준으로 갱신했다.
- PM-008과 PM-010은 2026-04-17 Data Showcase runtime 화면 캡처 기준으로 갱신했다.
- PM-009는 2026-04-17 Logs runtime 화면 캡처와 `logs\app.log.2026-04-17` 파일 생성 확인 기준으로 갱신했다.
- PM-011은 2026-04-17 independent web mode 서버 기동, `/health` 응답, 브라우저 snapshot 기준으로 갱신했다.
- PM-012는 2026-05-21 신규 추가. JSON 프리셋 테마 기능 구현 완료.
- 2026-05-21: upstream `longbridge/gpui-component` 문서 확인 후 Showcase 첫 화면과 examples 사이드바 구조로 전면 재작성했다. 참고한 upstream 기준: README의 `story` gallery, `examples` standalone folder, `crates/story/src/stories` 목록.
- 이후 레이아웃 구조나 상태 모델이 바뀌면 PM-002, PM-003, PM-004, PM-006을 다시 확인한다.
