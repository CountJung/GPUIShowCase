use gpui::*;
use gpui_component::{
    StyledExt,
    resizable::{h_resizable, resizable_panel},
};

use crate::app::layout::{content, header, sidebar};
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
    fn apply_route(
        &mut self,
        route: AppRoute,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
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

    fn apply_component(
        &mut self,
        component: ComponentId,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
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
}

impl Render for AppRoot {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let sidebar = sidebar::render_sidebar(
            &self.state,
            cx.listener(|this, route: &AppRoute, w, cx| {
                this.apply_route(*route, w, cx);
            }),
            cx.listener(|this, cat: &ComponentCategory, w, cx| {
                this.apply_category(*cat, w, cx);
            }),
            cx.listener(|this, id: &ComponentId, w, cx| {
                this.apply_component(*id, w, cx);
            }),
            cx.listener(|this, _: &ClickEvent, _, cx| {
                this.state.toggle_sidebar();
                cx.notify();
            }),
        );

        let main_content = div()
            .size_full()
            .v_flex()
            .gap_3()
            .child(header::render_header(&self.state))
            .child(content::render_content(
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
                        this.apply_playground_action(
                            PlaygroundAction::ToggleSelected,
                            window,
                            cx,
                        );
                    }),
                    on_toggle_loading: cx.listener(|this, _: &ClickEvent, window, cx| {
                        this.apply_playground_action(
                            PlaygroundAction::ToggleLoading,
                            window,
                            cx,
                        );
                    }),
                    on_toggle_compact: cx.listener(|this, _: &ClickEvent, window, cx| {
                        this.apply_playground_action(
                            PlaygroundAction::ToggleCompact,
                            window,
                            cx,
                        );
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
                            this.apply_log_level_selection(
                                selected_indices.as_slice(),
                                window,
                                cx,
                            );
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
                },
            ));

        div()
            .size_full()
            .child(
                h_resizable("showcase-main")
                    .child(
                        resizable_panel()
                            .size(px(260.0))
                            .size_range(px(160.0)..px(400.0))
                            .child(sidebar),
                    )
                    .child(resizable_panel().child(main_content)),
            )
    }
}
