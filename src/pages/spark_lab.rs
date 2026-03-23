use gpui::*;

use crate::app::SparkApp;
use crate::theme::*;

struct SparkItem {
    icon: &'static str,
    name: &'static str,
    description: &'static str,
    status: &'static str,
}

const FLASH_ITEMS: &[SparkItem] = &[
    SparkItem { icon: "⚡", name: "Web Serial Flash", description: "Flash firmware via Web Serial API", status: "shipped" },
    SparkItem { icon: "💾", name: "Firmware Dumper", description: "Read firmware from device", status: "shipped" },
    SparkItem { icon: "🔬", name: "Firmware Analyzer", description: "Analyze binary firmware files", status: "shipped" },
    SparkItem { icon: "📋", name: "Partition Editor", description: "Edit ESP32 partition tables", status: "shipped" },
    SparkItem { icon: "📦", name: "OTA Updates", description: "Over-the-air firmware updates", status: "planned" },
];

const DEVICE_ITEMS: &[SparkItem] = &[
    SparkItem { icon: "💻", name: "Serial Monitor", description: "Real-time serial communication", status: "shipped" },
    SparkItem { icon: "📊", name: "Serial Plotter", description: "Visualize serial data", status: "planned" },
    SparkItem { icon: "🔌", name: "Device Detection", description: "Auto-detect connected devices", status: "shipped" },
    SparkItem { icon: "📡", name: "BLE Scanner", description: "Scan BLE devices nearby", status: "spark" },
    SparkItem { icon: "🌐", name: "WiFi Provisioning", description: "Configure WiFi credentials", status: "spark" },
];

impl SparkApp {
    pub fn render_spark_lab(&self) -> impl IntoElement {
        let shipped = FLASH_ITEMS.iter().chain(DEVICE_ITEMS.iter()).filter(|i| i.status == "shipped").count();
        let planned = FLASH_ITEMS.iter().chain(DEVICE_ITEMS.iter()).filter(|i| i.status == "planned").count();
        let sparks = FLASH_ITEMS.iter().chain(DEVICE_ITEMS.iter()).filter(|i| i.status == "spark").count();
        let total = shipped + planned + sparks;
        let pct = if total > 0 { shipped * 100 / total } else { 0 };

        div()
            .flex_1()
            .flex()
            .flex_col()
            .overflow_hidden()
            // Tab header
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .px_4()
                    .py_2()
                    .border_b_1()
                    .border_color(glass_border())
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .px_4()
                            .py_2()
                            .rounded_lg()
                            .bg(hsla(270. / 360., 0.5, 0.5, 0.15))
                            .text_color(rgb(self.primary()))
                            .text_sm()
                            .cursor_pointer()
                            .child(format!("✨ {}", self.i18n.t("sparklab.sparkling_list"))),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .px_4()
                            .py_2()
                            .rounded_lg()
                            .text_color(rgb(TEXT_MUTED))
                            .text_sm()
                            .cursor_pointer()
                            .hover(|s| s.bg(hsla(0., 0., 0.5, 0.05)))
                            .child(format!("📖 {}", self.i18n.t("sparklab.guide"))),
                    ),
            )
            // Content
            .child(div()
            .flex_1()
            .flex()
            .flex_col()
            .p_6()
            .gap_6()
            .id("spark-lab-page")
            .overflow_y_scroll()
            .child(page_header_with_primary("🧪", self.i18n.t("sparklab.title"), self.i18n.t("sparklab.subtitle"), self.primary()))
            // Stats row
            .child(
                div()
                    .flex()
                    .gap_3()
                    .child(Self::status_pill("✅ Shipped", shipped, GREEN))
                    .child(Self::status_pill("🔜 Planned", planned, 0x3b82f6))
                    .child(Self::status_pill("💡 Sparks", sparks, AMBER)),
            )
            // Progress bar
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .h(px(6.0))
                            .w_full()
                            .rounded_full()
                            .bg(hsla(0., 0., 0., 0.2))
                            .child(
                                div()
                                    .h_full()
                                    .rounded_full()
                                    .bg(linear_gradient(
                                        90.,
                                        linear_color_stop(rgb(GREEN), 0.),
                                        linear_color_stop(rgb(0x059669), 1.),
                                    ))
                                    .w(px(pct as f32 / 100.0 * 500.0)),
                            ),
                    )
                    .child(
                        div().text_xs().text_color(rgb(TEXT_MUTED)).child(format!("{}% complete ({}/{})", pct, shipped, total)),
                    ),
            )
            // Categories
            .child(Self::spark_category("🔧 Flash & Firmware Management", FLASH_ITEMS))
            .child(Self::spark_category("🔌 Device Interaction", DEVICE_ITEMS)),
            )
    }

    fn status_pill(label: &str, count: usize, color: u32) -> Div {
        div()
            .flex()
            .items_center()
            .gap_2()
            .px_3()
            .py_1()
            .rounded_full()
            .bg(hsla(0., 0., 0., 0.2))
            .text_sm()
            .text_color(rgb(color))
            .child(format!("{} {}", label, count))
    }

    fn spark_category(title: &str, items: &[SparkItem]) -> Div {
        let mut card = glass_card_div()
            .flex()
            .flex_col()
            .p_4()
            .gap_3();

        card = card.child(
            div().text_color(rgb(TEXT_PRIMARY)).child(title.to_string()),
        );

        for item in items {
            let (status_text, status_color) = match item.status {
                "shipped" => ("✅ Shipped", GREEN),
                "planned" => ("🔜 Planned", 0x3b82f6),
                _ => ("💡 Spark", AMBER),
            };

            card = card.child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .py_2()
                    .child(
                        div()
                            .w(px(36.0))
                            .h(px(36.0))
                            .rounded_lg()
                            .bg(hsla(270. / 360., 0.3, 0.3, 0.2))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(item.icon.to_string()),
                    )
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .child(
                                div().text_sm().text_color(rgb(TEXT_PRIMARY)).child(item.name.to_string()),
                            )
                            .child(
                                div().text_xs().text_color(rgb(TEXT_MUTED)).child(item.description.to_string()),
                            ),
                    )
                    .child(
                        div()
                            .text_xs()
                            .px_2()
                            .py(px(2.0))
                            .rounded_md()
                            .text_color(rgb(status_color))
                            .child(status_text.to_string()),
                    ),
            );
        }

        card
    }
}
