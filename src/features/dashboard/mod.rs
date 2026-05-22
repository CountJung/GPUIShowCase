use gpui::*;
use gpui_component::{StyledExt};

use crate::shared::state::{AppRoute, AppState};

#[derive(Clone, Copy, Debug, PartialEq)]
struct PageCount {
    total_routes: usize,
}

impl PageCount {
    fn from_state(_state: &AppState) -> Self {
        Self {
            total_routes: AppRoute::ALL.len(),
        }
    }
}

pub fn render_dashboard(state: &AppState) -> impl IntoElement {
    let page_count = PageCount::from_state(state);

    div()
        .v_flex()
        .gap_3()
        .child(render_header())
        .child(
            div()
                .h_flex()
                .gap_3()
                .child(render_page_card(page_count)),
        )
}

fn render_header() -> Div {
    dashboard_card()
        .child(
            div()
                .text_xl()
                .font_weight(FontWeight::BOLD)
                .child("Dashboard"),
        )
        .child("Project Status Overview")
}

fn render_page_card(page_count: PageCount) -> Div {
    dashboard_card()
        .flex_1()
        .flex()
        .gap_4()
        .items_center()
        .child(
            div()
                .text_2xl()
                .font_weight(FontWeight::EXTRA_BOLD)
                .child(format!("{}", page_count.total_routes)),
        )
        .child("Registered Pages")
}

fn dashboard_card() -> Div {
    div().v_flex().gap_2().p_3().border_1().rounded(px(12.0))
}

#[cfg(test)]
mod tests {
    use super::PageCount;
    use crate::shared::state::{AppRoute, AppState};

    #[test]
    fn page_count_reflects_route_count() {
        let state = AppState::new();
        let page_count = PageCount::from_state(&state);
        assert_eq!(page_count.total_routes, AppRoute::ALL.len());
    }
}