use gpui::*;
use gpui_component::{
    Selectable as _, StyledExt,
    badge::Badge,
    button::{Button, ButtonGroup},
};

use crate::shared::state::{AppState, PlaygroundPreviewKind, PlaygroundSnapshot};

pub fn render_playground(
    state: &AppState,
    on_preview_kind_select: impl Fn(&Vec<usize>, &mut Window, &mut App) + 'static,
    on_toggle_selected: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    on_toggle_loading: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    on_toggle_compact: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    on_increase_intensity: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    on_decrease_intensity: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let snapshot = &state.playground.current;

    div()
        .size_full()
        .h_flex()
        .gap_3()
        .p_3()
        .child(
            playground_panel("Real-time Composition")
                .w(px(360.0))
                .child(
                    ButtonGroup::new("playground-preview-kind")
                        .outline()
                        .compact()
                        .child(preview_kind_button(
                            PlaygroundPreviewKind::Button,
                            snapshot.preview_kind,
                        ))
                        .child(preview_kind_button(
                            PlaygroundPreviewKind::Badge,
                            snapshot.preview_kind,
                        ))
                        .child(preview_kind_button(
                            PlaygroundPreviewKind::Status,
                            snapshot.preview_kind,
                        ))
                        .on_click(on_preview_kind_select),
                )
                .child(
                    div()
                        .h_flex()
                        .gap_2()
                        .child(
                            Button::new("playground-toggle-selected")
                                .label("Selected")
                                .outline()
                                .selected(snapshot.selected)
                                .on_click(on_toggle_selected),
                        )
                        .child(
                            Button::new("playground-toggle-loading")
                                .label("Loading")
                                .outline()
                                .selected(snapshot.loading)
                                .on_click(on_toggle_loading),
                        )
                        .child(
                            Button::new("playground-toggle-compact")
                                .label("Compact")
                                .outline()
                                .selected(snapshot.compact)
                                .on_click(on_toggle_compact),
                        ),
                )
                .child(
                    div()
                        .h_flex()
                        .gap_2()
                        .items_center()
                        .child(
                            Button::new("playground-intensity-down")
                                .label("-")
                                .outline()
                                .on_click(on_decrease_intensity),
                        )
                        .child(format!("Intensity: {}", snapshot.intensity))
                        .child(
                            Button::new("playground-intensity-up")
                                .label("+")
                                .outline()
                                .on_click(on_increase_intensity),
                        ),
                )
                .child(render_preview_widget(snapshot))
                .child(render_snapshot_block(
                    "Current Snapshot",
                    &snapshot.summary(),
                )),
        )
        .child(render_user_log_panel(state))
        .child(render_debug_panel(state))
}

fn render_preview_widget(snapshot: &PlaygroundSnapshot) -> AnyElement {
    match snapshot.preview_kind {
        PlaygroundPreviewKind::Button => Button::new("playground-preview-button")
            .label(if snapshot.loading {
                "Loading..."
            } else if snapshot.compact {
                "Compact Action"
            } else {
                "Preview Action"
            })
            .outline()
            .selected(snapshot.selected)
            .into_any_element(),
        PlaygroundPreviewKind::Badge => {
            Badge::new()
                .count(snapshot.intensity as usize)
                .child(div().px_3().py_2().border_1().rounded(px(10.0)).child(
                    if snapshot.selected {
                        "Selected Badge"
                    } else {
                        "Badge Preview"
                    },
                ))
                .into_any_element()
        }
        PlaygroundPreviewKind::Status => div()
            .v_flex()
            .gap_1()
            .p_3()
            .border_1()
            .rounded(px(10.0))
            .child(format!("selected: {}", snapshot.selected))
            .child(format!("loading: {}", snapshot.loading))
            .child(format!("compact: {}", snapshot.compact))
            .child(format!("intensity: {}", snapshot.intensity))
            .into_any_element(),
    }
}

fn render_user_log_panel(state: &AppState) -> Div {
    let mut panel = playground_panel("User Event Log")
        .flex_1()
        .child("사용자 관점 이벤트만 별도로 누적해 UI 동작 흐름을 추적합니다.");

    for entry in state.playground.user_events.iter().rev() {
        panel = panel.child(render_log_entry(entry));
    }

    panel
}

fn render_debug_panel(state: &AppState) -> Div {
    let mut panel = playground_panel("State Change Trace")
        .flex_1()
        .child(render_snapshot_block(
            "Previous Snapshot",
            &state.playground.previous.summary(),
        ))
        .child(render_snapshot_block(
            "Current Snapshot",
            &state.playground.current.summary(),
        ))
        .child("디버깅용 로그는 이전/현재 상태 비교와 함께 별도로 유지합니다.");

    for entry in state.playground.debug_events.iter().rev() {
        panel = panel.child(render_log_entry(entry));
    }

    panel
}

fn render_snapshot_block(title: &'static str, body: &str) -> Div {
    div()
        .v_flex()
        .gap_1()
        .p_2()
        .border_1()
        .rounded(px(8.0))
        .child(div().font_weight(FontWeight::BOLD).child(title))
        .child(body.to_string())
}

fn render_log_entry(body: &str) -> Div {
    div()
        .p_2()
        .border_1()
        .rounded(px(8.0))
        .child(body.to_string())
}

fn playground_panel(title: &'static str) -> Div {
    div()
        .v_flex()
        .gap_2()
        .p_3()
        .border_1()
        .rounded(px(12.0))
        .child(div().font_weight(FontWeight::BOLD).child(title))
}

fn preview_kind_button(
    kind: PlaygroundPreviewKind,
    selected_kind: PlaygroundPreviewKind,
) -> Button {
    Button::new(kind.title())
        .label(kind.title())
        .outline()
        .selected(kind == selected_kind)
}
