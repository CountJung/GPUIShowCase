use crate::features::log_viewer::entry::{LogLevel, LogEntry};
use crate::features::log_viewer::state::LogViewerState;
use gpui::*;
use gpui_component::StyledExt;
use gpui_component::button::Button;

/// 로그 뷰어 위젯을 렌더링합니다.
pub fn render_log_viewer(state: &mut LogViewerState) -> impl IntoElement {
    let palette = LogPalette::from_theme(true); // Dark mode
    
    div()
        .size_full()
        .v_flex()
        .gap_3()
        .p_4()
        .bg(palette.background)
        .text_color(palette.foreground)
        .child(render_toolbar(state, palette))
        .child(render_log_list(state, palette))
}

/// 툴바(날짜 선택, 레벨 필터, 검색)를 렌더링합니다.
fn render_toolbar(state: &mut LogViewerState, palette: LogPalette) -> Div {
    let dates = state.dates.clone();
    let selected_date = state.selected_date.clone();

    // 날짜 선택기 생성
    let date_picker = render_date_buttons(&dates, &selected_date);

    // 레벨 필터 버튼  
    let level_filters = render_level_buttons(state, palette);

    // 검색창
    let search_input = render_search_input(palette);

    div()
        .h_flex()
        .gap_3()
        .items_center()
        .flex_wrap()
        .child(div().text_sm().font_weight(FontWeight::BOLD).child("날짜:"))
        .child(date_picker)
        .child(level_filters)
        .child(search_input)
}

/// 날짜 선택 버튼들을 렌더링합니다.
fn render_date_buttons(dates: &[String], selected: &str) -> Div {
    if dates.is_empty() {
        return div()
            .text_sm()
            .text_color(rgba(0x8b949eff))
            .child("로그 파일이 없습니다.");
    }

    let mut container = div().h_flex().gap_1();

    for (idx, date) in dates.iter().enumerate() {
        let is_selected = *date == selected;
        
        let _is_selected = is_selected;
        container = container.child(
            Button::new(SharedString::from(format!("date-{}", idx)))
                .label(date.clone())
                .outline()
        );
    }

    container
}

/// 레벨 필터 버튼들을 렌더링합니다.
fn render_level_buttons(state: &LogViewerState, _palette: LogPalette) -> Div {
    let levels = [
        ("전체", 0),
        ("ERROR", 1),
        ("WARN", 2),
        ("INFO", 3),
        ("DEBUG", 4),
        ("TRACE", 5),
    ];

    // 현재 선택된 레벨의 인덱스 찾기
    let selected_idx = match state.level_filter {
        None => 0,
        Some(LogLevel::Error) => 1,
        Some(LogLevel::Warn) => 2,
        Some(LogLevel::Info) => 3,
        Some(LogLevel::Debug) => 4,
        Some(LogLevel::Trace) => 5,
        Some(LogLevel::Unknown) => 0,
    };

    let mut container = div().h_flex().gap_1();

    for (idx, (label, _)) in levels.iter().enumerate() {
        let is_selected = idx == selected_idx;
        
        let _is_selected = is_selected;
        container = container.child(
            Button::new(SharedString::from(format!("level-{}", idx)))
                .label(*label)
                .outline()
        );
    }

    container
}

/// 검색 입력창을 렌더링합니다.
fn render_search_input(palette: LogPalette) -> Div {
    div()
        .h_flex()
        .items_center()
        .gap_2()
        .flex_1()
        .max_w(px(300.0))
        .p_2()
        .border_1()
        .border_color(palette.border)
        .rounded(px(6.0))
        .bg(palette.panel_background)
        .child(div().text_sm().text_color(palette.muted).child("로그 검색..."))
}

