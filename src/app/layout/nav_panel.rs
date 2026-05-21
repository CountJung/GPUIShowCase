use gpui::*;
use gpui_component::{StyledExt, scroll::ScrollableElement as _};

use crate::features::components;
use crate::shared::state::{AppRoute, AppState, ComponentCategory, ComponentId};

type CategoryHandler = std::rc::Rc<dyn Fn(&ComponentCategory, &mut Window, &mut App)>;
type ComponentHandler = std::rc::Rc<dyn Fn(&ComponentId, &mut Window, &mut App)>;

const CATEGORIES: [ComponentCategory; 5] = [
    ComponentCategory::Basic,
    ComponentCategory::Form,
    ComponentCategory::Layout,
    ComponentCategory::Data,
    ComponentCategory::Advanced,
];

pub fn render_nav_panel(
    state: &AppState,
    on_category_select: impl Fn(&ComponentCategory, &mut Window, &mut App) + 'static,
    on_component_select: impl Fn(&ComponentId, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    use std::rc::Rc;

    let palette = PanelPalette::from_state(state);
    let on_cat: CategoryHandler = Rc::new(on_category_select);
    let on_comp: ComponentHandler = Rc::new(on_component_select);

    div()
        .flex_shrink_0()
        .w(px(328.0))
        .h_full()
        .v_flex()
        .bg(palette.background)
        .border_r_1()
        .border_color(palette.border)
        .child(panel_header(state, palette))
        .child(if state.current_route == AppRoute::Components {
            render_component_index(state, palette, on_cat, on_comp).into_any_element()
        } else {
            render_route_summary(state, palette).into_any_element()
        })
}

fn panel_header(state: &AppState, palette: PanelPalette) -> Div {
    div()
        .flex_shrink_0()
        .v_flex()
        .gap_3()
        .p_4()
        .border_b_1()
        .border_color(palette.border)
        .child(
            div()
                .h_flex()
                .items_center()
                .child(
                    div()
                        .font_weight(FontWeight::BOLD)
                        .text_size(rems(1.05))
                        .text_color(palette.foreground)
                        .child(if state.current_route == AppRoute::Components {
                            "Examples"
                        } else {
                            state.current_route.title()
                        }),
                )
                .child(div().flex_1())
                .child(
                    div()
                        .text_xs()
                        .px_2()
                        .py_px()
                        .rounded(px(4.0))
                        .bg(palette.badge_background)
                        .text_color(palette.accent)
                        .child("gpui-component"),
                ),
        )
        .child(
            div()
                .h(px(34.0))
                .rounded(px(8.0))
                .border_1()
                .border_color(palette.border)
                .bg(palette.search_background)
                .px_3()
                .flex()
                .items_center()
                .text_sm()
                .text_color(palette.muted)
                .child("Type / to search examples"),
        )
}

fn render_component_index(
    state: &AppState,
    palette: PanelPalette,
    on_cat: CategoryHandler,
    on_comp: ComponentHandler,
) -> AnyElement {
    let selected_component = state.selected_component.unwrap_or(ComponentId::Button);

    div()
        .flex_1()
        .overflow_y_scrollbar()
        .v_flex()
        .pb_3()
        .children(CATEGORIES.into_iter().map(|category| {
            let category_selected = state.selected_component_category == Some(category);
            let on_cat_clone = on_cat.clone();
            let on_comp_clone = on_comp.clone();

            div()
                .v_flex()
                .child(
                    div()
                        .h_flex()
                        .items_center()
                        .px_4()
                        .pt_4()
                        .pb_2()
                        .cursor_pointer()
                        .on_mouse_up(MouseButton::Left, move |_, window, cx| {
                            on_cat_clone(&category, window, cx);
                        })
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(if category_selected {
                                    palette.accent
                                } else {
                                    palette.muted
                                })
                                .child(category.title()),
                        )
                        .child(div().flex_1())
                        .child(
                            div().text_xs().text_color(palette.muted).child(
                                components::entries_for_category(category).len().to_string(),
                            ),
                        ),
                )
                .children(
                    components::entries_for_category(category)
                        .iter()
                        .map(move |entry| {
                            let id = entry.id;
                            let is_selected = selected_component == id;
                            let on_comp_item = on_comp_clone.clone();
                            example_row(entry.title, entry.summary, is_selected, palette)
                                .on_mouse_up(MouseButton::Left, move |_, window, cx| {
                                    on_comp_item(&id, window, cx);
                                })
                        }),
                )
        }))
        .into_any_element()
}

fn example_row(
    title: &'static str,
    summary: &'static str,
    selected: bool,
    palette: PanelPalette,
) -> Div {
    div()
        .mx_3()
        .mb_1()
        .px_3()
        .py_2()
        .rounded(px(7.0))
        .cursor_pointer()
        .bg(if selected {
            palette.selected_background
        } else {
            palette.background
        })
        .border_1()
        .border_color(if selected {
            palette.selected_border
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
        .child(
            div()
                .h_flex()
                .gap_2()
                .items_center()
                .child(div().w(px(7.0)).h(px(7.0)).rounded_full().bg(if selected {
                    palette.accent
                } else {
                    palette.muted
                }))
                .child(
                    div()
                        .font_weight(if selected {
                            FontWeight::BOLD
                        } else {
                            FontWeight::MEDIUM
                        })
                        .text_color(palette.foreground)
                        .child(title),
                ),
        )
        .child(
            div()
                .mt_1()
                .text_xs()
                .text_color(palette.muted)
                .child(summary),
        )
}

fn render_route_summary(state: &AppState, palette: PanelPalette) -> Div {
    div()
        .flex_1()
        .v_flex()
        .gap_2()
        .p_4()
        .child(
            div()
                .text_color(palette.foreground)
                .font_weight(FontWeight::BOLD)
                .child(state.current_route.title()),
        )
        .child(
            div()
                .text_sm()
                .text_color(palette.muted)
                .child(state.current_route.description()),
        )
        .child(
            div()
                .mt_3()
                .p_3()
                .rounded(px(8.0))
                .border_1()
                .border_color(palette.border)
                .text_sm()
                .text_color(palette.foreground)
                .child("왼쪽 레일의 Showcase로 돌아가면 모든 examples를 다시 선택할 수 있습니다."),
        )
}

#[derive(Clone, Copy)]
struct PanelPalette {
    background: Rgba,
    border: Rgba,
    foreground: Rgba,
    muted: Rgba,
    accent: Rgba,
    hover_background: Rgba,
    selected_background: Rgba,
    selected_border: Rgba,
    badge_background: Rgba,
    search_background: Rgba,
}

impl PanelPalette {
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
                hover_background: rgba(0x161b22ff),
                selected_background: rgba(0x1b341fff),
                selected_border: rgba(0x3fb950ff),
                badge_background: rgba(0x102a14ff),
                search_background: rgba(0x161b22ff),
            }
        } else {
            Self {
                background: rgba(0xffffffff),
                border: rgba(0xd0d7deff),
                foreground: rgba(0x24292fff),
                muted: rgba(0x6e7781ff),
                accent: rgba(0x0969daff),
                hover_background: rgba(0xf6f8faff),
                selected_background: rgba(0x0969da15),
                selected_border: rgba(0x0969daff),
                badge_background: rgba(0xddf4ffff),
                search_background: rgba(0xf6f8faff),
            }
        }
    }
}
