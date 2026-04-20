# GPUIShowCase 운영 체크리스트

이 파일이 유일한 실행 기준 체크리스트다. 상세 이력은 `docs/implementation-history.md`에서 관리한다.

## Canonical Documents

| 문서 | 역할 |
| --- | --- |
| `docs/todolist.md` | 실행 체크리스트 (이 파일) |
| `docs/master-plan.md` | 아키텍처 및 전략 계획 |
| `docs/implementation-history.md` | 구현 이력 |
| `docs/harness-map.md` | 하네스 검증 맵 |
| `docs/testing-strategy.md` | 테스트 전략 |
| `docs/code-tour.md` | 코드 구조 투어 |
| `docs/backlog.md` | 미결 항목 |
| `docs/debugging.md` | 디버깅 가이드 |

## Validation Baseline

- [x] `cargo fmt --check`
- [x] `cargo check`
- [x] `cargo clippy --all-targets --all-features -- -D warnings`
- [x] `cargo test`
- [x] web mode smoke and current harness review

## Phase 0~9 완료 현황

| 영역 | 상태 | 기준 문서 |
| --- | --- | --- |
| Native app shell and routes | DONE | `docs/master-plan.md` |
| Logging and settings | DONE | `docs/harness-map.md` |
| Quality and testing docs | DONE | `docs/testing-strategy.md`, `quality/` |
| VS Code debugging setup | DONE | `docs/debugging.md`, `.vscode/` |
| Independent web mode | DONE | `docs/debugging.md`, `web/`, `src/bin/web_mode.rs` |

---

## Phase 10: ThemeMode 정리

**배경**: 현재 `ThemeMode(Light/Dark/System)` 열거형은 `AppState`에 저장되지만
실제 렌더링에 어떤 시각적 변화도 적용하지 않는다.

**검토 결과**:
- `gpui-component = "0.5.1"`은 `ThemeRegistry` + JSON 파일 기반의 명명된 테마 시스템을 지원한다.
- 해당 테마 파일은 crates.io 패키지에 포함되지 않으며 별도 번들링이 필요하다.
- MUI 스타일의 색상 토큰 방식은 `cx.theme().primary` 등의 `ActiveTheme` trait으로
  이미 제공되지만, 런타임 테마 전환을 위해서는 `ThemeRegistry::watch_dir` + 외부 JSON이 필요하다.
- 현재 커스텀 `ThemeMode`는 gpui-component 테마 시스템과 연결되지 않으므로 **제거**한다.
- 향후 테마 선택 기능은 `docs/backlog.md` BG-005에 기록하고 재진입 조건 충족 시 구현한다.

- [x] PH10-001: `shared/theme/mod.rs` 파일 및 모듈 제거
- [x] PH10-002: `AppState`에서 `theme_mode` 필드 및 `set_theme_mode` 제거
- [x] PH10-003: `header.rs`에서 ThemeMode UI 및 파라미터 제거
- [x] PH10-004: `settings/mod.rs`에서 Theme 패널 제거
- [x] PH10-005: `dashboard/mod.rs`에서 ThemeOverview 제거
- [x] PH10-006: `root.rs` 및 `content.rs`에서 theme 콜백 제거
- [x] PH10-007: `docs/harness-map.md` HM-004 상태 SUPERSEDED 업데이트
- [x] PH10-008: `cargo check` + `cargo clippy` 검증

---

## Phase 11: Component Explorer 전체 커버리지

**목표**: `gpui-component` 갤러리 60+ 컴포넌트를 `ComponentId` / `ComponentCategory`로 등록하고
각 컴포넌트에 대한 실제 preview 또는 stub preview를 구현한다.

### Basic Components (신규)

| ID | 컴포넌트 | 상태 |
| --- | --- | --- |
| PH11-001 | Accordion | TODO |
| PH11-002 | Alert | TODO |
| PH11-003 | AlertDialog | TODO |
| PH11-004 | Avatar | TODO |
| PH11-005 | Clipboard | TODO |
| PH11-006 | Collapsible | TODO |
| PH11-007 | DropdownButton | TODO |
| PH11-008 | HoverCard | TODO |
| PH11-009 | Icon (showcase) | TODO |
| PH11-010 | Image | TODO |
| PH11-011 | Kbd | TODO |
| PH11-012 | Label | TODO |
| PH11-013 | Pagination | TODO |
| PH11-014 | Progress | TODO |
| PH11-015 | Radio | TODO |
| PH11-016 | Rating | TODO |
| PH11-017 | Skeleton | TODO |
| PH11-018 | Slider | TODO |
| PH11-019 | Spinner | TODO |
| PH11-020 | Stepper | TODO |
| PH11-021 | Switch | TODO |
| PH11-022 | Tag | TODO |
| PH11-023 | Toggle | TODO |
| PH11-024 | Tooltip | TODO |

