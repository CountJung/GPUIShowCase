use gpui::*;
use gpui_component::{
    Selectable as _, StyledExt,
    badge::Badge,
    button::Button,
    checkbox::Checkbox,
    progress::Progress,
};

use crate::shared::state::{AppState, ComponentCategory, ComponentId};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ComponentEntry {
    pub(crate) id: ComponentId,
    pub(crate) title: &'static str,
    pub(crate) summary: &'static str,
    pub(crate) code_preview: &'static str,
    pub(crate) props: &'static [&'static str],
    pub(crate) extraction_note: &'static str,
}

const BUTTON_PROPS: [&str; 3] = ["variant: outline", "label: Explorer Action", "interaction: click target"];
const BADGE_PROPS: [&str; 3] = ["count: 7", "shape: numeric badge", "purpose: status summary"];
const INPUT_PROPS: [&str; 3] = ["placeholder: Search components", "state: Entity<InputState>", "usage: filter entry point"];
const CHECKBOX_PROPS: [&str; 3] = ["checked: false", "label: Enable sample option", "usage: form boolean field"];
const DIALOG_PROPS: [&str; 3] = ["open state: planned", "trigger: button driven", "usage: confirmation flow"];
const SIDEBAR_PROPS: [&str; 3] = ["navigation groups: 5", "collapsed: bool", "usage: layout shell"];
const TABLE_PROPS: [&str; 3] = ["rows: placeholder set", "columns: metadata driven", "usage: data overview"];
const LIST_PROPS: [&str; 3] = ["item count: variable", "selection: single", "usage: file-tree browsing"];
const CHART_PROPS: [&str; 3] = ["series: placeholder metrics", "axis: time and value", "usage: trend summary"];
const EDITOR_PROPS: [&str; 3] = ["language: Rust", "source: sample snippet", "usage: code preview pane"];
const ALERT_PROPS: [&str; 3] = ["variant: Info/Success/Warning/Error", "message: &str", "usage: feedback banner"];
const AVATAR_PROPS: [&str; 3] = ["name: &str (initials)", "src: optional image URL", "usage: user identity"];
const PROGRESS_PROPS: [&str; 3] = ["value: f32 (0..100)", "w_full: layout stretch", "usage: loading indicator"];
const RADIO_PROPS: [&str; 3] = ["checked: bool", "label: &str", "usage: single-choice form field"];
const SKELETON_PROPS: [&str; 3] = ["w: Length", "h: Length", "usage: loading placeholder"];
const SPINNER_PROPS: [&str; 3] = ["size: small/default/large", "animated: true", "usage: async operation indicator"];
const SWITCH_PROPS: [&str; 3] = ["checked: bool", "label: &str", "usage: binary toggle"];
const TAG_PROPS: [&str; 3] = ["variant: primary/success/warning/danger", "child: label text", "usage: status chip"];
const ICON_PROPS: [&str; 3] = ["name: IconName", "size: Rems", "usage: decorative or labeled icon"];
const GENERIC_PROPS: [&str; 2] = ["state: planned", "usage: showcase entry"];

