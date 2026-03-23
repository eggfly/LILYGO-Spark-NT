use gpui::*;

use crate::app::SparkApp;
use crate::theme::*;

impl SparkApp {
    pub fn render_discovery(&self) -> impl IntoElement {
        let title = self.i18n.t("discovery.title").to_string();
        let subtitle = self.i18n.t("discovery.subtitle").to_string();
        div()
            .id("discovery-page")
            .flex_1()
            .flex()
            .flex_col()
            .overflow_hidden()
            // Header - matches Electron: title + refresh button
            .child(
                div()
                    .p_6()
                    .border_b_1()
                    .border_color(glass_border())
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_3()
                                    .child(
                                        div()
                                            .w(px(40.))
                                            .h(px(40.))
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .rounded_xl()
                                            .bg(linear_gradient(
                                                135.,
                                                linear_color_stop(rgb(self.primary()), 0.),
                                                linear_color_stop(rgb(0x7c3aed), 1.),
                                            ))
                                            .shadow(vec![BoxShadow {
                                                color: hsla(270. / 360., 0.7, 0.5, 0.3),
                                                offset: point(px(0.), px(2.)),
                                                blur_radius: px(8.),
                                                spread_radius: px(0.),
                                            }])
                                            .child(
                                                div().text_color(rgb(0xffffff)).child("📰"),
                                            ),
                                    )
                                    .child(
                                        div().text_2xl().text_color(rgb(TEXT_PRIMARY)).child(title),
                                    ),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(TEXT_MUTED))
                                    .child(subtitle),
                            ),
                    )
                    // Refresh button
                    .child(
                        div()
                            .id("refresh-btn")
                            .p_2()
                            .rounded_lg()
                            .cursor_pointer()
                            .text_color(rgb(TEXT_MUTED))
                            .hover(|s| s.bg(hsla(0., 0., 0.5, 0.1)).text_color(rgb(TEXT_PRIMARY)))
                            .child("🔄"),
                    ),
            )
            // Content Grid
            .child(
                div()
                    .id("discovery-scroll")
                    .flex_1()
                    .overflow_y_scroll()
                    .p_6()
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .gap_6()
                            .child(Self::news_card("Hackaday", "New ESP32-S3 Projects Roundup", "A collection of the latest projects using the ESP32-S3 chipset, featuring IoT, wearable, and robotics applications.", "2025-03-20", 0x1a1a2e, &["ESP32", "Projects"]))
                            .child(Self::news_card("CNX Software", "LILYGO T-Display S3 AMOLED Review", "Hands-on review of the T-Display S3 AMOLED board with 1.91-inch display, touch screen, and USB-C connectivity.", "2025-03-18", 0x1e40af, &["Review", "AMOLED"]))
                            .child(Self::news_card("Adafruit", "CircuitPython 9.0 Released", "Major update with new board support including ESP32-S3, improved USB host, and enhanced WiFi capabilities.", "2025-03-15", 0xbe185d, &["Python", "Release"]))
                            .child(Self::news_card("Reddit", "ESP32 Home Automation Guide", "Complete guide for building smart home automation system with ESP32, covering MQTT, sensors, and cloud integration.", "2025-03-12", 0xea580c, &["IoT", "Tutorial"]))
                            .child(Self::news_card("GitHub", "MicroPython v1.23 Highlights", "What's new in the latest MicroPython release: async improvements, new hardware drivers, and performance optimizations.", "2025-03-10", 0x374151, &["MicroPython"]))
                            .child(Self::news_card("Hackaday", "Building a LoRa Mesh Network", "Step by step guide to creating a long-range LoRa mesh network for remote IoT monitoring applications.", "2025-03-08", 0x1a1a2e, &["LoRa", "Mesh"])),
                    ),
            )
    }

    fn news_card(source: &str, title: &str, summary: &str, date: &str, source_color: u32, tags: &[&str]) -> Div {
        let mut card = glass_card_div()
            .w(px(320.0))
            .flex()
            .flex_col()
            .overflow_hidden()
            .cursor_pointer()
            .hover(|s| s.border_color(glass_border_hover()).shadow_xl())
            // Image placeholder area
            .child(
                div()
                    .h(px(180.0))
                    .w_full()
                    .bg(hsla(220. / 360., 0.1, 0.08, 0.8))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div().text_3xl().text_color(hsla(0., 0., 0.3, 0.5)).child("📰"),
                    )
                    // Source badge
                    .child(
                        div()
                            .absolute()
                            .top_3()
                            .left_3()
                            .px_2()
                            .py(px(4.0))
                            .rounded_lg()
                            .bg(rgb(source_color))
                            .text_xs()
                            .text_color(rgb(0xffffff))
                            .child(source.to_string()),
                    ),
            )
            // Content
            .child(
                div()
                    .p(px(20.0))
                    .flex_1()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_color(rgb(TEXT_PRIMARY))
                            .child(title.to_string()),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(TEXT_MUTED))
                            .child(summary.to_string()),
                    ),
            );

        // Footer with date and tags
        let mut footer = div()
            .px(px(20.0))
            .pb(px(16.0))
            .pt(px(12.0))
            .border_t_1()
            .border_color(glass_border())
            .flex()
            .items_center()
            .gap_3()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .text_xs()
                    .text_color(rgb(TEXT_MUTED))
                    .child("📅")
                    .child(date.to_string()),
            );

        for tag in tags {
            footer = footer.child(
                div()
                    .text_xs()
                    .px(px(6.0))
                    .py(px(2.0))
                    .rounded_sm()
                    .bg(hsla(0., 0., 0., 0.15))
                    .text_color(rgb(TEXT_MUTED))
                    .child(format!("#{}", tag)),
            );
        }

        // External link icon
        footer = footer.child(div().flex_1()).child(
            div().text_xs().text_color(rgb(TEXT_MUTED)).child("↗"),
        );

        card = card.child(footer);
        card
    }
}
