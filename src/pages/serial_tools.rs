use gpui::*;

use crate::app::SparkApp;
use crate::theme::*;

impl SparkApp {
    pub fn render_serial_tools(&self) -> impl IntoElement {
        let t_select_port = self.i18n.t("serial.select_port").to_string();
        let t_connect = self.i18n.t("serial.connect").to_string();
        let t_clear = self.i18n.t("serial.clear").to_string();
        let t_send = self.i18n.t("serial.send").to_string();
        let t_type_cmd = self.i18n.t("serial.type_command").to_string();
        div()
            .flex_1()
            .flex()
            .flex_col()
            .overflow_hidden()
            // Toolbar
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .px_4()
                    .py_2()
                    .border_b_1()
                    .border_color(glass_border())
                    // Port select
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .px_3()
                            .py(px(6.0))
                            .rounded_lg()
                            .bg(glass_card())
                            .border_1()
                            .border_color(glass_border())
                            .child(div().text_sm().child("🔌"))
                            .child(div().text_sm().text_color(rgb(TEXT_SECONDARY)).child(t_select_port))
                            .child(div().text_xs().text_color(rgb(TEXT_MUTED)).child("▼")),
                    )
                    // Refresh
                    .child(
                        div()
                            .px_2()
                            .py(px(6.0))
                            .rounded_lg()
                            .cursor_pointer()
                            .text_color(rgb(TEXT_MUTED))
                            .hover(|s| s.text_color(rgb(TEXT_PRIMARY)).bg(hsla(0., 0., 0.5, 0.1)))
                            .child("🔄"),
                    )
                    // Baud rate
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .px_3()
                            .py(px(6.0))
                            .rounded_lg()
                            .bg(glass_card())
                            .border_1()
                            .border_color(glass_border())
                            .child(div().text_sm().text_color(rgb(TEXT_SECONDARY)).child("115200"))
                            .child(div().text_xs().text_color(rgb(TEXT_MUTED)).child("▼")),
                    )
                    // Connect button
                    .child(
                        div()
                            .px_4()
                            .py(px(6.0))
                            .rounded_lg()
                            .bg(rgb(GREEN))
                            .text_sm()
                            .text_color(rgb(0xffffff))
                            .cursor_pointer()
                            .hover(|s| s.opacity(0.85))
                            .child(t_connect),
                    )
                    .child(div().flex_1())
                    // Clear
                    .child(
                        div()
                            .px_3()
                            .py(px(6.0))
                            .rounded_lg()
                            .cursor_pointer()
                            .text_sm()
                            .text_color(rgb(TEXT_MUTED))
                            .hover(|s| s.text_color(rgb(TEXT_PRIMARY)).bg(hsla(0., 0., 0.5, 0.1)))
                            .child(format!("🗑 {}", t_clear)),
                    )
                    // Auto-scroll
                    .child(
                        div()
                            .px_3()
                            .py(px(6.0))
                            .rounded_lg()
                            .text_sm()
                            .text_color(rgb(GREEN))
                            .child("↓ Auto"),
                    ),
            )
            // Terminal area
            .child(
                div()
                    .id("serial-terminal")
                    .flex_1()
                    .bg(rgb(0x0d1117))
                    .p_4()
                    .overflow_y_scroll()
                    .flex()
                    .flex_col()
                    .gap(px(2.0))
                    .child(
                        div()
                            .text_xs()
                            .text_color(hsla(140. / 360., 0.6, 0.6, 0.7))
                            .child("// Serial Monitor - Connect a device to start"),
                    )
                    .child(
                        div().text_xs().text_color(hsla(140. / 360., 0.6, 0.6, 0.4)).child("// Supported baud rates: 9600, 115200, 230400, 460800, 921600"),
                    )
                    .child(
                        div().text_xs().text_color(hsla(140. / 360., 0.6, 0.6, 0.4)).child("// Auto-detection for common errors: PSRAM, Brownout, Guru Meditation"),
                    )
                    .child(div().h(px(8.0)))
                    .child(
                        div()
                            .text_xs()
                            .text_color(hsla(0., 0., 0.5, 0.3))
                            .child("Waiting for connection..."),
                    ),
            )
            // Input bar
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .px_4()
                    .py_2()
                    .border_t_1()
                    .border_color(glass_border())
                    .child(
                        div()
                            .flex_1()
                            .px_3()
                            .py(px(6.0))
                            .rounded_lg()
                            .bg(hsla(0., 0., 0., 0.2))
                            .border_1()
                            .border_color(glass_border())
                            .child(
                                div().text_sm().text_color(rgb(TEXT_MUTED)).child(t_type_cmd),
                            ),
                    )
                    .child(
                        div()
                            .px_4()
                            .py(px(6.0))
                            .rounded_lg()
                            .bg(rgb(self.primary()))
                            .text_sm()
                            .text_color(rgb(0xffffff))
                            .cursor_pointer()
                            .hover(|s| s.opacity(0.85))
                            .child(t_send),
                    ),
            )
    }
}
