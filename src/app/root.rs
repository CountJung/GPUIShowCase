use gpui::*;
use gpui_component::{
    StyledExt,
    scroll::ScrollableElement as _,
    theme::{Theme, ThemeMode, ThemeRegistry},
};

use crate::app::layout::{content, header, nav_panel, sidebar};
use crate::features::log_viewer::entry::LogLevel;
use crate::shared::{
    logger::{self, AppLogLevel},
    state::{
        AppRoute, AppState, ComponentCategory, ComponentId, LayoutDensity, PerformanceMode,
        PlaygroundAction, PlaygroundPreviewKind,
    },
};

pub struct AppRoot {
    state: AppState,
}

impl AppRoot {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

impl Default for AppRoot {
    fn default() -> Self {
        Self::new(AppState::new())
    }
}

impl AppRoot {
    fn apply_route(&mut self, route: AppRoute, _: &mut Window, cx: &mut Context<Self>) {
        self.state.navigate_to(route);
        logger::emit_event(
            AppLogLevel::Info,
            "app.route",
            format!("route changed to {}", route.title()),
        );
        cx.notify();
    }

    fn apply_category(
        &mut self,
        category: ComponentCategory,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.state.select_component_category(category);
        logger::emit_event(
            AppLogLevel::Info,
            "app.components",
            format!("category changed to {}", category.title()),
        );
        cx.notify();
    }

    fn apply_component(&mut self, component: ComponentId, _: &mut Window, cx: &mut Context<Self>) {
        self.state.select_component(component);
        logger::emit_event(
            AppLogLevel::Info,
            "app.components",
            format!("component selected: {}", component.title()),
        );
        cx.notify();
    }

    fn apply_playground_preview_kind_selection(
        &mut self,
        selected_indices: &[usize],
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(selected_index) = selected_indices.first().copied()
            && let Some(kind) = PlaygroundPreviewKind::from_index(selected_index)
        {
            self.state
                .apply_playground_action(PlaygroundAction::SetPreviewKind(kind));
            logger::emit_event(
                AppLogLevel::Info,
                "app.playground",
                format!("playground preview changed to {}", kind.title()),
            );
            cx.notify();
        }
    }

    fn apply_playground_action(
        &mut self,
        action: PlaygroundAction,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.state.apply_playground_action(action);
        logger::emit_event(
            AppLogLevel::Info,
            "app.playground",
            format!("playground action: {}", action.title()),
        );
        cx.notify();
    }

    fn apply_log_level_selection(
        &mut self,
        selected_indices: &[usize],
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(selected_index) = selected_indices.first().copied()
            && let Some(level) = AppLogLevel::from_index(selected_index)
        {
            logger::emit_event(
                AppLogLevel::Info,
                "app.settings",
                format!("log level will change to {}", level.title()),
            );
            logger::set_log_level(level);
            self.state.set_log_level(level);
            cx.notify();
        }
    }

    fn apply_layout_density_selection(
        &mut self,
        selected_indices: &[usize],
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(selected_index) = selected_indices.first().copied()
            && let Some(density) = LayoutDensity::from_index(selected_index)
        {
            self.state.set_layout_density(density);
            logger::emit_event(
                AppLogLevel::Info,
                "app.settings",
                format!("layout density changed to {}", density.title()),
            );
            cx.notify();
        }
    }

    fn apply_performance_mode_selection(
        &mut self,
        selected_indices: &[usize],
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(selected_index) = selected_indices.first().copied()
            && let Some(mode) = PerformanceMode::from_index(selected_index)
        {
            self.state.set_performance_mode(mode);
            logger::emit_event(
                AppLogLevel::Info,
                "app.settings",
                format!("performance mode changed to {}", mode.title()),
            );
            cx.notify();
        }
    }

    fn apply_theme(
        &mut self,
        theme_name: &SharedString,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(theme_config) = ThemeRegistry::global(cx).themes().get(theme_name).cloned() {
            let mode = theme_config.mode;
            {
                let theme = Theme::global_mut(cx);
                if mode == ThemeMode::Dark {
                    theme.dark_theme = theme_config;
                } else {
                    theme.light_theme = theme_config;
                }
            }
            Theme::change(mode, Some(window), cx);
            self.state.set_selected_theme(theme_name.clone());
            logger::emit_event(
                AppLogLevel::Info,
                "app.theme",
                format!("theme changed to {}", theme_name),
            );
            cx.notify();
        }
    }

    fn apply_log_viewer_date_select(
        &mut self,
        selected_indices: &[usize],
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(idx) = selected_indices.first().copied() {
            let dates = self.state.log_viewer.dates.clone();
            if idx < dates.len() {
                self.state.log_viewer.select_date(dates[idx].clone());
                logger::emit_event(
                    AppLogLevel::Info,
                    "app.logs",
                    format!("log date changed to {}", self.state.log_viewer.selected_date),
                );
                cx.notify();
            }
        }
    }