### Basic Components (기존 - 검토 필요)

| ID | 컴포넌트 | 상태 |
| --- | --- | --- |
| PH11-B01 | Button | DONE - 실제 Button::new() 렌더링 적용 |
| PH11-B02 | Badge | DONE - 실제 Badge::new().count() 렌더링 적용 |
| PH11-B03 | Checkbox | DONE - Checkbox::new().label() 실제 위젯 연동 완료 |

### Form Components (신규)

| ID | 컴포넌트 | 상태 |
| --- | --- | --- |
| PH11-025 | Select | TODO |
| PH11-026 | NumberInput | TODO |
| PH11-027 | DatePicker | TODO |
| PH11-028 | OtpInput | TODO |
| PH11-029 | ColorPicker | TODO |
| PH11-030 | Form (컨테이너) | TODO |

### Form Components (기존 - 검토 필요)

| ID | 컴포넌트 | 상태 |
| --- | --- | --- |
| PH11-F01 | Input | IMPROVED - Entity<InputState> 필요, 스타일 플레이스홀더로 개선 |
| PH11-F02 | Editor | IMPROVED - Entity<InputState> 필요, 코드 블록 표시로 개선 |

### Layout Components (신규)

| ID | 컴포넌트 | 상태 |
| --- | --- | --- |
| PH11-031 | DescriptionList | TODO |
| PH11-032 | GroupBox | TODO |
| PH11-033 | Notification (Toast) | TODO |
| PH11-034 | Popover | TODO |
| PH11-035 | Resizable panels | TODO |
| PH11-036 | Scrollable | TODO |
| PH11-037 | Sheet | TODO |

### Layout Components (기존 - 검토 필요)

| ID | 컴포넌트 | 상태 |
| --- | --- | --- |
| PH11-L01 | Dialog | IMPROVED - 트리거 버튼 + 모달 레이아웃 preview 구현 |
| PH11-L02 | Sidebar | IMPROVED - 내비게이션 항목 미니 패널 preview 구현 |

### Advanced Components (신규)

| ID | 컴포넌트 | 상태 |
| --- | --- | --- |
| PH11-038 | Calendar | TODO |
| PH11-039 | Menu (context/dropdown) | TODO |
| PH11-040 | DataTable (고성능 가상화 테이블) | TODO |
| PH11-041 | Tabs | TODO |
| PH11-042 | Tree | TODO |
| PH11-043 | VirtualList | TODO |
| PH11-044 | Plot (추가 차트 유형) | TODO |

### Advanced Components (기존 - 검토 필요)

| ID | 컴포넌트 | 상태 |
| --- | --- | --- |
| PH11-A01 | Chart | IMPROVED - Progress 바 기반 카테고리 분포 차트 preview |
| PH11-A02 | Table | IMPROVED - 컬럼 헤더 + 샘플 데이터 행 표 preview 구현 |
| PH11-A03 | List | IMPROVED - 선택 하이라이트 포함 리스트 항목 preview 구현 |

### Phase 11 구현 진행 이력

- 2026-04-20: render_component_column 동적화 (하드코딩 제거, ButtonGroup.children() 사용)
- 2026-04-20: apply_component_selection 동적화 (component_id_at 함수 도입, root.rs 하드코딩 제거)
- 2026-04-20: Checkbox 실제 위젯 연동, Input/Editor/Dialog/Sidebar/Table/List/Chart 개선 preview 구현
- cargo check/clippy/test 모두 통과 (19/19)

### Phase 11 구현 순서 제안 (원안)

1. 현재 STUB 컴포넌트들의 실제 gpui-component 연동부터 시작한다.
2. 신규 컴포넌트는 카테고리별로 묶어서 처리한다.
3. 각 컴포넌트는 `ComponentId` 열거형 추가 → 상태 등록 → Preview 구현 순으로 진행한다.
4. 각 배치 완료마다 `docs/harness-map.md`에 HM-012 이상 항목을 추가한다.

---

## Phase 12: 플랫폼 및 CI

| ID | 항목 | 상태 |
| --- | --- | --- |
| BG-001 | Linux native smoke 및 harness evidence | OPEN |
| BG-002 | macOS native smoke 및 harness evidence | OPEN |
| BG-003 | GPUI native web target 안정성 재검토 | OPEN |
| BG-004 | desktop + web mode CI 자동화 | OPEN |

---

## Re-entry Rule

새 기능을 추가할 때:
1. 이 파일에 Phase 항목을 추가한다.
2. `docs/master-plan.md`의 문서 맵을 확인한다.
3. `docs/harness-map.md`에 검증 항목을 추가한다.
4. 코드와 문서를 같은 단위에서 갱신한다.
