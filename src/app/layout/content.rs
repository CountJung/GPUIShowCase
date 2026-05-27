use gpui::*;

use crate::features::{components, dashboard, data_showcase, log_viewer, playground, settings};
use crate::shared::state::AppState;


pub struct PlaygroundContentActions<
    OnPreviewKindSelect,
    OnToggleSelected,
    OnToggleLoading,
    OnToggleCompact,
    OnIncreaseIntensity,
    OnDecreaseIntensity,
> {
    pub on_preview_kind_select: OnPreviewKindSelect,
    pub on_toggle_selected: OnToggleSelected,
    pub on_toggle_loading: OnToggleLoading,
    pub on_toggle_compact: OnToggleCompact,
    pub on_increase_intensity: OnIncreaseIntensity,
    pub on_decrease_intensity: OnDecreaseIntensity,
}


pub struct SettingsContentActions<
    OnLogLevelSelect,
    OnLayoutDensitySelect,
    OnPerformanceModeSelect,
    OnThemeSelect,
> {
    pub on_log_level_select: OnLogLevelSelect,
    pub on_layout_density_select: OnLayoutDensitySelect,
    pub on_performance_mode_select: OnPerformanceModeSelect,
    pub on_theme_select: OnThemeSelect,
}


pub fn render_content(
    state: &AppState,
    playground_actions: PlaygroundContentActions<
        impl Fn(&Vec<usize>, &mut Window, &mut App) + 'static,
        impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
        impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
        impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
        impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
        impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    >,
    settings_actions: SettingsContentActions<
        impl Fn(&Vec<usize>, &mut Window, &mut App) + 'static,
        impl Fn(&Vec<usize>, &mut Window, &mut App) + 'static,
        impl Fn(&Vec<usize>, &mut Window, &mut App) + 'static,
        impl Fn(&gpui::SharedString, &mut Window, &mut App) + 'static,
    >,
    available_themes: Vec<(gpui::SharedString, bool)>,
) -> impl IntoElement {
    
    match state.current_route {
        crate::shared::state::AppRoute::Dashboard => {
            dashboard::render_dashboard(state).into_any_element()
        }
        crate::shared::state::AppRoute::Components => {
            components::render_component_showcase(state).into_any_element()
        }
        crate::shared::state::AppRoute::Playground => playground::render_playground(
            state,
            playground_actions.on_preview_kind_select,
            playground_actions.on_toggle_selected,
            playground_actions.on_toggle_loading,
            playground_actions.on_toggle_compact,
            playground_actions.on_increase_intensity,
            playground_actions.on_decrease_intensity,
        )
        .into_any_element(),
        crate::shared::state::AppRoute::Data => {
            data_showcase::render_data_showcase(state).into_any_element()
        }
        crate::shared::state::AppRoute::Settings => settings::render_settings(
            state,
            settings_actions.on_log_level_select,
            settings_actions.on_layout_density_select,
            settings_actions.on_performance_mode_select,
            settings_actions.on_theme_select,
            available_themes,
        )
        .into_any_element(),
        crate::shared::state::AppRoute::Logs => {
            log_viewer::widget::render_log_viewer(&mut state.log_viewer.clone()).into_any_element()
        }
    }
}