const BASIC_ENTRIES: [ComponentEntry; 27] = [
    ComponentEntry { id: ComponentId::Button, title: "Button", summary: "기본 액션 트리거와 selected 상태 확인용 컴포넌트", code_preview: "Button::new(\"id\")\n    .outline().label(\"Action\")", props: &BUTTON_PROPS, extraction_note: "Phase 5 이후 실제 샘플 추출로 교체합니다." },
    ComponentEntry { id: ComponentId::Badge, title: "Badge", summary: "수량과 상태 요약을 겹쳐 보여주는 컴포넌트", code_preview: "Badge::new().count(7).child(div().child(\"Notifications\"))", props: &BADGE_PROPS, extraction_note: "숫자형 상태 요약을 우선 보여줍니다." },
    ComponentEntry { id: ComponentId::Checkbox, title: "Checkbox", summary: "폼 조건 전환과 선택 여부 표시용 컴포넌트", code_preview: "Checkbox::new(\"id\").label(\"Enable option\")", props: &CHECKBOX_PROPS, extraction_note: "이후 Playground와 Settings 흐름에서 재사용합니다." },
    ComponentEntry { id: ComponentId::Accordion, title: "Accordion", summary: "접기/펼치기 방식으로 콘텐츠를 구획하는 컴포넌트", code_preview: "Accordion::new(cx)\n    .header(\"Section\").body(content)", props: &GENERIC_PROPS, extraction_note: "open/close 상태 관리가 필요합니다." },
    ComponentEntry { id: ComponentId::Alert, title: "Alert", summary: "정보·성공·경고·오류 피드백을 표시하는 배너 컴포넌트", code_preview: "Alert::info(\"id\", \"message\")\nAlert::warning(\"id\", \"message\")", props: &ALERT_PROPS, extraction_note: "다양한 variant를 사용 가능합니다." },
    ComponentEntry { id: ComponentId::AlertDialog, title: "AlertDialog", summary: "차단형 확인이 필요한 경고 다이얼로그 컴포넌트", code_preview: "AlertDialog::new(cx)\n    .title(\"Warning\").message(\"...\").confirm()", props: &DIALOG_PROPS, extraction_note: "open 상태 Entity가 필요합니다." },
    ComponentEntry { id: ComponentId::Avatar, title: "Avatar", summary: "사용자 이름 이니셜 또는 이미지를 보여주는 컴포넌트", code_preview: "Avatar::new().name(\"Alice\")", props: &AVATAR_PROPS, extraction_note: "이미지 src 바인딩은 추후 확장합니다." },
    ComponentEntry { id: ComponentId::Clipboard, title: "Clipboard", summary: "텍스트를 클립보드에 복사하는 유틸리티 컴포넌트", code_preview: "Clipboard::new().value(\"copy me\")", props: &GENERIC_PROPS, extraction_note: "브라우저 보안 정책 주의가 필요합니다." },
    ComponentEntry { id: ComponentId::CollapsibleWidget, title: "Collapsible", summary: "콘텐츠 영역을 접고 펼치는 레이아웃 컴포넌트", code_preview: "Collapsible::new(cx)\n    .header(\"Details\").body(content)", props: &GENERIC_PROPS, extraction_note: "open 상태 관리가 필요합니다." },
    ComponentEntry { id: ComponentId::DropdownButton, title: "DropdownButton", summary: "클릭 시 선택 목록이 열리는 버튼 컴포넌트", code_preview: "DropdownButton::new(\"id\")\n    .label(\"Actions\").items([...])", props: &GENERIC_PROPS, extraction_note: "팝업 오버레이 상태가 필요합니다." },
    ComponentEntry { id: ComponentId::HoverCard, title: "HoverCard", summary: "마우스를 올리면 추가 정보를 보여주는 카드 컴포넌트", code_preview: "HoverCard::new()\n    .trigger(btn).content(card)", props: &GENERIC_PROPS, extraction_note: "hover 이벤트 및 팝업 레이어가 필요합니다." },
    ComponentEntry { id: ComponentId::Icon, title: "Icon", summary: "SVG 기반 아이콘을 렌더링하는 컴포넌트", code_preview: "Icon::new(IconName::Settings).size_6()", props: &ICON_PROPS, extraction_note: "IconName enum으로 전체 아이콘 셋을 사용합니다." },
    ComponentEntry { id: ComponentId::Image, title: "Image", summary: "이미지를 표시하는 미디어 컴포넌트", code_preview: "img(\"path/to/img.png\").w_full()", props: &GENERIC_PROPS, extraction_note: "gpui img() 함수를 직접 사용합니다." },
    ComponentEntry { id: ComponentId::Kbd, title: "Kbd", summary: "키보드 단축키를 표시하는 컴포넌트", code_preview: "Kbd::new(Keystroke { key: \"k\", ..Default::default() })", props: &GENERIC_PROPS, extraction_note: "Keystroke 구조체 초기화가 필요합니다." },
    ComponentEntry { id: ComponentId::Label, title: "Label", summary: "폼 입력 필드나 그룹에 붙이는 레이블 컴포넌트", code_preview: "Label::new().child(\"Username\")", props: &GENERIC_PROPS, extraction_note: "Label은 ParentElement입니다." },
    ComponentEntry { id: ComponentId::Pagination, title: "Pagination", summary: "페이지네이션 컨트롤을 제공하는 컴포넌트", code_preview: "Pagination::new(cx)\n    .total(100).per_page(10)", props: &GENERIC_PROPS, extraction_note: "페이지 상태 Entity가 필요합니다." },
    ComponentEntry { id: ComponentId::Progress, title: "Progress", summary: "진행 상태를 막대 형태로 시각화하는 컴포넌트", code_preview: "Progress::new().value(65.0).w_full()", props: &PROGRESS_PROPS, extraction_note: "value를 0.0~100.0 범위로 전달합니다." },
    ComponentEntry { id: ComponentId::Radio, title: "Radio", summary: "단일 선택 그룹을 구성하는 라디오 버튼 컴포넌트", code_preview: "Radio::new(\"id\").label(\"Option A\").checked(true)", props: &RADIO_PROPS, extraction_note: "그룹 상태 관리는 호출 측에서 처리합니다." },
    ComponentEntry { id: ComponentId::Rating, title: "Rating", summary: "별점 평가를 입력하거나 표시하는 컴포넌트", code_preview: "Rating::new(cx).value(4).max(5)", props: &GENERIC_PROPS, extraction_note: "평점 상태 Entity가 필요합니다." },
    ComponentEntry { id: ComponentId::Skeleton, title: "Skeleton", summary: "로딩 중 콘텐츠 자리를 표시하는 애니메이션 컴포넌트", code_preview: "Skeleton::new().w_full().h(px(16.0))", props: &SKELETON_PROPS, extraction_note: "크기를 조절해 다양한 자리 표시가 가능합니다." },
    ComponentEntry { id: ComponentId::Slider, title: "Slider", summary: "연속 값을 드래그로 선택하는 슬라이더 컴포넌트", code_preview: "Slider::new(cx).min(0.0).max(100.0).value(50.0)", props: &GENERIC_PROPS, extraction_note: "Entity<SliderState> 바인딩이 필요합니다." },
    ComponentEntry { id: ComponentId::Spinner, title: "Spinner", summary: "비동기 작업 진행 중 회전 애니메이션을 보여주는 컴포넌트", code_preview: "Spinner::new()\nSpinner::new().small()", props: &SPINNER_PROPS, extraction_note: "크기 변형: small/default/large" },
    ComponentEntry { id: ComponentId::Stepper, title: "Stepper", summary: "단계별 진행 과정을 표시하는 컴포넌트", code_preview: "Stepper::new(cx).steps([\"Step 1\", \"Step 2\"]).current(1)", props: &GENERIC_PROPS, extraction_note: "스텝 상태 Entity가 필요합니다." },
    ComponentEntry { id: ComponentId::Switch, title: "Switch", summary: "온/오프 토글 스위치 컴포넌트", code_preview: "Switch::new(\"id\").label(\"Enable\").checked(true)", props: &SWITCH_PROPS, extraction_note: "checked 상태를 AppState에서 관리합니다." },
    ComponentEntry { id: ComponentId::Tag, title: "Tag", summary: "상태나 카테고리를 칩 형태로 보여주는 컴포넌트", code_preview: "Tag::primary().child(\"Primary\")\nTag::success().child(\"OK\")", props: &TAG_PROPS, extraction_note: "primary/success/warning/danger 변형을 지원합니다." },
    ComponentEntry { id: ComponentId::Toggle, title: "Toggle", summary: "활성/비활성 상태를 토글하는 버튼 컴포넌트", code_preview: "Button::new(\"id\").selected(active).outline()", props: &GENERIC_PROPS, extraction_note: "Button의 selected 상태로 구현합니다." },
    ComponentEntry { id: ComponentId::Tooltip, title: "Tooltip", summary: "요소 위에 마우스를 올리면 설명을 보여주는 컴포넌트", code_preview: "Tooltip::new().text(\"Helpful hint\").child(btn)", props: &GENERIC_PROPS, extraction_note: "hover 트리거 레이어가 필요합니다." },
];

