# gpui-component Reference (v0.5.1)

Repository: https://github.com/longbridge/gpui-component

## Initialization (필수)

```rust
// Application::run 콜백 최상단에서 반드시 호출
gpui_component::init(cx);
```

## Theme

```rust
use gpui_component::theme::{Theme, ThemeMode};

// 테마 전환
cx.update(|cx| {
    Theme::change(ThemeMode::Dark, cx);   // or ThemeMode::Light
});

// 뷰에서 현재 테마 색상 참조
fn render(&mut self, _w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
    div()
        .bg(cx.theme().colors().background)
        .text_color(cx.theme().colors().foreground)
}
```

## Layout — Dock

```rust
use gpui_component::dock::{Dock, DockArea, Panel, PanelView};

// 사이드바 + 메인 패널 레이아웃
div()
    .flex()
    .size_full()
    .child(
        // 사이드바
        div()
            .flex()
            .flex_col()
            .w(px(200.0))
            .bg(cx.theme().colors().sidebar_background)
            .child(nav_item("Dashboard", cx))
            .child(nav_item("Targets", cx))
            .child(nav_item("Settings", cx))
    )
    .child(
        // 메인 패널
        div()
            .flex_1()
            .overflow_hidden()
            .child(self.active_panel(cx))
    )
```

## Button

```rust
use gpui_component::button::{Button, ButtonVariant, ButtonSize};

Button::new("my-button")               // unique id (ElementId)
    .label("Activate")
    .variant(ButtonVariant::Primary)   // Primary | Secondary | Ghost | Destructive
    .size(ButtonSize::Medium)          // Small | Medium | Large
    .disabled(false)
    .on_click(cx.listener(|this, _event, cx| {
        this.handle_click(cx);
    }))

// Icon button
Button::new("icon-btn")
    .icon(IconName::Play)
    .on_click(cx.listener(|_, _, cx| { /* ... */ }))
```

## Label & Text

```rust
use gpui_component::label::Label;

Label::new("Hello, World!")
    .text_sm()        // text_xs | text_sm | text_base | text_lg | text_xl
    .text_muted()     // muted color
    .font_bold()
    .line_clamp(2)    // truncate after N lines
```

## Switch / Toggle

```rust
use gpui_component::switch::Switch;

// Controlled switch — state lives in your Model
Switch::new("active-switch")
    .checked(state.is_active)
    .on_change(cx.listener(|this, checked: &bool, cx| {
        this.model.update(cx, |m, cx| {
            m.state.is_active = *checked;
            cx.notify();
        });
    }))
```

## Text Input

```rust
use gpui_component::input::{TextInput, InputState};

// InputState must be a Model field — it holds the text value
pub struct SettingsView {
    url_input: Model<InputState>,
}

impl SettingsView {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            url_input: cx.new(|cx| InputState::new(cx).default_value("http://")),
        }
    }
}

// In render:
TextInput::new(&self.url_input)
    .placeholder("Enter URL...")
    .w_full()

// Read value:
let value = self.url_input.read(cx).value().to_string();
```

## Dropdown / Select

```rust
use gpui_component::dropdown::{Dropdown, DropdownState, DropdownItem};

// Items implement DropdownItem trait
#[derive(Clone)]
struct ProcessItem { name: String }
impl DropdownItem for ProcessItem {
    type Value = String;
    fn label(&self) -> SharedString { self.name.clone().into() }
    fn value(&self) -> &Self::Value { &self.name }
}

// State in your view model
process_dropdown: Model<DropdownState<ProcessItem>>

// Init:
let items = vec![
    ProcessItem { name: "KakaoTalk.exe".into() },
    ProcessItem { name: "slack.exe".into() },
];
let dropdown = cx.new(|cx| DropdownState::new(items, None, cx));

// Render:
Dropdown::new("process-select", &self.process_dropdown)
    .placeholder("Select process...")
    .w(px(200.0))

// Read selection:
let selected = self.process_dropdown.read(cx).selected_item();
```

## Table (Virtual — for large lists)

```rust
use gpui_component::table::{Table, TableDelegate, ColFixed, ColSort};

struct TargetTableDelegate {
    rows: Vec<TargetApp>,
    sort_col: Option<(usize, ColSort)>,
}

impl TableDelegate for TargetTableDelegate {
    fn cols_count(&self, _cx: &App) -> usize { 3 }
    fn rows_count(&self, _cx: &App) -> usize { self.rows.len() }

    fn col_name(&self, col_ix: usize, _cx: &App) -> SharedString {
        match col_ix {
            0 => "Process".into(),
            1 => "Status".into(),
            2 => "Enabled".into(),
            _ => "".into(),
        }
    }

    fn col_width(&self, col_ix: usize, _cx: &App) -> Pixels {
        match col_ix { 0 => px(200.), 1 => px(100.), _ => px(80.) }
    }

    fn render_td(
        &self, row_ix: usize, col_ix: usize, _cx: &App
    ) -> impl IntoElement {
        let row = &self.rows[row_ix];
        match col_ix {
            0 => div().child(row.process_name.clone()),
            1 => div().child(if row.enabled { "Active" } else { "Idle" }),
            _ => div(),
        }
    }
}

// In view:
Table::new("target-table", &self.table_state)
    .size_full()
    .striped(true)
```

## Toast Notifications

```rust
use gpui_component::notification::{Notification, NotificationList};

// Push a toast (call on cx from any view)
cx.push_notification(
    Notification::new("Ad blocked successfully!")
        .icon(IconName::ShieldCheck)
        .autohide(true)
);

// Error toast
cx.push_notification(
    Notification::new("Failed to find ad window")
        .icon(IconName::AlertCircle)
        .variant(NotificationVariant::Destructive)
);

// Root must include NotificationList somewhere
NotificationList::new()
    .absolute()
    .bottom_4()
    .right_4()
```

## Badge

```rust
use gpui_component::badge::Badge;

Badge::new()
    .label("12")                        // count
    .variant(BadgeVariant::Destructive) // Default | Secondary | Destructive
```

## Icon

```rust
use gpui_component::icon::{Icon, IconName};

Icon::new(IconName::ShieldCheck)
    .size(px(20.0))
    .text_color(cx.theme().colors().icon)

// Wrapped in a div for positioning
div()
    .flex()
    .items_center()
    .gap_2()
    .child(Icon::new(IconName::Play).size(px(16.0)))
    .child(Label::new("Start"))
```

## Common GPUI Element Methods

```rust
// Layout
.flex()          .flex_col()      .flex_row()
.flex_1()        .flex_none()     .flex_wrap()
.items_center()  .justify_between() .justify_center()
.gap_2()         .gap_4()         .gap_6()
.p_4()           .px_6()          .py_3()
.m_auto()        .mx_4()          .mt_2()
.w_full()        .h_full()        .size_full()
.w(px(200.))     .h(px(48.))      .min_w(px(100.))
.overflow_hidden() .overflow_y_scroll()

// Style
.bg(color)           .text_color(color)
.border_1()          .border_color(color)
.rounded_lg()        .rounded_full()
.shadow_md()         .opacity(0.5)
.cursor_pointer()    .cursor_default()

// Interaction
.on_click(handler)
.on_hover(handler)
.id("unique-id")     // required for interactive elements
```
