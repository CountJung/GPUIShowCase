use gpui::*;
use gpui::prelude::FluentBuilder as _;
use gpui_component::{scroll::ScrollableElement as _, StyledExt};

use crate::features::components;
use crate::shared::state::{AppRoute, AppState, ComponentCategory, ComponentId};

/// Context navigation panel (220 px default).
/// For the Components route: shows a flat category + component list using plain divs.
/// No SidebarMenuItem → no internal-state reconciliation → no freeze.
pub fn render_nav_panel(
    state: &AppState,
    on_category_select: impl Fn(&ComponentCategory, &mut Window, &mut App) + 'static,
    on_component_select: impl Fn(&ComponentId, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    use std::rc::Rc;

    let route = state.current_route;
    let sel_comp = state.selected_component;

    let header = div()
        .flex_shrink_0()
        .px_3()
        .py_2()
        .border_b_1()
        .h_flex()
        .items_center()
        .gap_2()
        .font_weight(FontWeight::SEMIBOLD)
        .text_sm()
        .child(route.title());

    let body: AnyElement = if route == AppRoute::Components {
        let on_cat: Rc<dyn Fn(&ComponentCategory, &mut Window, &mut App)> =
            Rc::new(on_category_select);
        let on_comp: Rc<dyn Fn(&ComponentId, &mut Window, &mut App)> =
            Rc::new(on_component_select);

        const CATEGORIES: [ComponentCategory; 5] = [
            ComponentCategory::Basic,
            ComponentCategory::Form,
            ComponentCategory::Layout,
            ComponentCategory::Data,
            ComponentCategory::Advanced,
        ];

        div()
            .flex_1()
            .overflow_y_scrollbar()
            .v_flex()
            .pb_2()
            .children(CATEGORIES.into_iter().map(|cat| {
                let on_cat_clone = on_cat.clone();
                let on_comp_clone = on_comp.clone();

                let items: Vec<Div> = components::entries_for_category(cat)
                    .iter()
                    .map(|entry| {
                        let id = entry.id;
                        let is_selected = sel_comp == Some(id);
                        let on_comp_item = on_comp_clone.clone();
                        div()
                            .px_4()
                            .py(px(5.0))
                            .text_sm()
                            .cursor_pointer()
                            .when(is_selected, |d| d.font_weight(FontWeight::SEMIBOLD).border_l_2())
                            .child(entry.title)
                            .on_mouse_up(MouseButton::Left, move |_, window, cx| {
                                on_comp_item(&id, window, cx);
                            })
                    })
                    .collect();

                div()
                    .v_flex()
                    .child(
                        // Category header — click selects the category
                        div()
                            .px_3()
                            .py(px(6.0))
                            .mt_1()
                            .text_xs()
                            .font_weight(FontWeight::BOLD)
                            .cursor_pointer()
                            .on_mouse_up(MouseButton::Left, move |_, window, cx| {
                                on_cat_clone(&cat, window, cx);
                            })
                            .child(cat.title()),
                    )
                    .children(items)
            }))
            .into_any_element()
    } else {
        div()
            .flex_1()
            .p_3()
            .text_sm()
            .child(route.description())
            .into_any_element()
    };

    div()
        .h_full()
        .v_flex()
        .child(header)
        .child(body)
}