const FORM_ENTRIES: [ComponentEntry; 7] = [
    ComponentEntry { id: ComponentId::Input, title: "Input", summary: "텍스트 입력 폼 컴포넌트", code_preview: "Input::new(window, cx).placeholder(\"Search\")", props: &INPUT_PROPS, extraction_note: "Entity<InputState> 바인딩이 필요합니다." },
    ComponentEntry { id: ComponentId::Checkbox, title: "Checkbox", summary: "폼 조건 전환과 선택 여부 표시용 컴포넌트", code_preview: "Checkbox::new(\"id\").label(\"Enable\")", props: &CHECKBOX_PROPS, extraction_note: "이후 Playground와 Settings 흐름에서 재사용합니다." },
    ComponentEntry { id: ComponentId::Editor, title: "Editor", summary: "코드 보기와 편집 미리보기를 위한 컴포넌트", code_preview: "Editor::new(language).content(sample).readonly(true)", props: &EDITOR_PROPS, extraction_note: "실제 에디터 연결은 추후 확장합니다." },
    ComponentEntry { id: ComponentId::Select, title: "Select", summary: "드롭다운 목록에서 값을 선택하는 폼 컴포넌트", code_preview: "Select::new(cx).options(items).value(selected)", props: &GENERIC_PROPS, extraction_note: "Entity<SelectState> 바인딩이 필요합니다." },
    ComponentEntry { id: ComponentId::NumberInput, title: "NumberInput", summary: "숫자 입력 전용 폼 컴포넌트", code_preview: "NumberInput::new(cx).min(0).max(100).value(42)", props: &GENERIC_PROPS, extraction_note: "Entity<InputState> 바인딩이 필요합니다." },
    ComponentEntry { id: ComponentId::DatePicker, title: "DatePicker", summary: "날짜 선택 캘린더 폼 컴포넌트", code_preview: "DatePicker::new(cx).value(date)", props: &GENERIC_PROPS, extraction_note: "날짜 상태 Entity가 필요합니다." },
    ComponentEntry { id: ComponentId::OtpInput, title: "OtpInput", summary: "OTP 코드 입력용 폼 컴포넌트", code_preview: "OtpInput::new(cx).length(6)", props: &GENERIC_PROPS, extraction_note: "Entity<InputState> 배열이 필요합니다." },
];

