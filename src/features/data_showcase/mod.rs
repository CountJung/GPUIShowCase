use gpui::*;
use gpui_component::{StyledExt, badge::Badge};

use crate::shared::state::AppState;

const SAMPLE_ROW_COUNT: usize = 8;
const LARGE_DATASET_HINT: usize = 2048;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DataRecord {
    service: &'static str,
    path: &'static str,
    owner: &'static str,
    status: &'static str,
    throughput_rps: u16,
    latency_ms: u16,
    error_count: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct TrendPoint {
    label: &'static str,
    throughput_rps: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DataNode {
    depth: usize,
    label: &'static str,
    detail: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct DataShowcaseModel {
    rows: [DataRecord; SAMPLE_ROW_COUNT],
    chart_points: [TrendPoint; SAMPLE_ROW_COUNT],
    file_nodes: [DataNode; SAMPLE_ROW_COUNT],
    total_throughput_rps: u32,
    average_latency_ms: u16,
    total_errors: u16,
    large_dataset_hint: usize,
}

impl DataShowcaseModel {
    fn sample() -> Self {
        let rows = [
            DataRecord {
                service: "Gateway",
                path: "workspace/gateway/routes.rs",
                owner: "Platform",
                status: "Healthy",
                throughput_rps: 840,
                latency_ms: 22,
                error_count: 1,
            },
            DataRecord {
                service: "Scheduler",
                path: "workspace/jobs/scheduler.rs",
                owner: "Operations",
                status: "Healthy",
                throughput_rps: 620,
                latency_ms: 27,
                error_count: 0,
            },
            DataRecord {
                service: "Indexer",
                path: "workspace/search/indexer.rs",
                owner: "Search",
                status: "Monitoring",
                throughput_rps: 410,
                latency_ms: 35,
                error_count: 2,
            },
            DataRecord {
                service: "Asset Sync",
                path: "workspace/assets/sync.rs",
                owner: "Content",
                status: "Healthy",
                throughput_rps: 580,
                latency_ms: 19,
                error_count: 0,
            },
            DataRecord {
                service: "Audit Trail",
                path: "workspace/audit/trail.rs",
                owner: "Security",
                status: "Healthy",
                throughput_rps: 360,
                latency_ms: 31,
                error_count: 1,
            },
            DataRecord {
                service: "Preview API",
                path: "workspace/preview/api.rs",
                owner: "Experience",
                status: "Warning",
                throughput_rps: 290,
                latency_ms: 48,
                error_count: 3,
            },
            DataRecord {
                service: "Exporter",
                path: "workspace/exporter/mod.rs",
                owner: "Reporting",
                status: "Healthy",
                throughput_rps: 510,
                latency_ms: 29,
                error_count: 0,
            },
            DataRecord {
                service: "Workspace Tree",
                path: "workspace/tree/state.rs",
                owner: "Desktop",
                status: "Monitoring",
                throughput_rps: 440,
                latency_ms: 33,
                error_count: 1,
            },
        ];

        let chart_points = rows.map(|row| TrendPoint {
            label: row.service,
            throughput_rps: row.throughput_rps,
        });

        let file_nodes = rows.map(|row| DataNode {
            depth: row.path.matches('/').count().saturating_sub(1),
            label: row.service,
            detail: row.path,
        });

        let total_throughput_rps = rows
            .iter()
            .map(|row| row.throughput_rps as u32)
            .sum::<u32>();
        let average_latency_ms =
            rows.iter().map(|row| row.latency_ms as u32).sum::<u32>() / SAMPLE_ROW_COUNT as u32;
        let total_errors = rows.iter().map(|row| row.error_count).sum::<u16>();

        Self {
            rows,
            chart_points,
            file_nodes,
            total_throughput_rps,
            average_latency_ms: average_latency_ms as u16,
            total_errors,
            large_dataset_hint: LARGE_DATASET_HINT,
        }
    }
}

pub fn render_data_showcase(_: &AppState) -> impl IntoElement {
    let model = DataShowcaseModel::sample();

    div()
        .v_flex()
        .gap_3()
        .p_3()
        .child(render_overview_banner(&model))
        .child(
            div()
                .h_flex()
                .gap_3()
                .items_start()
                .child(render_table_panel(&model))
                .child(render_chart_panel(&model)),
        )
        .child(
            div()
                .h_flex()
                .gap_3()
                .items_start()
                .child(render_file_list_panel(&model))
                .child(render_strategy_panel(&model)),
        )
}

fn render_overview_banner(model: &DataShowcaseModel) -> Div {
    showcase_panel("Shared Dataset Overview")
        .child("표, 차트, 파일 탐색형 리스트가 동일한 운영 데이터 샘플을 참조합니다.")
        .child(
            Badge::new().count(model.rows.len()).child(
                div()
                    .px_3()
                    .py_2()
                    .border_1()
                    .rounded(px(10.0))
                    .child("Shared operational records"),
            ),
        )
        .child(format!(
            "총 처리량: {} rps | 평균 지연: {} ms | 누적 오류: {}",
            model.total_throughput_rps, model.average_latency_ms, model.total_errors
        ))
}

fn render_table_panel(model: &DataShowcaseModel) -> Div {
    let mut panel = showcase_panel("DataTable Sample").flex_1();
    panel = panel.child(table_header_row());

    for row in model.rows {
        panel = panel.child(table_row(row));
    }

    panel
}

fn render_chart_panel(model: &DataShowcaseModel) -> Div {
    let mut panel = showcase_panel("Trend Chart")
        .flex_1()
        .child("같은 데이터셋의 throughput 값을 서비스별 막대 차트로 집계합니다.");

    panel = panel.child(
        div()
            .h_flex()
            .gap_2()
            .items_end()
            .child(render_chart_bar(model.chart_points[0]))
            .child(render_chart_bar(model.chart_points[1]))
            .child(render_chart_bar(model.chart_points[2]))
            .child(render_chart_bar(model.chart_points[3]))
            .child(render_chart_bar(model.chart_points[4]))
            .child(render_chart_bar(model.chart_points[5]))
            .child(render_chart_bar(model.chart_points[6]))
            .child(render_chart_bar(model.chart_points[7])),
    );

    panel
}

fn render_file_list_panel(model: &DataShowcaseModel) -> Div {
    let mut panel = showcase_panel("File Explorer List")
        .flex_1()
        .child("같은 레코드의 path 필드를 파일 탐색형 목록으로 노출합니다.");

    for node in model.file_nodes {
        panel = panel.child(
            div()
                .h_flex()
                .gap_2()
                .pl(px((node.depth as f32) * 12.0))
                .child(div().font_weight(FontWeight::MEDIUM).child(node.label))
                .child(node.detail),
        );
    }

    panel
}

fn render_strategy_panel(model: &DataShowcaseModel) -> Div {
    showcase_panel("Large Dataset Strategy")
        .w(px(340.0))
        .child(format!(
            "현재 UI 샘플은 {}개 레코드를 직접 렌더링합니다.",
            model.rows.len()
        ))
        .child(format!(
            "대용량 기준선은 {}개 synthetic rows를 목표로 두고 VirtualList 또는 집계 렌더링으로 확장합니다.",
            model.large_dataset_hint
        ))
        .child("HM-010은 Data Showcase route가 같은 데이터 구조로 표/차트/리스트를 동시에 렌더링하면서도 초기 표시가 멈추지 않는지 확인하는 smoke 기준으로 관리합니다.")
}

fn table_header_row() -> Div {
    div()
        .h_flex()
        .gap_3()
        .border_b_1()
        .pb_1()
        .child(header_cell("Service", 110.0))
        .child(header_cell("Status", 90.0))
        .child(header_cell("Throughput", 90.0))
        .child(header_cell("Latency", 80.0))
        .child(header_cell("Path", 220.0))
}

fn table_row(row: DataRecord) -> Div {
    div()
        .h_flex()
        .gap_3()
        .border_b_1()
        .py_1()
        .child(body_cell(row.service, 110.0))
        .child(body_cell(row.status, 90.0))
        .child(body_cell(format!("{} rps", row.throughput_rps), 90.0))
        .child(body_cell(format!("{} ms", row.latency_ms), 80.0))
        .child(body_cell(row.path, 220.0))
}

fn render_chart_bar(point: TrendPoint) -> Div {
    let height = (point.throughput_rps as f32 / 10.0).clamp(24.0, 96.0);

    div()
        .v_flex()
        .gap_1()
        .items_center()
        .w(px(72.0))
        .child(div().w(px(28.0)).h(px(height)).border_1().rounded(px(8.0)))
        .child(point.label)
        .child(format!("{} rps", point.throughput_rps))
}

fn header_cell(label: &'static str, width: f32) -> Div {
    div()
        .w(px(width))
        .font_weight(FontWeight::BOLD)
        .child(label)
}

fn body_cell(label: impl Into<SharedString>, width: f32) -> Div {
    div().w(px(width)).child(label.into())
}

fn showcase_panel(title: &'static str) -> Div {
    div()
        .v_flex()
        .gap_2()
        .p_3()
        .border_1()
        .rounded(px(12.0))
        .child(div().font_weight(FontWeight::BOLD).child(title))
}

#[cfg(test)]
mod tests {
    use super::DataShowcaseModel;

    #[test]
    fn data_showcase_uses_one_shared_dataset() {
        let model = DataShowcaseModel::sample();

        assert_eq!(model.rows.len(), 8);
        assert_eq!(model.chart_points.len(), model.rows.len());
        assert_eq!(model.file_nodes.len(), model.rows.len());
        assert!(model.total_throughput_rps > 0);
    }

    #[test]
    fn large_dataset_hint_stays_above_visible_slice() {
        let model = DataShowcaseModel::sample();

        assert!(model.large_dataset_hint > model.rows.len());
        assert!(model.average_latency_ms > 0);
    }
}
