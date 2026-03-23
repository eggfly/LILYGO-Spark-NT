#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gpui::*;

// ─── Color palette ───
const BG_PRIMARY: u32 = 0x1a1a2e;
const BG_SECONDARY: u32 = 0x16213e;
const BG_PANEL: u32 = 0x0f3460;
const ACCENT: u32 = 0x00d4ff;
const ACCENT_RED: u32 = 0xe94560;
const TEXT_PRIMARY: u32 = 0xffffff;
const TEXT_SECONDARY: u32 = 0xaaaaaa;
const TEXT_MUTED: u32 = 0x666666;
const BORDER: u32 = 0x2a2a4a;
const SIDEBAR_BG: u32 = 0x12122a;
const STATUS_BG: u32 = 0x0d0d1a;
const CARD_BG: u32 = 0x1e1e3a;

// ─── Navigation pages ───
#[derive(Clone, Copy, PartialEq)]
enum Page {
    Home,
    Devices,
    Firmware,
    Settings,
}

impl Page {
    fn label(&self) -> &'static str {
        match self {
            Page::Home => "Home",
            Page::Devices => "Devices",
            Page::Firmware => "Firmware",
            Page::Settings => "Settings",
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            Page::Home => "⚡",
            Page::Devices => "📱",
            Page::Firmware => "💾",
            Page::Settings => "⚙️",
        }
    }
}

// ─── Device info for the device list ───
struct DeviceInfo {
    name: &'static str,
    mcu: &'static str,
    status: &'static str,
    status_color: u32,
}

const DEVICES: &[DeviceInfo] = &[
    DeviceInfo {
        name: "T-Display S3",
        mcu: "ESP32-S3",
        status: "Connected",
        status_color: 0x00e676,
    },
    DeviceInfo {
        name: "T-Display S3 AMOLED",
        mcu: "ESP32-S3",
        status: "Disconnected",
        status_color: ACCENT_RED,
    },
    DeviceInfo {
        name: "T-Deck",
        mcu: "ESP32-S3",
        status: "Disconnected",
        status_color: ACCENT_RED,
    },
    DeviceInfo {
        name: "T-Watch S3",
        mcu: "ESP32-S3",
        status: "Disconnected",
        status_color: ACCENT_RED,
    },
];

// ─── App state ───
struct SparkApp {
    current_page: Page,
    flash_count: i32,
    sidebar_collapsed: bool,
}

impl SparkApp {
    fn new() -> Self {
        Self {
            current_page: Page::Home,
            flash_count: 0,
            sidebar_collapsed: false,
        }
    }

    fn navigate(&mut self, page: Page, cx: &mut Context<Self>) {
        self.current_page = page;
        cx.notify();
    }

    fn flash_firmware(&mut self, _: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        self.flash_count += 1;
        cx.notify();
    }

    fn toggle_sidebar(&mut self, _: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        self.sidebar_collapsed = !self.sidebar_collapsed;
        cx.notify();
    }