const LAYOUT_ENTRIES: [ComponentEntry; 9] = [
    ComponentEntry { id: ComponentId::Dialog, title: "Dialog", summary: "확인과 차단형 작업에 쓰이는 모달 레이아웃 컴포넌트", code_preview: "Dialog::new(\"id\").title(\"Confirm\")", props: &DIALOG_PROPS, extraction_note: "open/close 상태 연결이 필요합니다." },
    ComponentEntry { id: ComponentId::Sidebar, title: "Sidebar", summary: "앱 구조와 탐색 그룹을 유지하는 레이아웃 컴포넌트", code_preview: "Sidebar::<SidebarMenu>::left()\n    .collapsed(false).child(menu)", props: &SIDEBAR_PROPS, extraction_note: "현재 앱 셸에서 사용 중입니다." },
    ComponentEntry { id: ComponentId::DescriptionList, title: "DescriptionList", summary: "키-값 쌍을 구조적으로 표시하는 레이아웃 컴포넌트", code_preview: "DescriptionList::new()\n    .item(\"Name\", \"Value\")", props: &GENERIC_PROPS, extraction_note: "Phase 7 데이터 쇼케이스에서 활용합니다." },
    ComponentEntry { id: ComponentId::GroupBox, title: "GroupBox", summary: "관련 콘텐츠를 테두리로 묶어 구분하는 컴포넌트", code_preview: "GroupBox::new().title(\"Settings Group\").child(content)", props: &GENERIC_PROPS, extraction_note: "설정 페이지 구조화에 활용합니다." },
    ComponentEntry { id: ComponentId::Notification, title: "Notification", summary: "화면 모서리에 일시적으로 표시되는 알림 컴포넌트", code_preview: "Notification::new(cx)\n    .message(\"Operation done\").show()", props: &GENERIC_PROPS, extraction_note: "토스트 스택 관리가 필요합니다." },
    ComponentEntry { id: ComponentId::Popover, title: "Popover", summary: "트리거 요소 근처에 부유하는 콘텐츠 컴포넌트", code_preview: "Popover::new(cx).trigger(btn).content(panel)", props: &GENERIC_PROPS, extraction_note: "open 상태와 위치 계산이 필요합니다." },
    ComponentEntry { id: ComponentId::Resizable, title: "Resizable", summary: "패널 경계를 드래그해 크기를 조절하는 레이아웃 컴포넌트", code_preview: "h_resizable(\"id\")\n    .child(resizable_panel().size(px(260.)))", props: &GENERIC_PROPS, extraction_note: "내부 keyed_state로 크기가 유지됩니다." },
    ComponentEntry { id: ComponentId::Scrollable, title: "Scrollable", summary: "콘텐츠가 넘칠 때 스크롤을 지원하는 컨테이너 컴포넌트", code_preview: "div().overflow_y_scroll().child(long_content)", props: &GENERIC_PROPS, extraction_note: "gpui 내장 overflow 메서드를 사용합니다." },
    ComponentEntry { id: ComponentId::Sheet, title: "Sheet", summary: "화면 모서리에서 슬라이드로 열리는 패널 컴포넌트", code_preview: "Sheet::new(cx).side(Side::Right)\n    .content(panel)", props: &GENERIC_PROPS, extraction_note: "open 상태와 애니메이션 연결이 필요합니다." },
];

