# GPUIShowCase Harness Map

이 문서는 기능별 검증 경로를 추적한다.
현재는 Phase 0 기준의 초기 하네스 등록 상태이며, 실제 구현이 시작되면 명령 결과와 증거 위치를 채운다.

## Coverage Rules

- 모든 주요 기능은 최소 1개의 하네스를 가져야 한다.
- Phase 완료 시 관련 하네스 상태를 갱신한다.
- 실패한 검증은 삭제하지 않고 `BLOCKED` 또는 `FAIL`로 남긴다.
- 데이터 시나리오는 중립적 예제 데이터 기준으로 유지한다.

## Status Legend

- `TODO`: 아직 실행 전
- `READY`: 검증 경로와 기대 결과가 정의됨
- `PASS`: 최근 검증이 성공함
- `FAIL`: 최근 검증이 실패했으며 재작업 필요
- `BLOCKED`: 전제조건 부족으로 실행 불가

## Harness Table

| Harness ID | 대상 기능 | 계층 | 검증 방식 | 실행 트리거 또는 명령 | 기대 결과 | 증거 위치 | 상태 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| HM-001 | 앱 부트스트랩 | bootstrap | smoke | `cargo run` | 기본 창이 열리고 즉시 종료되지 않음 | `cargo run` 완료 후 `target-agent\debug\gpuishowcase.exe` 실행 확인. 화면 캡처 시각 `2026-04-17T06:03:37Z` | PASS |
| HM-002 | 루트 레이아웃 | layout | manual | 앱 실행 후 셸 확인 | Sidebar, Header, Content 구조가 유지됨 | 화면 캡처 시각 `2026-04-17T06:03:37Z`, 창 리사이즈 후 재확인 `2026-04-17T06:04:01Z` | PASS |
| HM-003 | 라우터 전환 | state | manual | 내비게이션 선택 | 선택 상태와 콘텐츠가 일치함 | `Components` 선택 후 화면 캡처 시각 `2026-04-17T06:19:44Z`. Header, 상태 문자열, 선택 컴포넌트 영역 동시 변경 확인 | PASS |
| HM-004 | 테마 전환 | theme | manual | (제거됨) | 커스텀 ThemeMode가 비기능적임을 확인하여 Phase 10에서 제거. 실제 테마 지원은 BG-005로 이관 | Phase 10 제거 결정 근거: gpui-component ThemeRegistry는 외부 JSON 필요, crates.io 패키지에 미포함 | SUPERSEDED |
| HM-005 | Dashboard 렌더링 | feature | smoke | Dashboard 진입 | 요약 카드와 보조 시각화가 안전하게 렌더링됨 | `cargo run` 후 Dashboard 화면 캡처 시각 `2026-04-17T06:39:33Z`. System, Component, Render Performance, Theme, Chart Placeholder 카드 확인 | PASS |
| HM-006 | Component Explorer | feature | manual | 컴포넌트 선택 및 전환 | Preview, Code Preview, Props Panel이 동기화됨 | `Components / Advanced / Editor` 화면 캡처 시각 `2026-04-17T06:55:47Z`. Header 상태 문자열과 Explorer 3개 패널 동기화 확인 | PASS |
| HM-007 | Playground 이벤트 로그 | feature | integration | Playground 상호작용 | 이벤트 로그와 상태 전이가 누락 없이 기록됨 | `target-agent\debug\gpuishowcase.exe --route playground --playground-script preview-badge,toggle-selected,increase-intensity,toggle-loading` 실행 후 화면 캡처 시각 `2026-04-17T07:35:44Z`. Real-time Composition, User Event Log, State Change Trace 동시 노출 확인 | PASS |
| HM-008 | Data Showcase | data | integration | 공통 샘플 데이터 로딩 | 표, 차트, 리스트가 동일 데이터 기준으로 갱신됨 | `target-agent\debug\gpuishowcase.exe --route data-showcase` 실행 후 화면 캡처 시각 `2026-04-17T07:50:35Z`. Shared Dataset Overview, DataTable Sample, Trend Chart, File Explorer List 동시 노출 확인 | PASS |
| HM-009 | 파일 로깅 및 UI 로그 뷰 | logging | integration | `target-agent\debug\gpuishowcase.exe --route logs` 실행 후 Logs 화면과 `logs/` 디렉터리 확인 | `logs/` 아래에 파일 로그가 생성되고, UI 로그 페이지 또는 패널에서 동일 계열 로그를 확인할 수 있음 | `target-agent\debug\gpuishowcase.exe --route logs` 실행 후 화면 캡처 시각 `2026-04-17T08:10:57Z`. `logs\app.log.2026-04-17` 생성 및 초기화 로그 기록 확인 | PASS |
| HM-010 | 성능 기본선 | performance | smoke | 대용량 리스트 또는 테이블 렌더링 | UI 프리즈 없이 상호작용 가능함 | `target-agent\debug\gpuishowcase.exe --route data-showcase` 실행 후 화면 캡처 시각 `2026-04-17T07:50:35Z`. 2048 row capacity hint와 동시 렌더링 화면이 즉시 표시됨 | PASS |
| HM-011 | Independent Web Mode | feature | integration | `cargo run --bin web_mode --target-dir target-agent -- --host 127.0.0.1 --port 4173` 실행 후 `/health`와 브라우저 렌더 확인 | 독립 웹 모드가 브라우저에서 로드되고 health endpoint가 응답함 | `Listening on http://127.0.0.1:4173` 출력 확인, `/health` 응답 `ok`, 브라우저 snapshot에서 Hero, Routes, Harness, Platforms, Launch Profiles 섹션 동시 확인 | PASS |

## Notes

- HM-001과 HM-002는 2026-04-17 수동 스모크와 스크린 캡처 기준으로 갱신했다.
- HM-003과 HM-004는 2026-04-17 런타임 클릭 검증과 스크린 캡처 기준으로 갱신했다.
- HM-004는 2026-04-20 Phase 10에서 SUPERSEDED로 변경했다. 커스텀 ThemeMode가 실제 렌더링에 적용되지 않음을 확인하여 제거. 재진입 조건: BG-005 완료.
- HM-005는 2026-04-17 Dashboard 런타임 화면 캡처 기준으로 갱신했다.
- HM-006은 2026-04-17 Explorer 런타임 화면 캡처 기준으로 갱신했다.
- HM-007은 2026-04-17 Playground scripted runtime 화면 캡처 기준으로 갱신했다.
- HM-008과 HM-010은 2026-04-17 Data Showcase runtime 화면 캡처 기준으로 갱신했다.
- HM-009는 2026-04-17 Logs runtime 화면 캡처와 `logs\app.log.2026-04-17` 파일 생성 확인 기준으로 갱신했다.
- HM-011은 2026-04-17 independent web mode 서버 기동, `/health` 응답, 브라우저 snapshot 기준으로 갱신했다.
- 이후 레이아웃 구조나 상태 모델이 바뀌면 HM-002, HM-003, HM-004, HM-006을 다시 확인한다.
