use gpui::*;
use gpui_component::{
    Root,
    theme::{Theme, ThemeRegistry},
};

mod app;
mod features;
mod shared;

use shared::{
    logger::{self, AppLogLevel},
    state::{AppRoute, AppState, ComponentCategory, ComponentId, PlaygroundAction},
};

#[derive(Default)]
struct LaunchOverrides {
    route: Option<AppRoute>,
    component_category: Option<ComponentCategory>,
    component: Option<ComponentId>,
    playground_script: Vec<PlaygroundAction>,
}

fn main() {
    // Debug builds generate deep call stacks through GPUI's generic element types.
    // Running on a 64 MB stack prevents stack-overflow crashes during debugging.
    std::thread::Builder::new()
        .name("gpuishowcase-main".to_string())
        .stack_size(64 * 1024 * 1024)
        .spawn(run)
        .expect("failed to spawn main thread")
        .join()
        .expect("main thread panicked");
}

fn run() {
    let logger_runtime =
        logger::init_logging(AppLogLevel::Info).expect("failed to initialize logging");
    let launch_overrides = launch_overrides_from_process();
    let app = Application::new();

    app.run(move |cx| {
        gpui_component::init(cx);

        // Load JSON preset themes and apply the GitHub pair as the default theme set.
        let themes_dir = std::path::PathBuf::from("themes");
        if let Err(e) = ThemeRegistry::watch_dir(themes_dir, cx, |cx| {
            if let Some(light) = ThemeRegistry::global(cx)
                .themes()
                .get(&gpui::SharedString::from("GitHub Light"))
                .cloned()
            {
                Theme::global_mut(cx).light_theme = light;
            }
            // Apply GitHub Dark as the default dark theme.
            if let Some(dark) = ThemeRegistry::global(cx)
                .themes()
                .get(&gpui::SharedString::from("GitHub Dark"))
                .cloned()
            {
                Theme::global_mut(cx).dark_theme = dark;
            }
            gpui_component::Theme::change(gpui::WindowAppearance::Dark, None, cx);
            tracing::info!("Theme presets loaded; GitHub Dark applied as the startup theme");
        }) {
            tracing::error!("Failed to watch themes directory: {}", e);
        }

        let initial_state = initial_state_from_launch(&launch_overrides, &logger_runtime);

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let initial_state = initial_state.clone();
                let view = cx.new(move |_| app::root::AppRoot::new(initial_state.clone()));
                cx.new(|cx| Root::new(view, window, cx))
            })
            .expect("failed to open GPUIShowCase window");
        })
        .detach();
    });
}

fn launch_overrides_from_process() -> LaunchOverrides {
    let mut overrides = LaunchOverrides {
        route: std::env::var("GPUISHOWCASE_START_ROUTE")
            .ok()
            .and_then(|value| AppRoute::from_label(&value)),
        component_category: std::env::var("GPUISHOWCASE_COMPONENT_CATEGORY")
            .ok()
            .and_then(|value| ComponentCategory::from_label(&value)),
        component: std::env::var("GPUISHOWCASE_COMPONENT")
            .ok()
            .and_then(|value| ComponentId::from_label(&value)),
        playground_script: std::env::var("GPUISHOWCASE_PLAYGROUND_SCRIPT")
            .ok()
            .map(|value| parse_playground_script(&value))
            .unwrap_or_default(),
    };

    let mut args = std::env::args().skip(1);
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--route" => {
                overrides.route = args.next().and_then(|value| AppRoute::from_label(&value));
            }
            "--category" => {
                overrides.component_category = args
                    .next()
                    .and_then(|value| ComponentCategory::from_label(&value));
            }
            "--component" => {
                overrides.component = args
                    .next()
                    .and_then(|value| ComponentId::from_label(&value));
            }
            "--playground-script" => {
                overrides.playground_script = args
                    .next()
                    .map(|value| parse_playground_script(&value))
                    .unwrap_or_default();
            }
            _ => {}
        }
    }

    overrides
}

fn initial_state_from_launch(
    overrides: &LaunchOverrides,
    logger_runtime: &shared::logger::LoggerRuntime,
) -> AppState {
    let mut state = AppState::new();
    state.register_logging_runtime(logger_runtime);

    if let Some(route) = overrides.route {
        state.navigate_to(route);
    }

    if state.current_route == AppRoute::Components {
        if let Some(category) = overrides.component_category {
            state.select_component_category(category);
        }

        if let Some(component) = overrides.component {
            state.select_component(component);
        }
    }

    if state.current_route == AppRoute::Playground {
        for action in &overrides.playground_script {
            state.apply_playground_action(*action);
        }
    }

    state
}

fn parse_playground_script(value: &str) -> Vec<PlaygroundAction> {
    value
        .split(',')
        .filter_map(PlaygroundAction::from_label)
        .collect()
}
