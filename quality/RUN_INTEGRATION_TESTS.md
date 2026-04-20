# GPUIShowCase Integration Test Protocol

## Objective

구현된 기능이 문서상 설계와 실제 동작 면에서 일치하는지 단계별로 확인한다.

## Execution Order

1. Rust crate 존재 여부 확인
2. 기본 검증 명령 실행
3. 현재 Phase 관련 하네스 실행
4. 실패 시 원인 분류
5. 결과를 하네스 맵에 반영

## Baseline Commands

- `cargo fmt --check`
- `cargo check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test`

## Manual Integration Scenarios

### Shell Baseline

- 앱 창이 열린다.
- Sidebar, Header, Content 영역이 동시에 보인다.

### Navigation Baseline

- 화면 전환 시 선택 상태가 유지되거나 의도대로 초기화된다.
- 존재하지 않는 데이터나 비어 있는 상태에서도 앱이 죽지 않는다.

### Theme and Logging Baseline

- Theme 설정이 화면에 반영된다.
- 사용자 액션 이후 로그가 남는다.

### Data Showcase Baseline

- 표, 차트, 리스트가 공통 샘플 데이터에 반응한다.
- 특정 산업 도메인에 묶이지 않은 일반 예제 데이터로도 화면 목적이 유지된다.

### Independent Web Mode Baseline

- `web_mode` 서버가 기동된다.
- `/health`가 응답한다.
- 브라우저에서 hero, route cards, harness summary, platform notes가 보인다.

## If Blocked

- 빠진 전제조건을 기록한다.
- 임시 우회 여부를 기록한다.
- 다음 재진입 조건을 기록한다.
