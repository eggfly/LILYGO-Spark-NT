use gpui::*;
use gpui::prelude::FluentBuilder;

use crate::app::SparkApp;
use crate::theme::*;

fn format_size(bytes: Option<u64>) -> String {
    match bytes {
        Some(b) if b >= 1024 * 1024 => format!("{:.1} MB", b as f64 / (1024.0 * 1024.0)),
        Some(b) if b >= 1024 => format!("{:.1} KB", b as f64 / 1024.0),
        Some(b) => format!("{} B", b),
        None => "—".to_string(),
    }
}

impl SparkApp {
    pub fn render_firmware_center(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let filtered = self.filtered_products();
        let product_count = filtered.len();
        let selected_idx = self.selected_product_idx;

        // Get selected product info
        let selected_name = selected_idx
            .and_then(|i| self.flat_products.get(i))
            .map(|p| p.name.clone())
            .unwrap_or_else(|| self.i18n.t("fc.select_device").to_string());
        let selected_desc = selected_idx
            .and_then(|i| self.flat_products.get(i))
            .map(|p| p.description.clone())
            .unwrap_or_default();
        let selected_mcu = selected_idx
            .and_then(|i| self.flat_products.get(i))
            .map(|p| p.mcu.clone())
            .unwrap_or_default();
        let firmware_count = self.selected_firmwares.len();

        // Build firmware items
        let mut firmware_list = div().px_6().pb_6().flex().flex_col().gap_3();
        for fw in &self.selected_firmwares {
            let (badge_color, badge_bg) = match fw.fw_type.as_str() {
                "factory" => (GREEN, hsla(150. / 360., 0.6, 0.4, 0.15)),
                "micropython" => (AMBER, hsla(40. / 360., 0.7, 0.5, 0.15)),
                "lvgl" => (PRIMARY, hsla(270. / 360., 0.5, 0.5, 0.15)),
                _ => (TEXT_MUTED, hsla(0., 0., 0.3, 0.15)),
            };
            let size_str = format_size(fw.size);
            let type_label = fw.fw_type.clone();
            let fw_name = fw.name.clone();
            let fw_version = fw.version.clone();
            let fw_filename = fw.filename.clone();

            firmware_list = firmware_list.child(
                glass_card_div()
                    .p_4()
                    .flex()
                    .items_center()
                    .justify_between()
                    .hover(|s| s.border_color(glass_border_hover()))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(div().text_color(rgb(TEXT_PRIMARY)).child(fw_name))
                                    .child(
                                        div()
                                            .text_xs()
                                            .px_2()
                                            .py(px(2.0))
                                            .rounded_md()
                                            .bg(badge_bg)
                                            .text_color(rgb(badge_color))
                                            .child(type_label),
                                    ),
                            )
                            .child(
                                div().text_xs().text_color(rgb(TEXT_MUTED)).child(
                                    format!("{} · {} · {}", fw_version, size_str, fw_filename),
                                ),
                            ),
                    )
                    .child(
                        div()
                            .px_4()
                            .py(px(6.0))
                            .rounded_lg()
                            .bg(hsla(220. / 360., 0.6, 0.5, 0.15))
                            .text_sm()
                            .text_color(rgb(0x3b82f6))
                            .cursor_pointer()
                            .hover(|s| s.bg(hsla(220. / 360., 0.6, 0.5, 0.25)))
                            .child(format!("⬇ {}", self.i18n.t("fc.download"))),
                    ),
            );
        }

        // Build product list
        let mut product_list_div = div()
            .id("product-list")
            .flex_1()
            .overflow_y_scroll()
            .p_2()
            .flex()
            .flex_col()
            .gap_1();

        if self.manifest_loading {
            // Loading skeleton
            for i in 0..6 {
                product_list_div = product_list_div.child(
                    div()
                        .id(SharedString::from(format!("skeleton-{}", i)))
                        .flex()
                        .items_center()
                        .gap_3()
                        .px_3()
                        .py(px(10.0))
                        .rounded_xl()
                        .child(
                            div()
                                .w(px(48.0))
                                .h(px(48.0))
                                .rounded_lg()
                                .bg(hsla(0., 0., 0., 0.15)),
                        )
                        .child(
                            div()
                                .flex_1()
                                .flex()
                                .flex_col()
                                .gap_2()
                                .child(
                                    div().h(px(14.0)).w(px(120.0)).rounded_sm().bg(hsla(0., 0., 0., 0.15)),
                                )
                                .child(
                                    div().h(px(10.0)).w(px(80.0)).rounded_sm().bg(hsla(0., 0., 0., 0.1)),
                                ),
                        ),
                );
            }
        } else {
            for (real_idx, product) in filtered {
                let is_selected = selected_idx == Some(real_idx);
                let idx = real_idx;

                let mut item = div()
                    .id(SharedString::from(format!("product-{}", idx)))
                    .flex()
                    .items_center()
                    .gap_3()
                    .px_3()
                    .py(px(10.0))
                    .rounded_xl()
                    .cursor_pointer()
                    .hover(|s| s.bg(hsla(0., 0., 0.5, 0.05)));

                if is_selected {
                    item = item
                        .bg(hsla(270. / 360., 0.4, 0.5, 0.10))
                        .border_1()
                        .border_color(hsla(270. / 360., 0.5, 0.5, 0.3))
                        .shadow_lg();
                }

                // Product image placeholder
                item = item.child(
                    div()
                        .w(px(48.0))
                        .h(px(48.0))
                        .rounded_lg()
                        .bg(rgb(0xffffff))
                        .flex()
                        .items_center()
                        .justify_center()
                        .flex_none()
                        .shadow_sm()
                        .child(div().text_color(rgb(TEXT_MUTED)).child("📱")),
                );

                let name = product.name.clone();
                let mcu = product.mcu.clone();
                let mcu_for_badge = mcu.clone();

                item = item.child(
                    div()
                        .flex_1()
                        .flex()
                        .flex_col()
                        .gap(px(2.0))
                        .child(
                            div()
                                .text_sm()
                                .text_color(rgb(TEXT_PRIMARY))
                                .when(is_selected, |d: Div| d.text_color(rgb(self.primary())))
                                .child(name),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_2()
                                .when(!mcu.is_empty(), |d: Div| {
                                    d.child(
                                        div()
                                            .text_xs()
                                            .px(px(6.0))
                                            .py(px(1.0))
                                            .rounded_sm()
                                            .bg(hsla(270. / 360., 0.3, 0.3, 0.2))
                                            .text_color(rgb(self.primary()))
                                            .child(mcu_for_badge),
                                    )
                                }),
                        ),
                );

                item = item.on_click(cx.listener(move |this, _, _, cx| {
                    this.select_product(idx);
                    cx.notify();
                }));

                product_list_div = product_list_div.child(item);
            }
        }

        div()
            .flex_1()
            .flex()
            .flex_row()
            .overflow_hidden()
            // Left panel - product list
            .child(
                div()
                    .w(px(300.0))
                    .h_full()
                    .flex()
                    .flex_col()
                    .border_r_1()
                    .border_color(glass_border())
                    .bg(hsla(220. / 360., 0.1, 0.08, 0.5))
                    .child(
                        // Search bar area
                        div()
                            .p(px(16.0))
                            .border_b_1()
                            .border_color(glass_border())
                            .flex()
                            .flex_col()
                            .gap_3()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .px_3()
                                    .py(px(8.0))
                                    .rounded_lg()
                                    .bg(hsla(0., 0., 0., 0.2))
                                    .border_1()
                                    .border_color(glass_border())
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(TEXT_MUTED))
                                            .child(format!("🔍 {}", self.i18n.t("fc.search"))),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        div()
                                            .w(px(14.0))
                                            .h(px(14.0))
                                            .rounded_sm()
                                            .bg(rgb(self.primary()))
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(rgb(0xffffff))
                                                    .child("✓"),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(TEXT_SECONDARY))
                                            .child(self.i18n.t("fc.only_with_firmware").to_string()),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(TEXT_MUTED))
                                            .child(format!("({} {})", product_count, self.i18n.t("fc.products"))),
                                    ),
                            ),
                    )
                    .child(product_list_div),
            )
            // Right panel - firmware list
            .child(
                div()
                    .id("firmware-list")
                    .flex_1()
                    .flex()
                    .flex_col()
                    .overflow_y_scroll()
                    // Product header
                    .child(
                        div()
                            .p_6()
                            .border_b_1()
                            .border_color(glass_border())
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_2()
                                            .child(
                                                div()
                                                    .text_2xl()
                                                    .text_color(rgb(TEXT_PRIMARY))
                                                    .child(selected_name),
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(TEXT_MUTED))
                                                    .child(selected_desc),
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .gap_2()
                                                    .mt_2()
                                                    .child(Self::header_action_btn("🐙", "GitHub"))
                                                    .child(Self::header_action_btn("🌐", "Product Page")),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .w(px(80.0))
                                            .h(px(80.0))
                                            .rounded_xl()
                                            .bg(rgb(0xffffff))
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .shadow_md()
                                            .child(div().text_2xl().child("📱")),
                                    ),
                            ),
                    )
                    // Available Firmware section title
                    .child(
                        div()
                            .px_6()
                            .pt_4()
                            .pb_2()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(TEXT_PRIMARY))
                                            .child(self.i18n.t("fc.available_firmware").to_string()),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .px(px(6.0))
                                            .py(px(2.0))
                                            .rounded_full()
                                            .bg(hsla(270. / 360., 0.3, 0.3, 0.2))
                                            .text_color(rgb(self.primary()))
                                            .child(format!("{}", firmware_count)),
                                    ),
                            ),
                    )
                    // Firmware items
                    .child(firmware_list),
            )
    }

    fn header_action_btn(icon: &str, label: &str) -> Div {
        div()
            .flex()
            .items_center()
            .gap_1()
            .px_2()
            .py(px(4.0))
            .rounded_lg()
            .bg(hsla(0., 0., 0., 0.15))
            .border_1()
            .border_color(glass_border())
            .text_xs()
            .text_color(rgb(TEXT_SECONDARY))
            .cursor_pointer()
            .hover(|s| s.bg(hsla(0., 0., 0., 0.25)).text_color(rgb(TEXT_PRIMARY)))
            .child(icon.to_string())
            .child(label.to_string())
    }
}
