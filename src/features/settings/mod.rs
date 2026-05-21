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
    on_theme_select: impl Fn(&SharedString, &mut Window, &mut App) + 'static,
    available_themes: Vec<(SharedString, bool)>,
) -> impl IntoElement {
    let palette = SettingsPalette::from_state(state);

    div()
        .size_full()
        .v_flex()
        .gap_4()
        .p_4()
        .bg(palette.background)
        .text_color(palette.foreground)
        .child(
            div()
                .v_flex()
                .gap_1()
                .child(
                    div()
                        .text_size(rems(1.35))
                        .font_weight(FontWeight::BOLD)
                        .child("Settings"),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(palette.muted)
                        .child("기본값은 GitHub Dark이며, 아래 프리셋에서 Light/Dark 모드를 포함한 모든 JSON 테마를 전환합니다."),
                ),
        )
        .child(render_theme_selector(
            state,
            available_themes,
            on_theme_select,
            palette,
        ))
        .child(
            div()
                .h_flex()
                .gap_3()
                .items_start()
                .child(render_logging_panel(state, on_log_level_select, palette))
                .child(render_layout_panel(state, on_layout_density_select, palette))
                .child(render_performance_panel(
                    state,
                    on_performance_mode_select,
                    palette,
                )),
        )
}

pub fn render_logs_page(state: &AppState) -> impl IntoElement {
    let palette = SettingsPalette::from_state(state);
    let mut panel = settings_panel("In-App Logs", palette)
        .child(
            Badge::new().count(state.ui_logs.len()).child(
                div()
                    .px_3()
                    .py_2()
                    .border_1()
                    .border_color(palette.border)
                    .rounded(px(8.0))
                    .child("UI log entries"),
            ),
        )
        .child(format!(
            "로그 디렉터리: {} | 롤링 규칙: {} | 현재 레벨: {}",
            state.settings.log_directory,
            state.settings.rolling_strategy,
            state.settings.log_level.title()
        ));

    for entry in state.ui_logs.iter().rev() {
        panel = panel.child(render_log_entry(entry, palette));
    }

    div()
        .size_full()
        .p_4()
        .bg(palette.background)
        .text_color(palette.foreground)
        .child(panel)
}

fn render_theme_selector(
    state: &AppState,
    available_themes: Vec<(SharedString, bool)>,
    on_theme_select: impl Fn(&SharedString, &mut Window, &mut App) + 'static,
    palette: SettingsPalette,
) -> Div {
    let selected_name = state.settings.selected_theme_name.clone();
    let on_select = std::rc::Rc::new(on_theme_select);
    let mut grid = div().flex().flex_wrap().gap_2();

    if available_themes.is_empty() {
        grid = grid.child(
            div()
                .text_sm()
                .text_color(palette.muted)
                .child("테마 목록을 불러오는 중입니다."),
        );
    } else {
        for (name, is_dark) in &available_themes {
            let is_selected = *name == selected_name;
            let name_clone = name.clone();
            let on_select_clone = on_select.clone();
            let mode_label = if *is_dark { "Dark" } else { "Light" };

            grid = grid.child(
                div()
                    .w(px(176.0))
                    .v_flex()
                    .gap_2()
                    .p_3()
                    .rounded(px(8.0))
                    .border_1()
                    .border_color(if is_selected {
                        palette.accent
                    } else {
                        palette.border
                    })
                    .bg(if is_selected {
                        palette.selected_background
                    } else {
                        palette.panel_background
                    })
                    .cursor_pointer()
                    .hover({
                        let hover = palette.hover_background;
                        move |mut style| {
                            style.background = Some(hover.into());
                            style
                        }
                    })
                    .on_mouse_up(MouseButton::Left, move |_: &MouseUpEvent, window, cx| {
                        (on_select_clone)(&name_clone, window, cx);
                    })
                    .child(div().font_weight(FontWeight::BOLD).child(name.clone()))
                    .child(
                        div()
                            .h_flex()
                            .gap_2()
                            .items_center()
                            .child(color_swatch(if *is_dark {
                                rgba(0x0d1117ff)
                            } else {
                                rgba(0xffffffff)
                            }))
                            .child(color_swatch(if *is_dark {
                                rgba(0x3fb950ff)
                            } else {
                                rgba(0x0969daff)
                            }))
                            .child(div().text_xs().text_color(palette.muted).child(mode_label)),
                    ),
            );
        }
    }

    settings_panel("Theme Presets", palette)
        .child(div().text_sm().text_color(palette.muted).child(format!(
            "현재 테마: {} | 사용 가능 프리셋: {}",
            selected_name,
            available_themes.len()
        )))
        .child(grid)
}