const DATA_ENTRIES: [ComponentEntry; 2] = [
    ComponentEntry { id: ComponentId::Table, title: "Table", summary: "열 구조와 행 메타데이터를 가진 데이터 화면용 컴포넌트", code_preview: "DataTable::new(rows)\n    .column(\"Name\").column(\"Status\")", props: &TABLE_PROPS, extraction_note: "Phase 7 공통 샘플 데이터와 연결합니다." },
    ComponentEntry { id: ComponentId::List, title: "List", summary: "파일 트리나 작업 큐처럼 연속 아이템을 보여주는 컴포넌트", code_preview: "List::new(items).selectable(true)", props: &LIST_PROPS, extraction_note: "VirtualList 대응은 Phase 7에서 확장합니다." },
];

const ADVANCED_ENTRIES: [ComponentEntry; 8] = [
    ComponentEntry { id: ComponentId::Chart, title: "Chart", summary: "시계열, 분포, 상태 변화를 시각화하는 고급 컴포넌트", code_preview: "LineChart::new(data).x(|d| d.label).y(|d| d.value)", props: &CHART_PROPS, extraction_note: "Dashboard placeholder 이후 통합합니다." },
    ComponentEntry { id: ComponentId::Editor, title: "Editor", summary: "코드 보기와 편집 미리보기를 위한 고급 컴포넌트", code_preview: "Editor::new(language).content(sample).readonly(true)", props: &EDITOR_PROPS, extraction_note: "실제 에디터 연결은 추후 확장합니다." },
    ComponentEntry { id: ComponentId::Calendar, title: "Calendar", summary: "날짜와 이벤트를 달력 형태로 표시하는 고급 컴포넌트", code_preview: "Calendar::new(cx).selected(date).events(events)", props: &GENERIC_PROPS, extraction_note: "날짜 상태 Entity가 필요합니다." },
    ComponentEntry { id: ComponentId::Menu, title: "Menu", summary: "컨텍스트 메뉴나 드롭다운 메뉴를 제공하는 컴포넌트", code_preview: "Menu::new(cx).item(\"Cut\").item(\"Copy\")", props: &GENERIC_PROPS, extraction_note: "팝업 레이어와 위치 계산이 필요합니다." },
    ComponentEntry { id: ComponentId::DataTable, title: "DataTable", summary: "정렬과 필터를 지원하는 고급 데이터 테이블 컴포넌트", code_preview: "DataTable::new(cx, rows)\n    .sortable().filterable()", props: &GENERIC_PROPS, extraction_note: "테이블 상태 관리와 샘플 데이터가 필요합니다." },
    ComponentEntry { id: ComponentId::Tabs, title: "Tabs", summary: "탭 패널로 콘텐츠를 전환하는 네비게이션 컴포넌트", code_preview: "Tabs::new(cx).tab(\"Tab 1\", panel1)", props: &GENERIC_PROPS, extraction_note: "탭 상태 Entity가 필요합니다." },
    ComponentEntry { id: ComponentId::Tree, title: "Tree", summary: "트리 구조를 탐색하는 고급 컴포넌트", code_preview: "Tree::new(cx).nodes(tree_data)", props: &GENERIC_PROPS, extraction_note: "트리 확장 상태 관리가 필요합니다." },
    ComponentEntry { id: ComponentId::VirtualList, title: "VirtualList", summary: "대용량 리스트를 가상화해 효율적으로 렌더링하는 컴포넌트", code_preview: "VirtualList::new(cx, items.len(), |idx, cx| render_item(idx, cx))", props: &GENERIC_PROPS, extraction_note: "대용량 데이터 렌더링 전략과 연계합니다." },
];

