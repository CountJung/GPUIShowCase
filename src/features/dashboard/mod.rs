use gpui::*;
use gpui_component::{StyledExt, badge::Badge, progress::Progress};

use crate::shared::state::{AppRoute, AppState};

const REGISTERED_COMPONENT_COUNT: usize = 24;
const FRAME_BUDGET_MS: f32 = 16.0;
const PLACEHOLDER_RENDER_MS: f32 = 9.4;

#[derive(Clone, Copy, Debug, PartialEq)]
struct ChartPoint {
    label: &'static str,
    value_ms: f32,
    bar_height: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct SystemOverview {
    tracked_routes: usize,
    os_label: &'static str,
    current_route: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ComponentOverview {
    registered_components: usize,
    planned_groups: usize,
    showcased_routes: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct PerformanceOverview {
    frame_budget_ms: f32,
    estimated_render_ms: f32,
    readiness_percent: f32,
}

#[derive(Clone, Debug, PartialEq)]
struct DashboardModel {
    system: SystemOverview,
    component: ComponentOverview,
    performance: PerformanceOverview,
    chart_points: [ChartPoint; 5],
}

impl DashboardModel {
    fn from_state(state: &AppState) -> Self {
        let estimated_render_ms = PLACEHOLDER_RENDER_MS;
        let readiness_percent =
            ((FRAME_BUDGET_MS - estimated_render_ms).max(0.0) / FRAME_BUDGET_MS) * 100.0;

        Self {
            system: SystemOverview {
                tracked_routes: AppRoute::ALL.len(),
                os_label: std::env::consts::OS,
                current_route: state.route_title(),
            },
            component: ComponentOverview {
                registered_components: REGISTERED_COMPONENT_COUNT,
                planned_groups: 5,
                showcased_routes: AppRoute::ALL.len(),
            },
            performance: PerformanceOverview {
                frame_budget_ms: FRAME_BUDGET_MS,
                estimated_render_ms,
                readiness_percent,
            },
            chart_points: [
                ChartPoint {
                    label: "Boot",
                    value_ms: 6.2,
                    bar_height: 48.0,
                },
                ChartPoint {
                    label: "Shell",
                    value_ms: 8.3,
                    bar_height: 62.0,
                },
                ChartPoint {
                    label: "State",
                    value_ms: 9.4,
                    bar_height: 72.0,
                },
                ChartPoint {
                    label: "Theme",
                    value_ms: 7.6,
                    bar_height: 58.0,
                },
                ChartPoint {
                    label: "Logs",
                    value_ms: 10.1,
                    bar_height: 78.0,
                },
            ],
        }
    }
}

pub fn render_dashboard(state: &AppState) -> impl IntoElement {
    let model = DashboardModel::from_state(state);

    div()
        .v_flex()
        .gap_3()
        .child(render_banner(&model))
        .child(
            div()
                .h_flex()
                .gap_3()
                .child(render_system_card(&model.system))
                .child(render_component_card(&model.component)),
        )
        .child(
            div()
                .h_flex()
                .gap_3()
                .child(render_performance_card(&model.performance)),
        )
        .child(render_chart_card(&model.chart_points))
        .child(render_notes_card())
}

fn render_banner(model: &DashboardModel) -> Div {
    dashboard_card()
        .child(
            div()
                .text_xl()
                .font_weight(FontWeight::BOLD)
                .child("Dashboard MVP"),
        )
        .child("프로젝트 상태를 보여주는 첫 실제 화면입니다.")
        .child(format!(
            "현재 라우트: {} | 추적 라우트: {} | 운영체제: {}",
            model.system.current_route, model.system.tracked_routes, model.system.os_label
        ))
}

fn render_system_card(system: &SystemOverview) -> Div {
    dashboard_card()
        .flex_1()
        .child(card_title("System Overview"))
        .child(format!("운영체제: {}", system.os_label))
        .child(format!("현재 화면: {}", system.current_route))
        .child(format!("연결된 라우트 수: {}", system.tracked_routes))
}

fn render_component_card(component: &ComponentOverview) -> Div {
    dashboard_card()
        .flex_1()
        .child(card_title("Component Summary"))
        .child(
            Badge::new().count(component.registered_components).child(
                div()
                    .px_3()
                    .py_2()
                    .border_1()
                    .rounded(px(10.0))
                    .child("Tracked showcase components"),
            ),
        )
        .child(format!("예정 카테고리 수: {}", component.planned_groups))
        .child(format!(
            "셸에 노출된 라우트 수: {}",
            component.showcased_routes
        ))
}

fn render_performance_card(performance: &PerformanceOverview) -> Div {
    dashboard_card()
        .flex_1()
        .child(card_title("Render Performance"))
        .child(
            div()
                .h_flex()
                .gap_3()
                .items_center()
                .child(
                    div()
                        .min_w(px(84.0))
                        .font_weight(FontWeight::BOLD)
                        .child(format!("{:.0}%", performance.readiness_percent)),
                )
                .child(
                    div()
                        .v_flex()
                        .gap_1()
                        .child(format!(
                            "프레임 예산: {:.1} ms",
                            performance.frame_budget_ms
                        ))
                        .child(format!(
                            "현재 추정 렌더링: {:.1} ms",
                            performance.estimated_render_ms
                        ))
                        .child(format!(
                            "예산 여유율: {:.0}%",
                            performance.readiness_percent
                        )),
                ),
        )
        .child(Progress::new().value(performance.readiness_percent))
}

fn render_chart_card(chart_points: &[ChartPoint; 5]) -> Div {
    dashboard_card()
        .child(card_title("Chart Placeholder"))
        .child("실시간 계측 연결 전까지는 고정 샘플 값을 사용합니다.")
        .child(
            div()
                .h_flex()
                .gap_3()
                .items_end()
                .child(render_chart_bar(chart_points[0]))
                .child(render_chart_bar(chart_points[1]))
                .child(render_chart_bar(chart_points[2]))
                .child(render_chart_bar(chart_points[3]))
                .child(render_chart_bar(chart_points[4])),
        )
}

fn render_chart_bar(point: ChartPoint) -> Div {
    div()
        .v_flex()
        .gap_1()
        .items_center()
        .child(
            div()
                .w(px(30.0))
                .h(px(point.bar_height))
                .border_1()
                .rounded(px(8.0)),
        )
        .child(point.label)
        .child(format!("{:.1} ms", point.value_ms))
}

fn render_notes_card() -> Div {
    dashboard_card()
		.child(card_title("Data Notes"))
		.child("계산값: 현재 라우트, 라우트 수")
		.child("하드코딩값: 컴포넌트 수, 차트 샘플, 초기 렌더 추정치")
		.child("향후 실제 성능 계측은 DashboardModel::from_state에서 PLACEHOLDER_RENDER_MS를 교체하는 방식으로 연결합니다.")
}

fn dashboard_card() -> Div {
    div().v_flex().gap_2().p_3().border_1().rounded(px(12.0))
}

fn card_title(title: &'static str) -> Div {
    div().font_weight(FontWeight::BOLD).child(title)
}

#[cfg(test)]
mod tests {
    use super::DashboardModel;
    use crate::shared::state::{AppRoute, AppState};

    #[test]
    fn dashboard_model_reflects_route_count() {
        let state = AppState::new();

        let model = DashboardModel::from_state(&state);

        assert_eq!(model.system.tracked_routes, AppRoute::ALL.len());
    }

    #[test]
    fn dashboard_model_uses_expected_component_baseline() {
        let state = AppState::new();

        let model = DashboardModel::from_state(&state);

        assert_eq!(model.component.registered_components, 24);
        assert!(model.performance.readiness_percent > 0.0);
    }
}