fn render_logging_panel(
    state: &AppState,
    on_log_level_select: impl Fn(&Vec<usize>, &mut Window, &mut App) + 'static,
    palette: SettingsPalette,
) -> Div {
    settings_panel("Logging", palette)
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
        .child(
            div()
                .text_sm()
                .text_color(palette.muted)
                .child(state.settings.log_level.description()),
        )
        .child(div().text_sm().text_color(palette.muted).child(format!(
            "{} ({})",
            state.settings.log_directory, state.settings.rolling_strategy
        )))
}

fn render_layout_panel(
    state: &AppState,
    on_layout_density_select: impl Fn(&Vec<usize>, &mut Window, &mut App) + 'static,
    palette: SettingsPalette,
) -> Div {
    settings_panel("Density", palette)
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
        .child(
            div()
                .text_sm()
                .text_color(palette.muted)
                .child(state.settings.layout_density.description()),
        )
}

fn render_performance_panel(
    state: &AppState,
    on_performance_mode_select: impl Fn(&Vec<usize>, &mut Window, &mut App) + 'static,
    palette: SettingsPalette,
) -> Div {
    settings_panel("Performance", palette)
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
        .child(
            div()
                .text_sm()
                .text_color(palette.muted)
                .child(state.settings.performance_mode.description()),
        )
}

fn render_log_entry(entry: &UiLogEntry, palette: SettingsPalette) -> Div {
    div()
        .v_flex()
        .gap_1()
        .p_2()
        .border_1()
        .border_color(palette.border)
        .rounded(px(8.0))
        .child(div().font_weight(FontWeight::BOLD).child(format!(
            "#{} [{}] {}",
            entry.sequence,
            entry.level.title(),
            entry.scope
        )))
        .child(
            div()
                .text_sm()
                .text_color(palette.muted)
                .child(entry.message.clone()),
        )
}

fn settings_panel(title: &'static str, palette: SettingsPalette) -> Div {
    div()
        .v_flex()
        .gap_3()
        .p_3()
        .border_1()
        .border_color(palette.border)
        .rounded(px(8.0))
        .bg(palette.panel_background)
        .child(
            div()
                .font_weight(FontWeight::BOLD)
                .text_color(palette.foreground)
                .child(title),
        )
}

fn color_swatch(color: Rgba) -> Div {
    div()
        .w(px(14.0))
        .h(px(14.0))
        .rounded_full()
        .border_1()
        .border_color(rgba(0x8b949e55))
        .bg(color)
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

#[derive(Clone, Copy)]
struct SettingsPalette {
    background: Rgba,
    panel_background: Rgba,
    hover_background: Rgba,
    selected_background: Rgba,
    border: Rgba,
    foreground: Rgba,
    muted: Rgba,
    accent: Rgba,
}

impl SettingsPalette {
    fn from_state(state: &AppState) -> Self {
        if state
            .settings
            .selected_theme_name
            .to_string()
            .contains("Dark")
        {
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
        } else {
            Self {
                background: rgba(0xffffffff),
                panel_background: rgba(0xf6f8faff),
                hover_background: rgba(0xeaeef2ff),
                selected_background: rgba(0xddf4ffff),
                border: rgba(0xd0d7deff),
                foreground: rgba(0x24292fff),
                muted: rgba(0x6e7781ff),
                accent: rgba(0x0969daff),
            }
        }
    }
}
