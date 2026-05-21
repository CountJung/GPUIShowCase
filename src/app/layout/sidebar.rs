use gpui::*;
use gpui::prelude::FluentBuilder as _;
use gpui_component::{Icon, IconName, StyledExt};

use crate::shared::state::{AppRoute, AppState};

/// Narrow icon strip (48 px).
/// Background, border, and active/hover colours use GitHub Light palette values
/// so the sidebar is always clearly visible regardless of the loaded theme.
pub fn render_icon_strip(
    state: &AppState,
    on_route_select: impl Fn(&AppRoute, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    use std::rc::Rc;

    let on_route: Rc<dyn Fn(&AppRoute, &mut Window, &mut App)> = Rc::new(on_route_select);
    let current = state.current_route;

    const NAV: [(AppRoute, IconName); 6] = [
        (AppRoute::Dashboard, IconName::LayoutDashboard),
        (AppRoute::Components, IconName::Frame),
        (AppRoute::Playground, IconName::SquareTerminal),
        (AppRoute::Data, IconName::File),
        (AppRoute::Settings, IconName::Settings),
        (AppRoute::Logs, IconName::Inbox),
    ];

    div()
        .flex_shrink_0()
        .w(px(48.0))
        .h_full()
        .bg(rgba(0xf6f8faff))           // GitHub sidebar.background
        .border_r_1()
        .border_color(rgba(0xd0d7deff)) // GitHub sidebar.border
        .v_flex()
        .items_center()
        .gap_1()
        .py_2()
        // App logo mark — blue badge
        .child(
            div()
                .w(px(36.0))
                .h(px(36.0))
                .flex()
                .items_center()
                .justify_center()
                .rounded(px(8.0))
                .bg(rgba(0x0969daff))   // GitHub primary.background
                .font_weight(FontWeight::BOLD)
                .text_sm()
                .text_color(rgba(0xffffffff))
                .child("GS"),
        )
        .child(div().h(px(4.0)))
        // Route icon buttons
        .children(NAV.into_iter().map(|(route, icon)| {
            let is_active = current == route;
            let on_route_clone = on_route.clone();

            div()
                .w(px(36.0))
                .h(px(36.0))
                .flex()
                .items_center()
                .justify_center()
                .rounded(px(6.0))
                .cursor_pointer()
                // Active: tinted background + left accent border
                .when(is_active, |d| {
                    d.bg(rgba(0xdde3e9ff))
                        .border_l_2()
                        .border_color(rgba(0x0969daff))
                })
                // Inactive: fade slightly on hover
                .when(!is_active, |d| {
                    d.hover(|mut s| {
                        s.background = Some(rgba(0xeaeef2ff).into());
                        s
                    })
                })
                .on_mouse_up(MouseButton::Left, move |_, window, cx| {
                    on_route_clone(&route, window, cx);
                })
                // Icon — blue for active route, muted grey otherwise
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_color(if is_active {
                            rgba(0x0969daff) // GitHub sidebar.accent.foreground
                        } else {
                            rgba(0x57606aff) // GitHub muted text
                        })
                        .child(Icon::new(icon)),
                )
        }))
}