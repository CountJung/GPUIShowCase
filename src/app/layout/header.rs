use gpui::*;
use gpui_component::StyledExt;

use crate::shared::state::{AppRoute, AppState};

pub fn render_header(state: &AppState) -> impl IntoElement {
    let palette = HeaderPalette::from_state(state);
    let title = if state.current_route == AppRoute::Components {
        state
            .selected_component
            .map(|component| component.title())
            .unwrap_or("Showcase")
    } else {
        state.current_route.title()
    };

    div()
        .flex_shrink_0()
        .h(px(54.0))
        .h_flex()
        .items_center()
        .gap_3()
        .px_4()
        .bg(palette.background)
        .border_b_1()
        .border_color(palette.border)
        .child(
            div()
                .font_weight(FontWeight::BOLD)
                .text_size(rems(1.02))
                .text_color(palette.foreground)
                .child(title),
        )
        .child(
            div()
                .text_sm()
                .text_color(palette.muted)
                .child(state.route_description()),
        )
        .child(div().flex_1())
        .child(
            div()
                .text_xs()
                .text_color(palette.accent)
                .child("GitHub theme active"),
        )
        .child(
            div()
                .w(px(180.0))
                .h(px(30.0))
                .rounded(px(8.0))
                .bg(palette.search_background)
                .border_1()
                .border_color(palette.border)
                .px_3()
                .flex()
                .items_center()
                .text_xs()
                .text_color(palette.muted)
                .child("Preview / props / docs"),
        )
}

#[derive(Clone, Copy)]
struct HeaderPalette {
    background: Rgba,
    border: Rgba,
    foreground: Rgba,
    muted: Rgba,
    accent: Rgba,
    search_background: Rgba,
}

impl HeaderPalette {
    fn from_state(state: &AppState) -> Self {
        if state
            .settings
            .selected_theme_name
            .to_string()
            .contains("Dark")
        {
            Self {
                background: rgba(0x0d1117ff),
                border: rgba(0x30363dff),
                foreground: rgba(0xe6edf3ff),
                muted: rgba(0x8b949eff),
                accent: rgba(0x3fb950ff),
                search_background: rgba(0x161b22ff),
            }
        } else {
            Self {
                background: rgba(0xffffffff),
                border: rgba(0xd0d7deff),
                foreground: rgba(0x24292fff),
                muted: rgba(0x6e7781ff),
                accent: rgba(0x0969daff),
                search_background: rgba(0xf6f8faff),
            }
        }
    }
}
