# GPUIShowCase Quality Constitution

## Purpose

이 프로젝트의 품질 기준은 구현 속도보다 재현 가능성과 검증 가능성을 우선한다.
모든 단계는 계획, 구현, 검증, 기록이 함께 끝나야 완료다.

## Quality Principles

1. 오류와 경고를 무시하지 않는다.
2. 계획 문서와 실제 결과를 분리하지 않는다.
3. 기능 시연용 데이터는 범용적이어야 한다.
4. 상위 문서와 어긋나는 구현은 즉시 수정하거나 blocker로 기록한다.
5. 작은 기능이라도 검증 경로를 먼저 정의한다.
6. 모든 Markdown 문서, 스킬 파일, 명령 체크리스트는 살아있는 산출물로 취급한다.
7. 사용자가 알아야 하거나 입력해야 하는 정보는 반드시 UI에 드러나야 한다.

## Mandatory Validation Gates

- Format: `cargo fmt --check`
- Build: `cargo check`
- Lint: `cargo clippy --all-targets --all-features -- -D warnings`
- Test: `cargo test`
- Manual UI check: current phase project map entries

## Evidence Rules

- 실행한 명령과 결과는 문서 또는 작업 기록에 남긴다.
- 프로젝트 맵에는 최소한 상태와 증거 위치를 남긴다.
- 우회가 있으면 이유, 리스크, 제거 조건을 명시한다.
- 문서와 명령 예시가 바뀌면 같은 단계 안에서 원본 문서를 갱신한다.

## Scope Rules

- 현재 범위의 Data Showcase는 범용 운영 데이터 기반이다.
- 성급한 실서비스 도메인 구체화는 금지한다.

## User Visibility Rules

- 파일 로그, 터미널 출력, 내부 상태만으로 사용자 커뮤니케이션을 대신하지 않는다.
- 사용자에게 의미 있는 상태 변화, 오류, 진행 상황, 확인 요청은 UI에 표시한다.
- 로깅 기능은 롤링 파일과 함께 UI에서 확인 가능한 로그 페이지 또는 패널을 제공해야 한다.
