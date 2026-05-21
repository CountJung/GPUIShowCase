use gpui::*;
use gpui_component::StyledExt;

use crate::shared::state::AppState;

/// Slim title bar at the top of the main content pane.
pub fn render_header(state: &AppState) -> impl IntoElement {
    div()
        .flex_shrink_0()
        .h_flex()
        .items_center()
        .gap_3()
        .px_4()
        .py_2()
        .border_b_1()
        .child(
            div()
                .font_weight(FontWeight::SEMIBOLD)
                .text_base()
                .child(state.route_title()),
        )
        .child(
            div()
                .text_sm()
                .child(state.route_description()),
        )
        .child(div().flex_1())
        .child(div().text_xs().child(state.status_message.clone()))
}

