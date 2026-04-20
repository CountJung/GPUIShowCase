use crate::shared::logger::{AppLogLevel, LoggerRuntime};

const PLAYGROUND_LOG_LIMIT: usize = 8;
const UI_LOG_LIMIT: usize = 24;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AppRoute {
    #[default]
    Dashboard,
    Components,
    Playground,
    Data,
    Settings,
    Logs,
}

impl AppRoute {
    pub const ALL: [Self; 6] = [
        Self::Dashboard,
        Self::Components,
        Self::Playground,
        Self::Data,
        Self::Settings,
        Self::Logs,
    ];

    pub fn title(self) -> &'static str {
        match self {
            Self::Dashboard => "Dashboard",
            Self::Components => "Components",
            Self::Playground => "Playground",
            Self::Data => "Data Showcase",
            Self::Settings => "Settings",
            Self::Logs => "Logs",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::Dashboard => "프로젝트 상태와 기본 요약",
            Self::Components => "컴포넌트 탐색과 선택 흐름",
            Self::Playground => "상태 변화와 이벤트 실험 공간",
            Self::Data => "공통 데이터셋 기반의 표, 차트, 리스트 실험 화면",
            Self::Settings => "사용자 설정과 적용 상태",
            Self::Logs => "롤링 파일 로그와 UI 로그 뷰",
        }
    }

    pub fn default_component_selection(self) -> Option<(ComponentCategory, ComponentId)> {
        match self {
            Self::Components => Some((ComponentCategory::Basic, ComponentId::Button)),
            _ => None,
        }
    }

    pub fn from_label(label: &str) -> Option<Self> {
        match label.trim().to_ascii_lowercase().as_str() {
            "dashboard" => Some(Self::Dashboard),
            "components" => Some(Self::Components),
            "playground" => Some(Self::Playground),
            "data" | "data-showcase" | "datashowcase" => Some(Self::Data),
            "settings" => Some(Self::Settings),
            "logs" => Some(Self::Logs),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ComponentCategory {
    #[default]
    Basic,
    Form,
    Layout,
    Data,
    Advanced,
}

impl ComponentCategory {
    pub fn title(self) -> &'static str {
        match self {
            Self::Basic => "Basic",
            Self::Form => "Form",
            Self::Layout => "Layout",
            Self::Data => "Data",
            Self::Advanced => "Advanced",
        }
    }

    pub fn from_label(label: &str) -> Option<Self> {
        match label.trim().to_ascii_lowercase().as_str() {
            "basic" => Some(Self::Basic),
            "form" => Some(Self::Form),
            "layout" => Some(Self::Layout),
            "data" => Some(Self::Data),
            "advanced" => Some(Self::Advanced),
            _ => None,
        }
    }

    pub fn default_component(self) -> ComponentId {
        match self {
            Self::Basic => ComponentId::Button,
            Self::Form => ComponentId::Input,
            Self::Layout => ComponentId::Dialog,
            Self::Data => ComponentId::Table,
            Self::Advanced => ComponentId::Chart,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ComponentId {
    // Basic
    #[default]
    Button,
    Badge,
    Checkbox,
    Accordion,
    Alert,
    AlertDialog,
    Avatar,
    Clipboard,
    CollapsibleWidget,
    DropdownButton,
    HoverCard,
    Icon,
    Image,
    Kbd,
    Label,
    Pagination,
    Progress,
    Radio,
    Rating,
    Skeleton,
    Slider,
    Spinner,
    Stepper,
    Switch,
    Tag,
    Toggle,
    Tooltip,
    // Form
    Input,
    Editor,
    Select,
    NumberInput,
    DatePicker,
    OtpInput,
    ColorPicker,
    // Layout
    Dialog,
    Sidebar,
    DescriptionList,
    GroupBox,
    Notification,
    Popover,
    Resizable,
    Scrollable,
    Sheet,
    // Data
    Table,
    List,
    // Advanced
    Chart,
    Calendar,
    Menu,
    DataTable,
    Tabs,
    Tree,
    VirtualList,
    Plot,
}

impl ComponentId {
    pub fn title(self) -> &'static str {
        match self {
            Self::Button => "Button",
            Self::Badge => "Badge",
            Self::Checkbox => "Checkbox",
            Self::Accordion => "Accordion",
            Self::Alert => "Alert",
            Self::AlertDialog => "AlertDialog",
            Self::Avatar => "Avatar",
            Self::Clipboard => "Clipboard",
            Self::CollapsibleWidget => "Collapsible",
            Self::DropdownButton => "DropdownButton",
            Self::HoverCard => "HoverCard",
            Self::Icon => "Icon",
            Self::Image => "Image",
            Self::Kbd => "Kbd",
            Self::Label => "Label",
            Self::Pagination => "Pagination",
            Self::Progress => "Progress",
            Self::Radio => "Radio",
            Self::Rating => "Rating",
            Self::Skeleton => "Skeleton",
            Self::Slider => "Slider",
            Self::Spinner => "Spinner",
            Self::Stepper => "Stepper",
            Self::Switch => "Switch",
            Self::Tag => "Tag",
            Self::Toggle => "Toggle",
            Self::Tooltip => "Tooltip",
            Self::Input => "Input",
            Self::Editor => "Editor",
            Self::Select => "Select",
            Self::NumberInput => "NumberInput",
            Self::DatePicker => "DatePicker",
            Self::OtpInput => "OtpInput",
            Self::ColorPicker => "ColorPicker",
            Self::Dialog => "Dialog",
            Self::Sidebar => "Sidebar",
            Self::DescriptionList => "DescriptionList",
            Self::GroupBox => "GroupBox",
            Self::Notification => "Notification",
            Self::Popover => "Popover",
            Self::Resizable => "Resizable",
            Self::Scrollable => "Scrollable",
            Self::Sheet => "Sheet",
            Self::Table => "Table",
            Self::List => "List",
            Self::Chart => "Chart",
            Self::Calendar => "Calendar",
            Self::Menu => "Menu",
            Self::DataTable => "DataTable",
            Self::Tabs => "Tabs",
            Self::Tree => "Tree",
            Self::VirtualList => "VirtualList",
            Self::Plot => "Plot",
        }
    }

    pub fn category(self) -> ComponentCategory {
        match self {
            Self::Button
            | Self::Badge
            | Self::Checkbox
            | Self::Accordion
            | Self::Alert
            | Self::AlertDialog
            | Self::Avatar
            | Self::Clipboard
            | Self::CollapsibleWidget
            | Self::DropdownButton
            | Self::HoverCard
            | Self::Icon
            | Self::Image
            | Self::Kbd
            | Self::Label
            | Self::Pagination
            | Self::Progress
            | Self::Radio
            | Self::Rating
            | Self::Skeleton
            | Self::Slider
            | Self::Spinner
            | Self::Stepper
            | Self::Switch
            | Self::Tag
            | Self::Toggle
            | Self::Tooltip => ComponentCategory::Basic,
            Self::Input
            | Self::Editor
            | Self::Select
            | Self::NumberInput
            | Self::DatePicker
            | Self::OtpInput
            | Self::ColorPicker => ComponentCategory::Form,
            Self::Dialog
            | Self::Sidebar
            | Self::DescriptionList
            | Self::GroupBox
            | Self::Notification
            | Self::Popover
            | Self::Resizable
            | Self::Scrollable
            | Self::Sheet => ComponentCategory::Layout,
            Self::Table | Self::List => ComponentCategory::Data,
            Self::Chart
            | Self::Calendar
            | Self::Menu
            | Self::DataTable
            | Self::Tabs
            | Self::Tree
            | Self::VirtualList
            | Self::Plot => ComponentCategory::Advanced,
        }
    }

    pub fn from_label(label: &str) -> Option<Self> {
        match label.trim().to_ascii_lowercase().as_str() {
            "button" => Some(Self::Button),
            "badge" => Some(Self::Badge),
            "checkbox" => Some(Self::Checkbox),
            "accordion" => Some(Self::Accordion),
            "alert" => Some(Self::Alert),
            "alertdialog" | "alert-dialog" => Some(Self::AlertDialog),
            "avatar" => Some(Self::Avatar),
            "clipboard" => Some(Self::Clipboard),
            "collapsible" => Some(Self::CollapsibleWidget),
            "dropdownbutton" | "dropdown-button" => Some(Self::DropdownButton),
            "hovercard" | "hover-card" => Some(Self::HoverCard),
            "icon" => Some(Self::Icon),
            "image" => Some(Self::Image),
            "kbd" => Some(Self::Kbd),
            "label" => Some(Self::Label),
            "pagination" => Some(Self::Pagination),
            "progress" => Some(Self::Progress),
            "radio" => Some(Self::Radio),
            "rating" => Some(Self::Rating),
            "skeleton" => Some(Self::Skeleton),
            "slider" => Some(Self::Slider),
            "spinner" => Some(Self::Spinner),
            "stepper" => Some(Self::Stepper),
            "switch" => Some(Self::Switch),
            "tag" => Some(Self::Tag),
            "toggle" => Some(Self::Toggle),
            "tooltip" => Some(Self::Tooltip),
            "input" => Some(Self::Input),
            "editor" => Some(Self::Editor),
            "select" => Some(Self::Select),
            "numberinput" | "number-input" => Some(Self::NumberInput),
            "datepicker" | "date-picker" => Some(Self::DatePicker),
            "otpinput" | "otp-input" => Some(Self::OtpInput),
            "colorpicker" | "color-picker" => Some(Self::ColorPicker),
            "dialog" => Some(Self::Dialog),
            "sidebar" => Some(Self::Sidebar),
            "descriptionlist" | "description-list" => Some(Self::DescriptionList),
            "groupbox" | "group-box" => Some(Self::GroupBox),
            "notification" => Some(Self::Notification),
            "popover" => Some(Self::Popover),
            "resizable" => Some(Self::Resizable),
            "scrollable" => Some(Self::Scrollable),
            "sheet" => Some(Self::Sheet),
            "table" => Some(Self::Table),
            "list" => Some(Self::List),
            "chart" => Some(Self::Chart),
            "calendar" => Some(Self::Calendar),
            "menu" => Some(Self::Menu),
            "datatable" | "data-table" => Some(Self::DataTable),
            "tabs" => Some(Self::Tabs),
            "tree" => Some(Self::Tree),
            "virtuallist" | "virtual-list" => Some(Self::VirtualList),
            "plot" => Some(Self::Plot),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PlaygroundPreviewKind {
    #[default]
    Button,
    Badge,
    Status,
}

impl PlaygroundPreviewKind {
    pub const ALL: [Self; 3] = [Self::Button, Self::Badge, Self::Status];

    pub fn title(self) -> &'static str {
        match self {
            Self::Button => "Button",
            Self::Badge => "Badge",
            Self::Status => "Status",
        }
    }

    pub fn from_index(index: usize) -> Option<Self> {
        Self::ALL.get(index).copied()
    }

    pub fn from_label(label: &str) -> Option<Self> {
        match label.trim().to_ascii_lowercase().as_str() {
            "button" | "preview-button" => Some(Self::Button),
            "badge" | "preview-badge" => Some(Self::Badge),
            "status" | "preview-status" => Some(Self::Status),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlaygroundAction {
    ToggleSelected,
    ToggleLoading,
    ToggleCompact,
    IncreaseIntensity,
    DecreaseIntensity,
    SetPreviewKind(PlaygroundPreviewKind),
}

impl PlaygroundAction {
    pub fn title(self) -> &'static str {
        match self {
            Self::ToggleSelected => "selected 토글",
            Self::ToggleLoading => "loading 토글",
            Self::ToggleCompact => "compact 토글",
            Self::IncreaseIntensity => "intensity 증가",
            Self::DecreaseIntensity => "intensity 감소",
            Self::SetPreviewKind(kind) => match kind {
                PlaygroundPreviewKind::Button => "preview를 Button으로 변경",
                PlaygroundPreviewKind::Badge => "preview를 Badge로 변경",
                PlaygroundPreviewKind::Status => "preview를 Status로 변경",
            },
        }
    }

    pub fn from_label(label: &str) -> Option<Self> {
        match label.trim().to_ascii_lowercase().as_str() {
            "toggle-selected" => Some(Self::ToggleSelected),
            "toggle-loading" => Some(Self::ToggleLoading),
            "toggle-compact" => Some(Self::ToggleCompact),
            "increase-intensity" => Some(Self::IncreaseIntensity),
            "decrease-intensity" => Some(Self::DecreaseIntensity),
            preview_label => {
                PlaygroundPreviewKind::from_label(preview_label).map(Self::SetPreviewKind)
            }
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PlaygroundSnapshot {
    pub preview_kind: PlaygroundPreviewKind,
    pub selected: bool,
    pub loading: bool,
    pub compact: bool,
    pub intensity: u8,
}

impl PlaygroundSnapshot {
    pub fn summary(&self) -> String {
        format!(
            "preview={} | selected={} | loading={} | compact={} | intensity={}",
            self.preview_kind.title(),
            self.selected,
            self.loading,
            self.compact,
            self.intensity
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlaygroundState {
    pub previous: PlaygroundSnapshot,
    pub current: PlaygroundSnapshot,
    pub user_events: Vec<String>,
    pub debug_events: Vec<String>,
}

impl Default for PlaygroundState {
    fn default() -> Self {
        Self::new()
    }
}

impl PlaygroundState {
    pub fn new() -> Self {
        let snapshot = PlaygroundSnapshot {
            preview_kind: PlaygroundPreviewKind::Button,
            selected: false,
            loading: false,
            compact: false,
            intensity: 3,
        };

        Self {
            previous: snapshot.clone(),
            current: snapshot.clone(),
            user_events: vec!["Playground ready".to_string()],
            debug_events: vec![format!("Initialized snapshot: {}", snapshot.summary())],
        }
    }

    pub fn apply_action(&mut self, action: PlaygroundAction) {
        self.previous = self.current.clone();

        match action {
            PlaygroundAction::ToggleSelected => {
                self.current.selected = !self.current.selected;
            }
            PlaygroundAction::ToggleLoading => {
                self.current.loading = !self.current.loading;
            }
            PlaygroundAction::ToggleCompact => {
                self.current.compact = !self.current.compact;
            }
            PlaygroundAction::IncreaseIntensity => {
                self.current.intensity = (self.current.intensity + 1).min(9);
            }
            PlaygroundAction::DecreaseIntensity => {
                self.current.intensity = self.current.intensity.saturating_sub(1).max(1);
            }
            PlaygroundAction::SetPreviewKind(kind) => {
                self.current.preview_kind = kind;
            }
        }

        push_limited(
            &mut self.user_events,
            format!("사용자 액션: {}", action.title()),
        );
        push_limited(
            &mut self.debug_events,
            format!(
                "{} | before: {} | after: {}",
                action.title(),
                self.previous.summary(),
                self.current.summary()
            ),
        );
    }
}

fn push_limited(entries: &mut Vec<String>, value: String) {
    entries.push(value);
    if entries.len() > PLAYGROUND_LOG_LIMIT {
        let overflow = entries.len() - PLAYGROUND_LOG_LIMIT;
        entries.drain(0..overflow);
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LayoutDensity {
    #[default]
    Comfortable,
    Compact,
}

impl LayoutDensity {
    pub const ALL: [Self; 2] = [Self::Comfortable, Self::Compact];

    pub fn title(self) -> &'static str {
        match self {
            Self::Comfortable => "Comfortable",
            Self::Compact => "Compact",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::Comfortable => "넉넉한 간격과 설명 영역을 유지하는 기본 레이아웃 밀도입니다.",
            Self::Compact => "밀도 높은 패널 배치와 짧은 여백으로 빠른 탐색을 우선합니다.",
        }
    }

    pub fn from_index(index: usize) -> Option<Self> {
        Self::ALL.get(index).copied()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PerformanceMode {
    #[default]
    Balanced,
    Responsive,
}

impl PerformanceMode {
    pub const ALL: [Self; 2] = [Self::Balanced, Self::Responsive];

    pub fn title(self) -> &'static str {
        match self {
            Self::Balanced => "Balanced",
            Self::Responsive => "Responsive",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::Balanced => "표시 정보와 렌더링 비용 사이의 균형을 유지합니다.",
            Self::Responsive => "대용량 데이터와 빠른 상호작용을 위해 가벼운 렌더링을 우선합니다.",
        }
    }

    pub fn from_index(index: usize) -> Option<Self> {
        Self::ALL.get(index).copied()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SettingsState {
    pub log_level: AppLogLevel,
    pub layout_density: LayoutDensity,
    pub performance_mode: PerformanceMode,
    pub log_directory: String,
    pub rolling_strategy: String,
}

impl Default for SettingsState {
    fn default() -> Self {
        Self {
            log_level: AppLogLevel::Info,
            layout_density: LayoutDensity::Comfortable,
            performance_mode: PerformanceMode::Balanced,
            log_directory: "logs".to_string(),
            rolling_strategy: "daily app.log rotation".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UiLogEntry {
    pub sequence: usize,
    pub level: AppLogLevel,
    pub scope: &'static str,
    pub message: String,
}

fn push_limited_ui_logs(entries: &mut Vec<UiLogEntry>, value: UiLogEntry) {
    entries.push(value);
    if entries.len() > UI_LOG_LIMIT {
        let overflow = entries.len() - UI_LOG_LIMIT;
        entries.drain(0..overflow);
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AppState {
    pub current_route: AppRoute,
    pub selected_component_category: Option<ComponentCategory>,
    pub selected_component: Option<ComponentId>,
    pub playground: PlaygroundState,
    pub settings: SettingsState,
    pub ui_logs: Vec<UiLogEntry>,
    pub status_message: String,
    pub sidebar_collapsed: bool,
    next_log_sequence: usize,
}

impl AppState {
    pub fn new() -> Self {
        let mut state = Self {
            current_route: AppRoute::Dashboard,
            selected_component_category: None,
            selected_component: None,
            playground: PlaygroundState::new(),
            settings: SettingsState::default(),
            ui_logs: Vec::new(),
            status_message: String::new(),
            sidebar_collapsed: false,
            next_log_sequence: 1,
        };

        state.status_message = state.compose_status_message();
        state.record_ui_log(AppLogLevel::Info, "app.state", "App state initialized");
        state
    }

    pub fn register_logging_runtime(&mut self, runtime: &LoggerRuntime) {
        self.settings.log_directory = runtime.log_directory.clone();
        self.settings.rolling_strategy = runtime.rolling_strategy.clone();
        self.record_ui_log(
            AppLogLevel::Info,
            "app.logging",
            format!(
                "File logging ready in {} ({})",
                self.settings.log_directory, self.settings.rolling_strategy
            ),
        );
        self.status_message = self.compose_status_message();
    }

    pub fn toggle_sidebar(&mut self) {
        self.sidebar_collapsed = !self.sidebar_collapsed;
    }

    pub fn navigate_to(&mut self, route: AppRoute) {
        self.current_route = route;
        if let Some((category, component)) = route.default_component_selection() {
            self.selected_component_category = Some(category);
            self.selected_component = Some(component);
        } else {
            self.selected_component_category = None;
            self.selected_component = None;
        }
        self.record_ui_log(
            AppLogLevel::Info,
            "app.route",
            format!("Route changed to {}", route.title()),
        );
        self.status_message = self.compose_status_message();
    }

    pub fn select_component_category(&mut self, category: ComponentCategory) {
        self.selected_component_category = Some(category);
        self.selected_component = Some(category.default_component());
        self.record_ui_log(
            AppLogLevel::Info,
            "app.components",
            format!("Category changed to {}", category.title()),
        );
        self.status_message = self.compose_status_message();
    }

    pub fn select_component(&mut self, component: ComponentId) {
        self.selected_component_category = Some(component.category());
        self.selected_component = Some(component);
        self.record_ui_log(
            AppLogLevel::Info,
            "app.components",
            format!("Component changed to {}", component.title()),
        );
        self.status_message = self.compose_status_message();
    }

    pub fn apply_playground_action(&mut self, action: PlaygroundAction) {
        self.playground.apply_action(action);
        self.record_ui_log(
            AppLogLevel::Info,
            "app.playground",
            format!("Playground action: {}", action.title()),
        );
        self.status_message = self.compose_status_message();
    }

    pub fn set_log_level(&mut self, level: AppLogLevel) {
        self.settings.log_level = level;
        self.record_ui_log(
            AppLogLevel::Info,
            "app.settings",
            format!("Log level changed to {}", level.title()),
        );
        self.status_message = self.compose_status_message();
    }

    pub fn set_layout_density(&mut self, density: LayoutDensity) {
        self.settings.layout_density = density;
        self.record_ui_log(
            AppLogLevel::Info,
            "app.settings",
            format!("Layout density changed to {}", density.title()),
        );
        self.status_message = self.compose_status_message();
    }

    pub fn set_performance_mode(&mut self, mode: PerformanceMode) {
        self.settings.performance_mode = mode;
        self.record_ui_log(
            AppLogLevel::Info,
            "app.settings",
            format!("Performance mode changed to {}", mode.title()),
        );
        self.status_message = self.compose_status_message();
    }

    pub fn route_title(&self) -> &'static str {
        self.current_route.title()
    }

    pub fn route_description(&self) -> &'static str {
        self.current_route.description()
    }

    fn compose_status_message(&self) -> String {
        let component_message = self
            .selected_component
            .map(|component| format!("선택된 컴포넌트: {}", component.title()))
            .unwrap_or_else(|| "선택된 컴포넌트 없음".to_string());

        let category_message = self
            .selected_component_category
            .map(|category| format!("카테고리: {}", category.title()))
            .unwrap_or_else(|| "카테고리 없음".to_string());

        let route_message = match self.current_route {
            AppRoute::Components => format!("{} | {}", category_message, component_message),
            AppRoute::Playground => self.playground.current.summary(),
            AppRoute::Data => {
                "shared dataset: 8 rows | file nodes: 8 | capacity hint: 2048".to_string()
            }
            AppRoute::Settings => format!(
                "log level: {} | density: {} | performance: {}",
                self.settings.log_level.title(),
                self.settings.layout_density.title(),
                self.settings.performance_mode.title()
            ),
            AppRoute::Logs => format!(
                "ui logs: {} | log dir: {}",
                self.ui_logs.len(),
                self.settings.log_directory
            ),
            _ => component_message,
        };

        format!(
            "현재 화면: {} | {}",
            self.current_route.title(),
            route_message
        )
    }

    fn record_ui_log(
        &mut self,
        level: AppLogLevel,
        scope: &'static str,
        message: impl Into<String>,
    ) {
        let entry = UiLogEntry {
            sequence: self.next_log_sequence,
            level,
            scope,
            message: message.into(),
        };
        self.next_log_sequence += 1;
        push_limited_ui_logs(&mut self.ui_logs, entry);
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AppRoute, AppState, ComponentCategory, ComponentId, LayoutDensity, PerformanceMode,
        PlaygroundAction, PlaygroundPreviewKind,
    };
    use crate::shared::logger::AppLogLevel;

    #[test]
    fn navigation_updates_route_and_component_selection() {
        let mut state = AppState::new();

        state.navigate_to(AppRoute::Components);

        assert_eq!(state.current_route, AppRoute::Components);
        assert_eq!(
            state.selected_component_category,
            Some(ComponentCategory::Basic)
        );
        assert_eq!(state.selected_component, Some(ComponentId::Button));
        assert!(state.status_message.contains("Components"));
    }

    #[test]
    fn component_category_selection_updates_default_component() {
        let mut state = AppState::new();

        state.navigate_to(AppRoute::Components);
        state.select_component_category(ComponentCategory::Data);

        assert_eq!(
            state.selected_component_category,
            Some(ComponentCategory::Data)
        );
        assert_eq!(state.selected_component, Some(ComponentId::Table));
        assert!(state.status_message.contains("카테고리: Data"));
    }

    #[test]
    fn component_selection_updates_category() {
        let mut state = AppState::new();

        state.select_component(ComponentId::Chart);

        assert_eq!(
            state.selected_component_category,
            Some(ComponentCategory::Advanced)
        );
        assert_eq!(state.selected_component, Some(ComponentId::Chart));
        assert!(state.status_message.contains("Chart"));
    }

    #[test]
    fn playground_action_updates_snapshot_and_logs() {
        let mut state = AppState::new();

        state.navigate_to(AppRoute::Playground);
        state.apply_playground_action(PlaygroundAction::SetPreviewKind(
            PlaygroundPreviewKind::Badge,
        ));
        state.apply_playground_action(PlaygroundAction::IncreaseIntensity);

        assert_eq!(
            state.playground.current.preview_kind,
            PlaygroundPreviewKind::Badge
        );
        assert_eq!(state.playground.current.intensity, 4);
        assert!(
            state
                .playground
                .debug_events
                .last()
                .unwrap()
                .contains("after:")
        );
        assert!(state.status_message.contains("preview=Badge"));
    }

    #[test]
    fn data_route_updates_status_message() {
        let mut state = AppState::new();

        state.navigate_to(AppRoute::Data);

        assert!(state.status_message.contains("shared dataset"));
        assert_eq!(state.route_title(), "Data Showcase");
    }

    #[test]
    fn settings_updates_are_reflected_in_status_and_ui_logs() {
        let mut state = AppState::new();

        state.navigate_to(AppRoute::Settings);
        state.set_log_level(AppLogLevel::Debug);
        state.set_layout_density(LayoutDensity::Compact);
        state.set_performance_mode(PerformanceMode::Responsive);

        assert!(state.status_message.contains("Debug"));
        assert!(state.status_message.contains("Compact"));
        assert!(state.status_message.contains("Responsive"));
        assert!(
            state
                .ui_logs
                .iter()
                .any(|entry| entry.message.contains("Log level changed"))
        );
    }

    #[test]
    fn logs_route_reports_current_log_context() {
        let mut state = AppState::new();

        state.navigate_to(AppRoute::Logs);

        assert!(state.status_message.contains("ui logs"));
        assert!(!state.ui_logs.is_empty());
    }

    #[test]
    fn route_and_component_parse_from_labels() {
        assert_eq!(
            AppRoute::from_label("components"),
            Some(AppRoute::Components)
        );
        assert_eq!(AppRoute::from_label("data-showcase"), Some(AppRoute::Data));
        assert_eq!(
            ComponentCategory::from_label("advanced"),
            Some(ComponentCategory::Advanced)
        );
        assert_eq!(ComponentId::from_label("editor"), Some(ComponentId::Editor));
        assert_eq!(
            PlaygroundAction::from_label("preview-status"),
            Some(PlaygroundAction::SetPreviewKind(
                PlaygroundPreviewKind::Status
            ))
        );
    }
}