    fn apply_log_viewer_level_filter(
        &mut self,
        selected_indices: &[usize],
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(idx) = selected_indices.first().copied() {
            let level = match idx {
                0 => None,
                1 => Some(LogLevel::Error),
                2 => Some(LogLevel::Warn),
                3 => Some(LogLevel::Info),
                4 => Some(LogLevel::Debug),
                5 => Some(LogLevel::Trace),
                _ => None,
            };
            self.state.log_viewer.set_level_filter(level);
            logger::emit_event(
                AppLogLevel::Info,
                "app.logs",
                format!("log level filter changed"),
            );
            cx.notify();
        }
    }

    fn get_available_themes(cx: &Context<Self>) -> Vec<(SharedString, bool)> {
        ThemeRegistry::global(cx)
            .sorted_themes()
            .iter()
            .map(|t| (t.name.clone(), t.mode == ThemeMode::Dark))
            .collect()
    }
}

impl Render for AppRoot {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let available_themes = Self::get_available_themes(cx);

        // ── Column 1: narrow icon strip (48 px) ──────────────────────────────
        let icon_strip = sidebar::render_icon_strip(
            &self.state,
            cx.listener(|this, route: &AppRoute, w, cx| {
                this.apply_route(*route, w, cx);
            }),
        );

        // ── Column 2: context nav panel (220 px default, resizable) ──────────
        let nav = nav_panel::render_nav_panel(
            &self.state,
            cx.listener(|this, cat: &ComponentCategory, w, cx| {
                if this.state.current_route != AppRoute::Components {
                    this.apply_route(AppRoute::Components, w, cx);
                }
                this.apply_category(*cat, w, cx);
            }),
            cx.listener(|this, id: &ComponentId, w, cx| {
                if this.state.current_route != AppRoute::Components {
                    this.apply_route(AppRoute::Components, w, cx);
                }
                this.apply_component(*id, w, cx);
            }),
        );

        // ── Column 3: main content ────────────────────────────────────────────
        let header_state = self.state.clone();
        
        let main_content = div()
            .size_full()
            .v_flex()
            .child(header::render_header(&header_state))
            .child(div().flex_1().min_h(px(0.0)).overflow_y_scrollbar().child({
                content::render_content(
                    &self.state,
                    content::PlaygroundContentActions {
                        on_preview_kind_select: cx.listener(
                            |this, selected_indices: &Vec<usize>, window, cx| {
                                this.apply_playground_preview_kind_selection(
                                    selected_indices.as_slice(),
                                    window,
                                    cx,
                                );
                            },
                        ),
                        on_toggle_selected: cx.listener(|this, _: &ClickEvent, window, cx| {
                            this.apply_playground_action(PlaygroundAction::ToggleSelected, window, cx);
                        }),
                        on_toggle_loading: cx.listener(|this, _: &ClickEvent, window, cx| {
                            this.apply_playground_action(PlaygroundAction::ToggleLoading, window, cx);
                        }),
                        on_toggle_compact: cx.listener(|this, _: &ClickEvent, window, cx| {
                            this.apply_playground_action(PlaygroundAction::ToggleCompact, window, cx);
                        }),
                        on_increase_intensity: cx.listener(|this, _: &ClickEvent, window, cx| {
                            this.apply_playground_action(
                                PlaygroundAction::IncreaseIntensity,
                                window,
                                cx,
                            );
                        }),
                        on_decrease_intensity: cx.listener(|this, _: &ClickEvent, window, cx| {
                            this.apply_playground_action(
                                PlaygroundAction::DecreaseIntensity,
                                window,
                                cx,
                            );
                        }),
                    },
                    content::SettingsContentActions {
                        on_log_level_select: cx.listener(
                            |this, selected_indices: &Vec<usize>, window, cx| {
                                this.apply_log_level_selection(selected_indices.as_slice(), window, cx);
                            },
                        ),
                        on_layout_density_select: cx.listener(
                            |this, selected_indices: &Vec<usize>, window, cx| {
                                this.apply_layout_density_selection(
                                    selected_indices.as_slice(),
                                    window,
                                    cx,
                                );
                            },
                        ),
                        on_performance_mode_select: cx.listener(
                            |this, selected_indices: &Vec<usize>, window, cx| {
                                this.apply_performance_mode_selection(
                                    selected_indices.as_slice(),
                                    window,
                                    cx,
                                );
                            },
                        ),
                        on_theme_select: cx.listener(|this, theme_name: &SharedString, window, cx| {
                            this.apply_theme(theme_name, window, cx);
                        }),
                    },
                    content::LogViewerContentActions {
                        on_date_select: cx.listener(
                            |this, selected_indices: &Vec<usize>, window, cx| {
                                this.apply_log_viewer_date_select(selected_indices.as_slice(), window, cx);
                            },
                        ),
                        on_level_filter: cx.listener(
                            |this, selected_indices: &Vec<usize>, window, cx| {
                                this.apply_log_viewer_level_filter(selected_indices.as_slice(), window, cx);
                            },
                        ),
                    },
                    available_themes,
                )
            }));

        div()
            .size_full()
            .h_flex()
            .child(icon_strip)
            .child(nav)
            .child(main_content)
    }
}