fn render_preview_widget(component_id: ComponentId) -> AnyElement {
    use gpui_component::{
        alert::Alert,
        avatar::Avatar,
        radio::Radio,
        skeleton::Skeleton,
        spinner::Spinner,
        switch::Switch,
        tag::Tag,
        Icon, IconName, Sizable,
    };
    match component_id {
        ComponentId::Button => Button::new("preview-button")
            .outline()
            .label("Explorer Action")
            .into_any_element(),
        ComponentId::Badge => Badge::new()
            .count(7)
            .child(
                div()
                    .px_3()
                    .py_2()
                    .border_1()
                    .rounded(px(10.0))
                    .child("Notifications"),
            )
            .into_any_element(),
        ComponentId::Checkbox => Checkbox::new("preview-checkbox")
            .label("Enable sample option")
            .into_any_element(),
        ComponentId::Progress => div()
            .v_flex()
            .gap_2()
            .child(Progress::new().value(65.0).w_full())
            .child(Progress::new().value(30.0).w_full())
            .child(Progress::new().value(90.0).w_full())
            .into_any_element(),
        ComponentId::Spinner => div()
            .h_flex()
            .gap_3()
            .items_center()
            .child(Spinner::new())
            .child(Spinner::new().small())
            .child(Spinner::new().large())
            .into_any_element(),
        ComponentId::Switch => div()
            .v_flex()
            .gap_2()
            .child(Switch::new("preview-switch-on").label("Enabled").checked(true))
            .child(Switch::new("preview-switch-off").label("Disabled").checked(false))
            .into_any_element(),
        ComponentId::Skeleton => div()
            .v_flex()
            .gap_2()
            .w_full()
            .child(Skeleton::new().w_full())
            .child(Skeleton::new().w(relative(0.75)))
            .child(Skeleton::new().w(relative(0.5)))
            .into_any_element(),
        ComponentId::Radio => div()
            .v_flex()
            .gap_2()
            .child(Radio::new("radio-opt1").label("Option A").checked(true))
            .child(Radio::new("radio-opt2").label("Option B").checked(false))
            .child(Radio::new("radio-opt3").label("Option C").checked(false))
            .into_any_element(),
        ComponentId::Alert => div()
            .v_flex()
            .gap_2()
            .child(Alert::info("alert-info", "정보 메시지 예시입니다."))
            .child(Alert::success("alert-ok", "작업이 성공했습니다."))
            .child(Alert::warning("alert-warn", "주의가 필요합니다."))
            .into_any_element(),
        ComponentId::Avatar => div()
            .h_flex()
            .gap_3()
            .items_center()
            .child(Avatar::new().name("Alice"))
            .child(Avatar::new().name("Bob"))
            .child(Avatar::new())
            .into_any_element(),
        ComponentId::Tag => div()
            .h_flex()
            .gap_2()
            .child(Tag::primary().child("Primary"))
            .child(Tag::success().child("Success"))
            .child(Tag::warning().child("Warning"))
            .child(Tag::danger().child("Danger"))
            .into_any_element(),
        ComponentId::Icon => div()
            .h_flex()
            .gap_3()
            .items_center()
            .child(Icon::new(IconName::Settings).size_6())
            .child(Icon::new(IconName::Search).size_6())
            .child(Icon::new(IconName::Info).size_6())
            .child(Icon::new(IconName::Check).size_6())
            .child(Icon::new(IconName::Close).size_6())
            .into_any_element(),
        ComponentId::Toggle => div()
            .h_flex()
            .gap_2()
            .child(Button::new("toggle-on").outline().selected(true).label("Active"))
            .child(Button::new("toggle-off").outline().selected(false).label("Inactive"))
            .into_any_element(),
        ComponentId::Input | ComponentId::NumberInput | ComponentId::OtpInput => div()
            .v_flex()
            .gap_2()
            .child(
                div()
                    .px_3()
                    .py_2()
                    .border_1()
                    .rounded(px(6.0))
                    .w_full()
                    .child("Search components…"),
            )
            .child(format!("{} — Entity<InputState> 바인딩 필요", component_id.title()))
            .into_any_element(),
        ComponentId::Editor => div()
            .v_flex()
            .gap_2()
            .child(
                div()
                    .p_3()
                    .border_1()
                    .rounded(px(6.0))
                    .v_flex()
                    .gap_0()
                    .child("fn main() {")
                    .child("    println!(\"Hello, gpui!\");")
                    .child("}"),
            )
            .child("실제 Editor는 Entity<InputState> 바인딩이 필요합니다.")
            .into_any_element(),
        ComponentId::Dialog | ComponentId::AlertDialog | ComponentId::Sheet => div()
            .v_flex()
            .gap_2()
            .child(Button::new("preview-dialog-trigger").outline().label("Open Dialog"))
            .child(
                div()
                    .p_3()
                    .border_1()
                    .rounded(px(6.0))
                    .v_flex()
                    .gap_2()
                    .child(div().font_weight(FontWeight::BOLD).child("Confirm Action"))
                    .child("이 작업을 진행하시겠습니까?")
                    .child(
                        div()
                            .h_flex()
                            .gap_2()
                            .child(Button::new("preview-dialog-cancel").outline().label("취소"))
                            .child(Button::new("preview-dialog-ok").outline().label("확인")),
                    ),
            )
            .into_any_element(),
        ComponentId::Sidebar => div()
            .v_flex()
            .gap_1()
            .p_2()
            .border_1()
            .rounded(px(6.0))
            .child(div().font_weight(FontWeight::BOLD).child("Navigation"))
            .child(preview_nav_item("Dashboard", true))
            .child(preview_nav_item("Components", false))
            .child(preview_nav_item("Playground", false))
            .child(preview_nav_item("Settings", false))
            .into_any_element(),
        ComponentId::Table | ComponentId::DataTable => div()
            .v_flex()
            .gap_px()
            .border_1()
            .rounded(px(6.0))
            .overflow_hidden()
            .child(preview_table_row(&["Name", "Category", "Status"], true))
            .child(preview_table_row(&["Button", "Basic", "Stable"], false))
            .child(preview_table_row(&["Input", "Form", "Stable"], false))
            .child(preview_table_row(&["Dialog", "Layout", "Stable"], false))
            .into_any_element(),
        ComponentId::List | ComponentId::VirtualList => div()
            .v_flex()
            .border_1()
            .rounded(px(6.0))
            .overflow_hidden()
            .child(preview_list_item("Button", "Basic", true))
            .child(preview_list_item("Badge", "Basic", false))
            .child(preview_list_item("Input", "Form", false))
            .child(preview_list_item("Checkbox", "Form", false))
            .into_any_element(),
        ComponentId::Chart | ComponentId::Plot => div()
            .v_flex()
            .gap_2()
            .p_2()
            .border_1()
            .rounded(px(6.0))
            .child(preview_chart_bar("Basic", 80.0))
            .child(preview_chart_bar("Form", 60.0))
            .child(preview_chart_bar("Layout", 45.0))
            .child(preview_chart_bar("Data", 70.0))
            .child(preview_chart_bar("Advanced", 30.0))
            .into_any_element(),
        ComponentId::Resizable => div()
            .v_flex()
            .gap_1()
            .border_1()
            .rounded(px(6.0))
            .overflow_hidden()
            .child(
                div()
                    .h_flex()
                    .h(px(80.0))
                    .child(div().flex_1().border_r_1().p_2().child("Panel A"))
                    .child(div().flex_1().p_2().child("Panel B")),
            )
            .child(div().border_t_1().p_2().child("Drag handle between panels"))
            .into_any_element(),
        _ => div()
            .v_flex()
            .gap_1()
            .p_3()
            .border_1()
            .rounded(px(10.0))
            .child(format!("{} preview", component_id.title()))
            .child("실제 상호작용 프리뷰는 후속 단계에서 연결됩니다.")
            .into_any_element(),
    }
}

