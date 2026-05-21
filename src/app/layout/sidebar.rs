use gpui::*;
use gpui_component::{Icon, IconName, StyledExt};

use crate::shared::state::{AppRoute, AppState};

type RouteHandler = std::rc::Rc<dyn Fn(&AppRoute, &mut Window, &mut App)>;

const NAV: [(AppRoute, IconName); 6] = [
    (AppRoute::Dashboard, IconName::LayoutDashboard),
    (AppRoute::Components, IconName::Frame),
    (AppRoute::Playground, IconName::SquareTerminal),
    (AppRoute::Data, IconName::File),
    (AppRoute::Settings, IconName::Settings),
    (AppRoute::Logs, IconName::Inbox),
];

pub fn render_icon_strip(
    state: &AppState,
    on_route_select: impl Fn(&AppRoute, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    use std::rc::Rc;

    let palette = RailPalette::from_state(state);
    let on_route: RouteHandler = Rc::new(on_route_select);
    let current = state.current_route;

    div()
        .flex_shrink_0()
        .w(px(64.0))
        .h_full()
        .bg(palette.background)
        .border_r_1()
        .border_color(palette.border)
        .v_flex()
        .items_center()
        .py_3()
        .gap_3()
        .child(app_mark(palette))
        .child(div().h(px(8.0)))
        .children(NAV.into_iter().map(|(route, icon)| {
            let is_active = current == route;
            let on_route_clone = on_route.clone();

            div()
                .w(px(42.0))
                .h(px(42.0))
                .rounded(px(8.0))
                .flex()
                .items_center()
                .justify_center()
                .cursor_pointer()
                .bg(if is_active {
                    palette.active_background
                } else {
                    palette.background
                })
                .border_1()
                .border_color(if is_active {
                    palette.active_border
                } else {
                    palette.background
                })
                .hover({
                    let hover = palette.hover_background;
                    move |mut style| {
                        style.background = Some(hover.into());
                        style
                    }
                })
                .on_mouse_up(MouseButton::Left, move |_, window, cx| {
                    on_route_clone(&route, window, cx);
                })
                .child(
                    Icon::new(icon)
                        .size_5()
                        .text_color(if is_active {
                            palette.active_foreground
                        } else {
                            palette.foreground
                        }),
                )
        }))
        .child(div().flex_1())
        .child(
            div()
                .w(px(34.0))
                .h(px(34.0))
                .rounded_full()
                .flex()
                .items_center()
                .justify_center()
                .bg(palette.active_background)
                .text_color(palette.active_foreground)
                .text_xs()
                .font_weight(FontWeight::BOLD)
                .child("GH"),
        )
}

fn app_mark(palette: RailPalette) -> Div {
    div()
        .w(px(40.0))
        .h(px(40.0))
        .rounded(px(8.0))
        .flex()
        .items_center()
        .justify_center()
        .bg(palette.logo_background)
        .text_color(rgba(0xffffffff))
        .font_weight(FontWeight::BOLD)
        .child("GS")
}

#[derive(Clone, Copy)]
struct RailPalette {
    background: Rgba,
    border: Rgba,
    foreground: Rgba,
    hover_background: Rgba,
    active_background: Rgba,
    active_border: Rgba,
    active_foreground: Rgba,
    logo_background: Rgba,
}

impl RailPalette {
    fn from_state(state: &AppState) -> Self {
        if state
            .settings
            .selected_theme_name
            .to_string()
            .contains("Dark")
        {
            Self {
                background: rgba(0x010409ff),
                border: rgba(0x30363dff),
                foreground: rgba(0x8b949eff),
                hover_background: rgba(0x161b22ff),
                active_background: rgba(0x0d1117ff),
                active_border: rgba(0x3fb950ff),
                active_foreground: rgba(0x3fb950ff),
                logo_background: rgba(0x238636ff),
            }
        } else {
            Self {
                background: rgba(0xf6f8faff),
                border: rgba(0xd0d7deff),
                foreground: rgba(0x57606aff),
                hover_background: rgba(0xeaeef2ff),
                active_background: rgba(0xffffffff),
                active_border: rgba(0x0969daff),
                active_foreground: rgba(0x0969daff),
                logo_background: rgba(0x0969daff),
            }
        }
    }
}
