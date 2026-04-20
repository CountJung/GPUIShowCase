use gpui::*;
use gpui_component::StyledExt;

use crate::shared::state::AppState;

pub fn render_header(state: &AppState) -> impl IntoElement {
    div()
        .v_flex()
        .gap_2()
        .p_3()
        .child(
            div()
                .text_xl()
                .font_weight(FontWeight::BOLD)
                .child(state.route_title()),
        )
        .child(state.route_description())
        .child(state.status_message.clone())
}
