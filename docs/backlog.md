# GPUIShowCase Backlog

## Open Items

| ID | 상태 | 항목 | 재진입 조건 |
| --- | --- | --- | --- |
| BG-001 | OPEN | Linux native smoke와 harness evidence 추가 | Linux runner 또는 실기기 확보 |
| BG-002 | OPEN | macOS native smoke와 harness evidence 추가 | macOS runner 또는 실기기 확보 |
| BG-003 | OPEN | GPUI native web target 안정성 재검토 | upstream docs가 browser target을 공식 운영 경로로 명시 |
| BG-004 | OPEN | desktop + web mode CI 자동화 | GitHub Actions 또는 사내 runner 정책 확정 |
| BG-005 | CLOSED | ThemeRegistry 기반 실제 테마 선택 구현 | 2026-05-21 `themes/` JSON 프리셋과 Settings theme selector로 구현 완료 |

## Deferred Notes

- 현재 web mode는 independent companion surface다. native GPUI browser target으로 가정하지 않는다.
- Windows는 현재 workspace에서 직접 검증했지만, cross-platform parity는 아직 문서/스크립트 수준 검토 단계다.
- BG-005는 닫았다. 기본값은 GitHub Dark이고, Settings에서 `themes/`의 JSON 프리셋을 선택한다.
