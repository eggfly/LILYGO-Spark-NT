#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod i18n;
mod manifest;
mod pages;
mod sidebar;
mod theme;

use app::SparkApp;
use gpui::*;

// Define global actions
actions!(
    spark,
    [
        Quit,
        CloseWindow,
        Minimize,
        ToggleFullScreen,
        Zoom,
        Hide,
        HideOthers,
        ShowAll,
    ]
);

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .init();

    log::info!("Starting LILYGO Spark NT v{}", env!("CARGO_PKG_VERSION"));

    Application::new().run(|cx: &mut App| {
        // Register global action handlers
        cx.on_action(|_: &Quit, cx| {
            cx.quit();
        });

        cx.on_action(|_: &CloseWindow, cx| {
            if let Some(window) = cx.active_window() {
                let _ = window.update(cx, |_, window, _| {
                    window.remove_window();
                });
            }
        });

        cx.on_action(|_: &Minimize, cx| {
            if let Some(window) = cx.active_window() {
                let _ = window.update(cx, |_, window, _| {
                    window.minimize_window();
                });
            }
        });

        cx.on_action(|_: &Zoom, cx| {
            if let Some(window) = cx.active_window() {
                let _ = window.update(cx, |_, window, _| {
                    window.zoom_window();
                });
            }
        });

        cx.on_action(|_: &ToggleFullScreen, cx| {
            if let Some(window) = cx.active_window() {
                let _ = window.update(cx, |_, window, _| {
                    window.toggle_fullscreen();
                });
            }
        });

        // macOS-specific actions
        #[cfg(target_os = "macos")]
        {
            cx.on_action(|_: &Hide, cx| cx.hide());
            cx.on_action(|_: &HideOthers, cx| cx.hide_other_apps());
            cx.on_action(|_: &ShowAll, cx| cx.unhide_other_apps());
        }

        // Bind platform-specific key bindings
        #[cfg(target_os = "macos")]
        cx.bind_keys([
            KeyBinding::new("cmd-q", Quit, None),
            KeyBinding::new("cmd-w", CloseWindow, None),
            KeyBinding::new("cmd-m", Minimize, None),
            KeyBinding::new("ctrl-cmd-f", ToggleFullScreen, None),
            KeyBinding::new("cmd-h", Hide, None),
            KeyBinding::new("alt-cmd-h", HideOthers, None),
        ]);

        #[cfg(target_os = "windows")]
        cx.bind_keys([
            KeyBinding::new("ctrl-q", Quit, None),
            KeyBinding::new("alt-f4", Quit, None),
            KeyBinding::new("ctrl-w", CloseWindow, None),
            KeyBinding::new("f11", ToggleFullScreen, None),
        ]);

        #[cfg(target_os = "linux")]
        cx.bind_keys([
            KeyBinding::new("ctrl-q", Quit, None),
            KeyBinding::new("ctrl-w", CloseWindow, None),
            KeyBinding::new("f11", ToggleFullScreen, None),
        ]);

        let bounds = Bounds::centered(None, size(px(1100.0), px(700.0)), cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                window_background: WindowBackgroundAppearance::Blurred,
                titlebar: Some(TitlebarOptions {
                    title: Some("LILYGO Spark NT".into()),
                    appears_transparent: true,
                    traffic_light_position: Some(point(px(12.0), px(12.0))),
                }),
                window_min_size: Some(Size {
                    width: px(800.0),
                    height: px(500.0),
                }),
                ..Default::default()
            },
            |_window, cx| {
                let model = cx.new(|_| SparkApp::new());
                model.update(cx, |app, cx| {
                    app.load_manifest(cx);
                });
                model
            },
        )
        .unwrap();

        cx.activate(true);
    });
}