/// 로그 목록을 렌더링합니다.
fn render_log_list(state: &LogViewerState, palette: LogPalette) -> Div {
    let entries = &state.filtered_entries;
    let total = state.total_count();
    let filtered = state.filtered_count();

    let container = div()
        .v_flex()
        .gap_1()
        .flex_1()
        .min_h(px(400.0))
        .border_1()
        .border_color(palette.border)
        .rounded(px(8.0))
        .p_2()
        .bg(palette.panel_background);

    if entries.is_empty() {
        container.child(
            div()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .text_sm()
                .text_color(palette.muted)
                .child("표시할 로그가 없습니다."),
        )
    } else {
        let mut result = container;
        for entry in entries {
            result = result.child(render_log_entry(entry, palette));
        }
        result.child(
            div()
                .h_flex()
                .justify_between()
                .text_xs()
                .text_color(palette.muted)
                .child(format!("총 {}개 중 {}개 표시 (전체: {})", filtered, filtered, total))
        )
    }
}

/// 개별 로그 엔트리를 렌더링합니다.
fn render_log_entry(entry: &LogEntry, palette: LogPalette) -> Div {
    let level_color = get_level_text_color(entry.level);
    let badge_bg = get_level_badge_bg(entry.level);

    div()
        .h_flex()
        .gap_2()
        .p_2()
        .rounded(px(4.0))
        .child(
            div()
                .text_xs()
                .font_family(SharedString::from("Consolas, Monaco, Courier New"))
                .text_color(palette.muted)
                .w(px(80.0))
                .flex_shrink_0()
                .child(format!("{} {}", entry.date, entry.time)),
        )
        .child(
            div()
                .px_2()
                .py_1()
                .rounded(px(4.0))
                .bg(badge_bg)
                .text_xs()
                .font_weight(FontWeight::BOLD)
                .text_color(level_color)
                .w(px(58.0))
                .flex_shrink_0()
                .flex()
                .items_center()
                .justify_center()
                .child(entry.level.name()),
        )
        .child(
            div()
                .flex_1()
                .overflow_x_hidden()
                .text_sm()
                .font_family(SharedString::from("Consolas, Monaco, Courier New"))
                .text_color(palette.foreground)
                .child(entry.raw_line.clone()),
        )
}

/// 로그 레벨별 텍스트 색상 반환 (배경과 대비되는 색상)
fn get_level_text_color(level: LogLevel) -> Rgba {
    match level {
        LogLevel::Error => rgba(0xffccccff), // 밝은 빨강
        LogLevel::Warn => rgba(0xfff5e0ff),   // 밝은 노랑
        LogLevel::Info => rgba(0xd1f4d1ff),   // 밝은 초록
        LogLevel::Debug => rgba(0xc5c8ffff),  // 연한 회색
        LogLevel::Trace => rgba(0x999999ff),  // 더 밝은 회색
        LogLevel::Unknown => rgba(0xffffff80),
    }
}

/// 로그 레벨별 배경 색상 반환 (대비 고려)
fn get_level_badge_bg(level: LogLevel) -> Rgba {
    match level {
        LogLevel::Error => rgba(0x8b1a1aff),   // 어두운 빨강
        LogLevel::Warn => rgba(0x856a00ff),     // 어두운 노랑
        LogLevel::Info => rgba(0x0f5132ff),     // 어두운 초록
        LogLevel::Debug => rgba(0x1a3a6aff),    // 어두운 파랑
        LogLevel::Trace => rgba(0x4a4a4aff),    // 회색
        LogLevel::Unknown => rgba(0x666666ff),
    }
}

/// 로그 뷰어용 팔레트.
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct LogPalette {
    background: Rgba,
    panel_background: Rgba,
    #[allow(dead_code)]
    pub hover_background: Rgba,
    selected_background: Rgba,
    border: Rgba,
    foreground: Rgba,
    muted: Rgba,
    accent: Rgba,
}

impl LogPalette {
    fn from_theme(_is_dark: bool) -> Self {
        Self {
            background: rgba(0x010409ff),
            panel_background: rgba(0x0d1117ff),
            hover_background: rgba(0x161b22ff),
            selected_background: rgba(0x102a14ff),
            border: rgba(0x30363dff),
            foreground: rgba(0xe6edf3ff),
            muted: rgba(0x8b949eff),
            accent: rgba(0x3fb950ff),
        }
    }
}