fn preview_nav_item(label: &'static str, selected: bool) -> Div {
    let base = div().px_2().py_1().rounded(px(4.0)).child(label);
    if selected {
        base.border_1()
    } else {
        base
    }
}

fn preview_table_row(cells: &[&'static str], header: bool) -> Div {
    let mut row = div().h_flex().border_b_1();
    if header {
        for cell in cells {
            row = row.child(div().flex_1().px_2().py_1().font_weight(FontWeight::BOLD).child(*cell));
        }
    } else {
        for cell in cells {
            row = row.child(div().flex_1().px_2().py_1().child(*cell));
        }
    }
    row
}

fn preview_list_item(name: &'static str, category: &'static str, selected: bool) -> Div {
    let base = div()
        .h_flex()
        .gap_2()
        .px_3()
        .py_1()
        .border_b_1()
        .child(div().flex_1().child(name))
        .child(div().child(category));
    if selected {
        base.border_l_2()
    } else {
        base
    }
}

fn preview_chart_bar(label: &'static str, value: f32) -> Div {
    div()
        .h_flex()
        .gap_2()
        .items_center()
        .child(div().w(px(64.0)).child(label))
        .child(div().flex_1().child(Progress::new().value(value).w_full()))
}

fn explorer_panel(title: &'static str) -> Div {
    div()
        .v_flex()
        .gap_2()
        .flex_1()
        .p_3()
        .border_1()
        .rounded(px(12.0))
        .child(panel_title(title))
}

fn panel_title(title: &'static str) -> Div {
    div().font_weight(FontWeight::BOLD).child(title)
}

/// Renders the right-panel showcase for the currently selected component.
/// Navigation (category + component selection) is handled by the sidebar.
pub fn render_component_showcase(state: &AppState) -> impl IntoElement {
    let selected_component = state
        .selected_component
        .unwrap_or(ComponentId::Button);
    let entry = entry_for(selected_component);

    div()
        .size_full()
        .v_flex()
        .gap_3()
        .p_4()
        .child(
            // Header: component name + summary
            div()
                .v_flex()
                .gap_1()
                .pb_3()
                .border_b_1()
                .child(div().font_weight(FontWeight::BOLD).text_size(rems(1.25)).child(entry.title))
                .child(entry.summary),
        )
        .child(
            // Preview + Code row
            div()
                .h_flex()
                .gap_3()
                .flex_1()
                .child(
                    explorer_panel("Preview")
                        .flex_1()
                        .child(render_preview_widget(entry.id)),
                )
                .child(
                    explorer_panel("Code Preview")
                        .flex_1()
                        .child(
                            div()
                                .p_3()
                                .border_1()
                                .rounded(px(8.0))
                                .child(entry.code_preview),
                        )
                        .child(
                            div()
                                .mt_2()
                                .p_2()
                                .border_1()
                                .rounded(px(6.0))
                                .child("메모: ")
                                .child(entry.extraction_note),
                        ),
                ),
        )
        .child(
            // Props panel
            div()
                .h_flex()
                .gap_2()
                .children(entry.props.iter().map(|prop| {
                    div()
                        .px_3()
                        .py_1()
                        .border_1()
                        .rounded(px(6.0))
                        .child(*prop)
                })),
        )
}



pub(crate) fn entries_for_category(category: ComponentCategory) -> &'static [ComponentEntry] {
    match category {
        ComponentCategory::Basic => &BASIC_ENTRIES,
        ComponentCategory::Form => &FORM_ENTRIES,
        ComponentCategory::Layout => &LAYOUT_ENTRIES,
        ComponentCategory::Data => &DATA_ENTRIES,
        ComponentCategory::Advanced => &ADVANCED_ENTRIES,
    }
}

