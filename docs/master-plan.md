# GPUIShowCase Master Plan

## 목적

- `gpui`와 `gpui-component` 기반의 데스크톱 쇼케이스 앱을 유지 가능한 구조로 운영한다.
- 단순 갤러리가 아니라 상태, 로그, 데이터 시각화, 검증 경로를 포함한 실제 앱 수준 UX를 제공한다.
- 네이티브 데스크톱 앱을 주 축으로 두고, 독립 웹 모드는 문서·데모·디버깅 보조 채널로 분리해 운영한다.

## 런타임 전략

### Native Desktop

- 주 실행 대상: Windows, Linux, macOS
- 현재 이 저장소에서 직접 검증된 대상: Windows
- Linux/macOS는 upstream GPUI 의존성과 도구 체인을 충족하는 환경에서 동일 Cargo 명령으로 실행 가능하도록 유지한다.

### Independent Web Mode

- 웹 모드는 GPUI 렌더러를 그대로 브라우저에 이식하는 방식이 아니라, 별도 정적 자산과 독립 서버 바이너리로 운영한다.
- 이유: 현재 `gpui` 문서는 macOS/Linux 중심의 네이티브 런타임을 명확히 안내하고 있으며, 이 저장소에서는 네이티브 앱이 주 제품이다.
- 목적: 브라우저에서 구조, 검증 상태, 라우트 개요, 디버그 진입점을 독립적으로 확인할 수 있게 한다.

## 핵심 구조

### Native App

- `src/main.rs`: 앱 부트스트랩, 시작 인자 해석, 로그 런타임 초기화
- `src/app/`: 루트 뷰와 레이아웃 조합
- `src/features/`: Dashboard, Components, Playground, Data Showcase, Settings
- `src/shared/state/`: 공용 앱 상태, 라우트, 로그/설정 상태
- `src/shared/logger/`: rolling file logging, log-level 반영

### Web Mode

- `src/bin/web_mode.rs`: 독립 정적 서버
- `web/`: 브라우저용 정적 자산
- `.vscode/launch.json`: desktop/web 디버그 진입점
- `.vscode/tasks.json`: background server 및 보조 task

## 사용자 표면

### Native Routes

- Dashboard: 시스템/구성 요약
- Components: 카테고리별 컴포넌트 탐색
- Playground: 상태 변화와 이벤트 실험
- Data Showcase: 공통 운영 데이터셋 기반 시각화
- Settings: theme, log level, density, performance mode
- Logs: in-app UI 로그와 파일 로그 정보

### Web Surface

- 프로젝트 현재 상태 요약
- feature route 카드
- harness 상태 요약
- 디버그 실행 방법과 플랫폼 메모

## 품질 규칙

- 경고, 빌드 실패, 테스트 실패를 넘기지 않는다.
- 사용자에게 필요한 상태와 입력은 UI에 드러나야 한다.
- 로그는 rolling file과 UI surface를 동시에 제공한다.
- 데이터 예시는 운영형/중립적 데이터로 유지한다.
- 문서와 스크립트는 코드와 함께 같은 단위에서 갱신한다.

## 검증 기준

- `cargo fmt --check`
- `cargo check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test`
- `docs/harness-map.md`의 현재 PASS 항목 유지
- web mode 추가 이후에는 독립 서버와 브라우저 로드 확인 포함

## 문서 맵

- 실행 체크리스트: `docs/todolist.md`
- 현재 구조/목표: `docs/master-plan.md`
- 구현 이력: `docs/implementation-history.md`
- 검증 범위: `docs/harness-map.md`
- 테스트 전략: `docs/testing-strategy.md`
- 구조 투어: `docs/code-tour.md`
- 남은 리스크와 재진입 조건: `docs/backlog.md`
- 디버깅 가이드: `docs/debugging.md`