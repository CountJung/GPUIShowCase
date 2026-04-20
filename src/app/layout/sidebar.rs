use gpui::*;
use gpui::prelude::FluentBuilder as _;
use gpui_component::{
    IconName,
    sidebar::{Sidebar, SidebarMenu, SidebarMenuItem, SidebarToggleButton},
};

use crate::{
    features::components,
    shared::state::{AppRoute, AppState, ComponentCategory, ComponentId},
};

pub fn render_sidebar(
    state: &AppState,
    on_route_select: impl Fn(&AppRoute, &mut Window, &mut App) + 'static,
    on_category_select: impl Fn(&ComponentCategory, &mut Window, &mut App) + 'static,
    on_component_select: impl Fn(&ComponentId, &mut Window, &mut App) + 'static,
    on_toggle: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    use std::rc::Rc;

    let on_route = Rc::new(on_route_select);
    let on_cat = Rc::new(on_category_select);
    let on_comp = Rc::new(on_component_select);

    let current = state.current_route;
    let sel_cat = state.selected_component_category;
    let sel_comp = state.selected_component;
    let collapsed = state.sidebar_collapsed;

    // Build component submenu items per category
    let component_children: Vec<SidebarMenuItem> = [
        ComponentCategory::Basic,
        ComponentCategory::Form,
        ComponentCategory::Layout,
        ComponentCategory::Data,
        ComponentCategory::Advanced,
    ]
    .iter()
    .map(|&cat| {
        let cat_active = sel_cat == Some(cat) && current == AppRoute::Components;
        let on_cat_clone = on_cat.clone();
        let on_comp_clone = on_comp.clone();

        let component_items: Vec<SidebarMenuItem> = components::entries_for_category(cat)
            .iter()
            .map(|entry| {
                let id = entry.id;
                let comp_active = sel_comp == Some(id) && current == AppRoute::Components;
                let on_comp_item = on_comp_clone.clone();
                SidebarMenuItem::new(entry.title)
                    .active(comp_active)
                    .on_click(move |_ev, w, cx| {
                        on_comp_item(&id, w, cx);
                    })
            })
            .collect();

        SidebarMenuItem::new(cat.title())
            .icon(category_icon(cat))
            .active(cat_active)
            .default_open(cat_active)
            .click_to_open(true)
            .on_click(move |_ev, w, cx| {
                on_cat_clone(&cat, w, cx);
            })
            .children(component_items)
    })
    .collect();

    // Build top-level route items
    let build_route_item = |route: AppRoute, icon: IconName| {
        let on_route_clone = on_route.clone();
        let active = current == route;
        SidebarMenuItem::new(route.title())
            .icon(icon)
            .active(active)
            .on_click(move |_ev, w, cx| {
                on_route_clone(&route, w, cx);
            })
    };

    let menu = SidebarMenu::new()
        .child(build_route_item(AppRoute::Dashboard, IconName::LayoutDashboard))
        .child(
            SidebarMenuItem::new(AppRoute::Components.title())
                .icon(IconName::Frame)
                .active(current == AppRoute::Components)
                .default_open(current == AppRoute::Components)
                .click_to_open(true)
                .on_click({
                    let on_route2 = on_route.clone();
                    move |_ev, w, cx| {
                        on_route2(&AppRoute::Components, w, cx);
                    }
                })
                .children(component_children),
        )
        .child(build_route_item(AppRoute::Playground, IconName::SquareTerminal))
        .child(build_route_item(AppRoute::Data, IconName::File))
        .child(build_route_item(AppRoute::Settings, IconName::Settings))
        .child(build_route_item(AppRoute::Logs, IconName::Inbox));

    let header = div()
        .flex()
        .flex_row()
        .gap_2()
        .items_center()
        .p_2()
        .child(
            SidebarToggleButton::left()
                .collapsed(collapsed)
                .on_click(on_toggle),
        )
        .when(!collapsed, |el| {
            el.child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_size(rems(1.1))
                    .child("GPUIShowCase"),
            )
        });

    Sidebar::<SidebarMenu>::left()
        .collapsed(collapsed)
        .header(header)
        .child(menu)
}

fn category_icon(cat: ComponentCategory) -> IconName {
    match cat {
        ComponentCategory::Basic => IconName::Frame,
        ComponentCategory::Form => IconName::GalleryVerticalEnd,
        ComponentCategory::Layout => IconName::PanelLeft,
        ComponentCategory::Data => IconName::File,
        ComponentCategory::Advanced => IconName::ChartPie,
    }
}

