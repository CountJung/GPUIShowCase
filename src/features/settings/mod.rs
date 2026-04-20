use gpui::*;
use gpui_component::{
    Selectable as _, StyledExt,
    badge::Badge,
    button::{Button, ButtonGroup},
};

use crate::shared::{
    logger::AppLogLevel,
    state::{AppState, LayoutDensity, PerformanceMode, UiLogEntry},
};

pub fn render_settings(
    state: &AppState,
    on_log_level_select: impl Fn(&Vec<usize>, &mut Window, &mut App) + 'static,
    on_layout_density_select: impl Fn(&Vec<usize>, &mut Window, &mut App) + 'static,
    on_performance_mode_select: impl Fn(&Vec<usize>, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    div()
        .v_flex()
        .gap_3()
        .p_3()
        .child(
            div()
                .h_flex()
                .gap_3()
                .items_start()
                .child(render_logging_panel(state, on_log_level_select))
                .child(render_layout_panel(state, on_layout_density_select))
                .child(render_performance_panel(state, on_performance_mode_select)),
        )
}

pub fn render_logs_page(state: &AppState) -> impl IntoElement {
    let mut panel = settings_panel("In-App Logs")
		.child(
			Badge::new().count(state.ui_logs.len()).child(
				div()
					.px_3()
					.py_2()
					.border_1()
					.rounded(px(10.0))
					.child("UI log entries"),
			),
		)
		.child(format!(
			"로그 디렉터리: {} | 롤링 규칙: {} | 현재 레벨: {}",
			state.settings.log_directory,
			state.settings.rolling_strategy,
			state.settings.log_level.title()
		))
		.child("UI 로그는 파일 로그와 같은 사용자 상호작용 계열 이벤트를 앱 내부에서도 확인할 수 있도록 유지합니다.");

    for entry in state.ui_logs.iter().rev() {
        panel = panel.child(render_log_entry(entry));
    }

    panel
}

fn render_logging_panel(
    state: &AppState,
    on_log_level_select: impl Fn(&Vec<usize>, &mut Window, &mut App) + 'static,
) -> Div {
    settings_panel("Logging Settings")
        .flex_1()
        .child(
            ButtonGroup::new("settings-log-level-group")
                .layout(Axis::Vertical)
                .outline()
                .compact()
                .child(log_level_button(
                    AppLogLevel::Error,
                    state.settings.log_level,
                ))
                .child(log_level_button(
                    AppLogLevel::Warn,
                    state.settings.log_level,
                ))
                .child(log_level_button(
                    AppLogLevel::Info,
                    state.settings.log_level,
                ))
                .child(log_level_button(
                    AppLogLevel::Debug,
                    state.settings.log_level,
                ))
                .on_click(on_log_level_select),
        )
        .child(format!("설명: {}", state.settings.log_level.description()))
        .child(format!("로그 디렉터리: {}", state.settings.log_directory))
        .child(format!("롤링 규칙: {}", state.settings.rolling_strategy))
}

fn render_layout_panel(
    state: &AppState,
    on_layout_density_select: impl Fn(&Vec<usize>, &mut Window, &mut App) + 'static,
) -> Div {
    settings_panel("Layout Density")
        .flex_1()
        .child(
            ButtonGroup::new("settings-layout-density-group")
                .layout(Axis::Vertical)
                .outline()
                .compact()
                .child(layout_density_button(
                    LayoutDensity::Comfortable,
                    state.settings.layout_density,
                ))
                .child(layout_density_button(
                    LayoutDensity::Compact,
                    state.settings.layout_density,
                ))
                .on_click(on_layout_density_select),
        )
        .child(state.settings.layout_density.description())
}

fn render_performance_panel(
    state: &AppState,
    on_performance_mode_select: impl Fn(&Vec<usize>, &mut Window, &mut App) + 'static,
) -> Div {
    settings_panel("Performance Mode")
        .flex_1()
        .child(
            ButtonGroup::new("settings-performance-mode-group")
                .layout(Axis::Vertical)
                .outline()
                .compact()
                .child(performance_mode_button(
                    PerformanceMode::Balanced,
                    state.settings.performance_mode,
                ))
                .child(performance_mode_button(
                    PerformanceMode::Responsive,
                    state.settings.performance_mode,
                ))
                .on_click(on_performance_mode_select),
        )
        .child(state.settings.performance_mode.description())
}

fn render_log_entry(entry: &UiLogEntry) -> Div {
    div()
        .v_flex()
        .gap_1()
        .p_2()
        .border_1()
        .rounded(px(10.0))
        .child(div().font_weight(FontWeight::BOLD).child(format!(
            "#{} [{}] {}",
            entry.sequence,
            entry.level.title(),
            entry.scope
        )))
        .child(entry.message.clone())
}

fn settings_panel(title: &'static str) -> Div {
    div()
        .v_flex()
        .gap_2()
        .p_3()
        .border_1()
        .rounded(px(12.0))
        .child(div().font_weight(FontWeight::BOLD).child(title))
}

fn log_level_button(level: AppLogLevel, current_level: AppLogLevel) -> Button {
    Button::new(match level {
        AppLogLevel::Error => "settings-log-level-error",
        AppLogLevel::Warn => "settings-log-level-warn",
        AppLogLevel::Info => "settings-log-level-info",
        AppLogLevel::Debug => "settings-log-level-debug",
    })
    .label(level.title())
    .outline()
    .selected(level == current_level)
}

fn layout_density_button(density: LayoutDensity, current_density: LayoutDensity) -> Button {
    Button::new(match density {
        LayoutDensity::Comfortable => "settings-layout-comfortable",
        LayoutDensity::Compact => "settings-layout-compact",
    })
    .label(density.title())
    .outline()
    .selected(density == current_density)
}

fn performance_mode_button(mode: PerformanceMode, current_mode: PerformanceMode) -> Button {
    Button::new(match mode {
        PerformanceMode::Balanced => "settings-performance-balanced",
        PerformanceMode::Responsive => "settings-performance-responsive",
    })
    .label(mode.title())
    .outline()
    .selected(mode == current_mode)
}