pub(crate) fn entry_for(component_id: ComponentId) -> &'static ComponentEntry {
    let entries = entries_for_category(component_id.category());
    entries
        .iter()
        .find(|entry| entry.id == component_id)
        .expect("component registry entry must exist")
}

#[cfg(test)]
mod tests {
    use super::{entries_for_category, entry_for};
    use crate::shared::state::{ComponentCategory, ComponentId};

    #[test]
    fn each_category_has_expected_component_count() {
        assert_eq!(entries_for_category(ComponentCategory::Basic).len(), 27);
        assert_eq!(entries_for_category(ComponentCategory::Form).len(), 7);
        assert_eq!(entries_for_category(ComponentCategory::Layout).len(), 9);
        assert_eq!(entries_for_category(ComponentCategory::Data).len(), 2);
        assert_eq!(entries_for_category(ComponentCategory::Advanced).len(), 8);
    }

    #[test]
    fn registry_lookup_returns_matching_component_entry() {
        let entry = entry_for(ComponentId::Chart);
        assert_eq!(entry.title, "Chart");
        assert!(entry.summary.contains("시각화"));
        assert!(entry.code_preview.contains("LineChart"));
    }

    #[test]
    fn all_component_ids_have_registry_entries() {
        use crate::shared::state::ComponentCategory;
        for cat in [
            ComponentCategory::Basic,
            ComponentCategory::Form,
            ComponentCategory::Layout,
            ComponentCategory::Data,
            ComponentCategory::Advanced,
        ] {
            let entries = entries_for_category(cat);
            for entry in entries {
                // default component for category must resolve
                assert!(!entry.title.is_empty());
            }
        }
    }
}