    // ─── Sidebar ───
    fn render_sidebar(&mut self, cx: &mut Context<Self>) -> Div {
        let current = self.current_page;
        let collapsed = self.sidebar_collapsed;
        let width = if collapsed { px(56.0) } else { px(200.0) };

        let mut sidebar = div()
            .w(width)
            .h_full()
            .flex()
            .flex_col()
            .bg(rgb(SIDEBAR_BG))
            .border_r_1()
            .border_color(rgb(BORDER));

        // Logo area
        sidebar = sidebar.child(
            div()
                .flex()
                .items_center()
                .justify_center()
                .h(px(56.0))
                .border_b_1()
                .border_color(rgb(BORDER))
                .child(
                    div()
                        .text_color(rgb(ACCENT))
                        .child(if collapsed { "⚡" } else { "⚡ Spark NT" }),
                ),
        );

        // Nav items
        for page in [Page::Home, Page::Devices, Page::Firmware, Page::Settings] {
            let is_active = current == page;
            let bg = if is_active { BG_PANEL } else { SIDEBAR_BG };
            let text_color = if is_active { ACCENT } else { TEXT_SECONDARY };
            let label = if collapsed {
                page.icon().to_string()
            } else {
                format!("{}  {}", page.icon(), page.label())
            };

            sidebar = sidebar.child(
                div()
                    .id(SharedString::from(format!("nav-{}", page.label())))
                    .mx(px(8.0))
                    .mt(px(4.0))
                    .px(px(12.0))
                    .py(px(10.0))
                    .rounded_md()
                    .bg(rgb(bg))
                    .hover(|s| s.bg(rgb(BG_SECONDARY)))
                    .cursor_pointer()
                    .text_color(rgb(text_color))
                    .child(label)
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.navigate(page, cx);
                    })),
            );
        }

        // Spacer
        sidebar = sidebar.child(div().flex_1());

        // Collapse toggle
        sidebar = sidebar.child(
            div()
                .id("toggle-sidebar")
                .flex()
                .justify_center()
                .py_3()
                .border_t_1()
                .border_color(rgb(BORDER))
                .cursor_pointer()
                .text_color(rgb(TEXT_MUTED))
                .hover(|s| s.text_color(rgb(TEXT_SECONDARY)))
                .child(if collapsed { "▶" } else { "◀ Collapse" })
                .on_click(cx.listener(Self::toggle_sidebar)),
        );

        sidebar
    }

    // ─── Status bar ───
    fn render_status_bar(&self) -> Div {
        div()
            .w_full()
            .h(px(28.0))
            .flex()
            .items_center()
            .justify_between()
            .px_4()
            .bg(rgb(STATUS_BG))
            .border_t_1()
            .border_color(rgb(BORDER))
            .child(
                div()
                    .flex()
                    .gap_4()
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x00e676))
                            .child("● 1 device connected"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(TEXT_MUTED))
                            .child(format!("Flashed: {} times", self.flash_count)),
                    ),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(TEXT_MUTED))
                    .child("LILYGO Spark NT v0.1.0 | GPUI"),
            )
    }

    // ─── Home page ───
    fn render_home(&self, cx: &mut Context<Self>) -> Div {
        div()
            .flex_1()
            .flex()
            .flex_col()
            .p_6()
            .gap_6()
            .overflow_hidden()
            // Welcome header
            .child(
                div().flex().flex_col().gap_2().child(
                    div()
                        .text_2xl()
                        .text_color(rgb(TEXT_PRIMARY))
                        .child("Welcome to LILYGO Spark NT"),
                ).child(
                    div()
                        .text_color(rgb(TEXT_SECONDARY))
                        .child("Flash firmware to your LILYGO devices with ease"),
                ),
            )
            // Stats cards row
            .child(
                div()
                    .flex()
                    .gap_4()
                    .child(Self::stat_card("Devices", "4", "Total supported", ACCENT))
                    .child(Self::stat_card("Connected", "1", "Ready to flash", 0x00e676))
                    .child(Self::stat_card(
                        "Firmwares",
                        "12",
                        "Available",
                        0xffa726,
                    ))
                    .child(Self::stat_card(
                        "Flashed",
                        &self.flash_count.to_string(),
                        "This session",
                        ACCENT_RED,
                    )),
            )
            // Quick action
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .p_5()
                    .rounded_lg()
                    .bg(rgb(CARD_BG))
                    .border_1()
                    .border_color(rgb(BORDER))
                    .child(
                        div()
                            .text_color(rgb(TEXT_PRIMARY))
                            .child("Quick Flash"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(TEXT_SECONDARY))
                            .child("T-Display S3 is connected. Flash the latest firmware?"),
                    )
                    .child(
                        div()
                            .id("flash-btn")
                            .mt_2()
                            .px_6()
                            .py(px(10.0))
                            .w(px(200.0))
                            .flex()
                            .justify_center()
                            .rounded_lg()
                            .bg(rgb(ACCENT_RED))
                            .hover(|s| s.bg(rgb(0xc53050)))
                            .active(|s| s.bg(rgb(0xa02040)))
                            .cursor_pointer()
                            .text_color(rgb(TEXT_PRIMARY))
                            .child("⚡ Flash Firmware")
                            .on_click(cx.listener(Self::flash_firmware)),
                    ),
            )
    }

    fn stat_card(title: &str, value: &str, subtitle: &str, accent: u32) -> Div {
        div()
            .flex_1()
            .flex()
            .flex_col()
            .gap_1()
            .p_4()
            .rounded_lg()
            .bg(rgb(CARD_BG))
            .border_1()
            .border_color(rgb(BORDER))
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(TEXT_MUTED))
                    .child(title.to_string()),
            )
            .child(
                div()
                    .text_2xl()
                    .text_color(rgb(accent))
                    .child(value.to_string()),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(TEXT_SECONDARY))
                    .child(subtitle.to_string()),
            )
    }

    // ─── Devices page ───
    fn render_devices(&self) -> Div {
        let mut page = div()
            .flex_1()
            .flex()
            .flex_col()
            .p_6()
            .gap_4()
            .overflow_hidden()
            .child(
                div()
                    .text_2xl()
                    .text_color(rgb(TEXT_PRIMARY))
                    .child("Devices"),
            )
            .child(
                div()
                    .text_color(rgb(TEXT_SECONDARY))
                    .child("Manage your LILYGO development boards"),
            );

        for device in DEVICES {
            page = page.child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .p_4()
                    .rounded_lg()
                    .bg(rgb(CARD_BG))
                    .border_1()
                    .border_color(rgb(BORDER))
                    .hover(|s| s.border_color(rgb(ACCENT)))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .text_color(rgb(TEXT_PRIMARY))
                                    .child(device.name.to_string()),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(TEXT_MUTED))
                                    .child(format!("MCU: {}", device.mcu)),
                            ),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(device.status_color))
                            .child(format!("● {}", device.status)),
                    ),
            );
        }

        page
    }

    // ─── Firmware page ───
    fn render_firmware(&self) -> Div {
        let firmwares = [
            ("Factory Test", "v1.0.0", "2024-12-01", "Official"),
            ("LVGL Demo", "v9.2.0", "2025-01-15", "Community"),
            ("MicroPython", "v1.23.0", "2025-02-20", "Official"),
            ("Arduino Blink", "v1.0.0", "2024-11-10", "Example"),
        ];

        let mut page = div()
            .flex_1()
            .flex()
            .flex_col()
            .p_6()
            .gap_4()
            .overflow_hidden()
            .child(
                div()
                    .text_2xl()
                    .text_color(rgb(TEXT_PRIMARY))
                    .child("Firmware Library"),
            )
            .child(
                div()
                    .text_color(rgb(TEXT_SECONDARY))
                    .child("Browse and flash available firmware images"),
            );

        for (name, version, date, source) in firmwares {
            let badge_color = match source {
                "Official" => ACCENT,
                "Community" => 0xffa726,
                _ => TEXT_MUTED,
            };

            page = page.child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .p_4()
                    .rounded_lg()
                    .bg(rgb(CARD_BG))
                    .border_1()
                    .border_color(rgb(BORDER))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .flex()
                                    .gap_3()
                                    .items_center()
                                    .child(
                                        div()
                                            .text_color(rgb(TEXT_PRIMARY))
                                            .child(name.to_string()),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .px_2()
                                            .py(px(2.0))
                                            .rounded_sm()
                                            .bg(rgb(BG_SECONDARY))
                                            .text_color(rgb(badge_color))
                                            .child(source.to_string()),
                                    ),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(TEXT_MUTED))
                                    .child(format!("{} · {}", version, date)),
                            ),
                    ),
            );
        }

        page
    }

    // ─── Settings page ───
    fn render_settings(&self) -> Div {
        let settings = [
            ("Auto-connect", "Automatically connect to devices on startup", true),
            ("Dark mode", "Use dark theme (currently the only theme)", true),
            ("Check updates", "Check for firmware updates on launch", false),
            ("Verbose logging", "Show detailed flash progress logs", false),
        ];

        let mut page = div()
            .flex_1()
            .flex()
            .flex_col()
            .p_6()
            .gap_4()
            .overflow_hidden()
            .child(
                div()
                    .text_2xl()
                    .text_color(rgb(TEXT_PRIMARY))
                    .child("Settings"),
            );

        for (name, desc, enabled) in settings {
            let indicator_color = if enabled { ACCENT } else { TEXT_MUTED };
            let indicator_text = if enabled { "ON" } else { "OFF" };

            page = page.child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .p_4()
                    .rounded_lg()
                    .bg(rgb(CARD_BG))
                    .border_1()
                    .border_color(rgb(BORDER))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .text_color(rgb(TEXT_PRIMARY))
                                    .child(name.to_string()),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(TEXT_MUTED))
                                    .child(desc.to_string()),
                            ),
                    )
                    .child(
                        div()
                            .px_3()
                            .py_1()
                            .rounded_md()
                            .text_sm()
                            .text_color(rgb(indicator_color))
                            .border_1()
                            .border_color(rgb(indicator_color))
                            .child(indicator_text.to_string()),
                    ),
            );
        }

        // About section
        page = page.child(
            div()
                .mt_4()
                .p_4()
                .rounded_lg()
                .bg(rgb(CARD_BG))
                .border_1()
                .border_color(rgb(BORDER))
                .flex()
                .flex_col()
                .gap_2()
                .child(
                    div()
                        .text_color(rgb(TEXT_PRIMARY))
                        .child("About"),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(rgb(TEXT_MUTED))
                        .child("LILYGO Spark NT v0.1.0"),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(rgb(TEXT_MUTED))
                        .child("Built with GPUI (Zed's UI Framework)"),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(rgb(ACCENT))
                        .child("github.com/eggfly/LILYGO-Spark-NT"),
                ),
        );

        page
    }
}

impl Render for SparkApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let content = match self.current_page {
            Page::Home => self.render_home(cx),
            Page::Devices => self.render_devices(),
            Page::Firmware => self.render_firmware(),
            Page::Settings => self.render_settings(),
        };

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(BG_PRIMARY))
            // Main area: sidebar + content
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_row()
                    .overflow_hidden()
                    .child(self.render_sidebar(cx))
                    .child(content),
            )
            // Status bar
            .child(self.render_status_bar())
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(960.0), px(640.0)), cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("LILYGO Spark NT".into()),
                    ..Default::default()
                }),
                window_min_size: Some(Size {
                    width: px(640.0),
                    height: px(400.0),
                }),
                ..Default::default()
            },
            |_window, cx| cx.new(|_| SparkApp::new()),
        )
        .unwrap();

        cx.activate(true);
    });
}
