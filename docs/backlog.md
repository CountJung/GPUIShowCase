# GPUIShowCase Backlog

## Open Items

| ID | 상태 | 항목 | 재진입 조건 |
| --- | --- | --- | --- |
| BG-001 | OPEN | Linux native smoke와 harness evidence 추가 | Linux runner 또는 실기기 확보 |
| BG-002 | OPEN | macOS native smoke와 harness evidence 추가 | macOS runner 또는 실기기 확보 |
| BG-003 | OPEN | GPUI native web target 안정성 재검토 | upstream docs가 browser target을 공식 운영 경로로 명시 |
| BG-004 | OPEN | desktop + web mode CI 자동화 | GitHub Actions 또는 사내 runner 정책 확정 |
| BG-005 | OPEN | ThemeRegistry 기반 실제 테마 선택 구현 | `gpui-component` 테마 JSON 파일 번들링 경로 확정 및 `ThemeRegistry::watch_dir` 통합 |

## Deferred Notes

- 현재 web mode는 independent companion surface다. native GPUI browser target으로 가정하지 않는다.
- Windows는 현재 workspace에서 직접 검증했지만, cross-platform parity는 아직 문서/스크립트 수준 검토 단계다.
- BG-005: `gpui-component`의 `ThemeRegistry`는 `watch_dir` + 외부 JSON 파일 방식이다. crates.io 패키지에는 테마 파일이 포함되지 않으므로 별도 번들링이 필요하다. 재진입 조건: 테마 JSON 파일 소스 확정 (upstream themes/ 디렉터리 복사 또는 자체 JSON 정의).