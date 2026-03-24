use gpui::*;

use crate::app::SparkApp;
use crate::theme::*;

impl SparkApp {
    pub fn render_firmware_lab(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let t_burner = self.i18n.t("lab.burner").to_string();
        let t_dumper = self.i18n.t("lab.dumper").to_string();
        let t_analyzer = self.i18n.t("lab.analyzer").to_string();
        let t_partition = self.i18n.t("lab.partition_editor").to_string();
        let t_start_flash = self.i18n.t("lab.start_flash").to_string();
        let t_ready = self.i18n.t("lab.ready").to_string();
        let t_drop_file = self.i18n.t("lab.drop_file").to_string();
        let active_tab = self.active_lab_tab;
        let primary = self.primary();

        let tabs = [
            ("⚡", t_burner),
            ("⬇", t_dumper),
            ("🔬", t_analyzer),
            ("📋", t_partition),
        ];

        let mut tab_bar = div()
            .flex()
            .items_center()
            .gap_1()
            .px_4()
            .py_2()
            .border_b_1()
            .border_color(glass_border());

        for (i, (icon, label)) in tabs.iter().enumerate() {
            let is_active = active_tab == i;
            let mut tab = div()
                .id(SharedString::from(format!("lab-tab-{}", i)))
                .flex()
                .items_center()
                .gap_2()
                .px_4()
                .py_2()
                .rounded_lg()
                .cursor_pointer()
                .text_sm();

            if is_active {
                tab = tab
                    .bg(self.primary_alpha(0.15))
                    .text_color(rgb(primary));
            } else {
                tab = tab
                    .text_color(rgb(TEXT_MUTED))
                    .hover(|s| s.bg(hsla(0., 0., 0.5, 0.05)));
            }

            tab = tab
                .child(icon.to_string())
                .child(label.clone())
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.active_lab_tab = i;
                    cx.notify();
                }));

            tab_bar = tab_bar.child(tab);
        }

        let content: AnyElement = match active_tab {
            0 => self.render_burner_tab(&t_start_flash, &t_ready, &t_drop_file),
            1 => self.render_dumper_tab(),
            2 => self.render_analyzer_tab(),
            3 => self.render_partition_tab(),
            _ => self.render_burner_tab(&t_start_flash, &t_ready, &t_drop_file),
        };

        div()
            .flex_1()
            .flex()
            .flex_col()
            .overflow_hidden()
            .child(tab_bar)
            .child(content)
    }

    fn render_burner_tab(&self, t_start_flash: &str, t_ready: &str, t_drop_file: &str) -> AnyElement {
        let primary = self.primary();
        div()
            .id("burner-content")
            .flex_1()
            .flex()
            .flex_col()
            .p_6()
            .gap_4()
            .overflow_y_scroll()
            // Mode toggle row
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            .p(px(2.0))
                            .rounded_lg()
                            .bg(hsla(0., 0., 0., 0.15))
                            .child(
                                div()
                                    .px_3()
                                    .py(px(6.0))
                                    .rounded_md()
                                    .bg(rgb(primary))
                                    .text_xs()
                                    .text_color(rgb(0xffffff))
                                    .cursor_pointer()
                                    .child("Basic"),
                            )
                            .child(
                                div()
                                    .px_3()
                                    .py(px(6.0))
                                    .rounded_md()
                                    .text_xs()
                                    .text_color(rgb(TEXT_MUTED))
                                    .cursor_pointer()
                                    .hover(|s| s.text_color(rgb(TEXT_PRIMARY)))
                                    .child("Advanced"),
                            ),
                    )
                    .child(
                        div()
                            .px_3()
                            .py(px(6.0))
                            .rounded_lg()
                            .bg(hsla(0., 0., 0., 0.15))
                            .border_1()
                            .border_color(glass_border())
                            .text_xs()
                            .text_color(rgb(TEXT_MUTED))
                            .cursor_pointer()
                            .hover(|s| s.text_color(rgb(TEXT_PRIMARY)))
                            .child("👁 Preview"),
                    ),
            )
            // Controls row - 4 columns matching Electron
            .child(
                div()
                    .flex()
                    .gap_3()
                    .child(Self::control_select("Port", "Select port...", "🔌"))
                    .child(Self::control_select("Tool", "esptool-js", "🛠"))
                    .child(Self::control_select("Chip", "Auto Detect", "💾"))
                    .child(Self::control_select("Baud", "921600", "📡")),
            )
            // File drop zone
            .child(
                glass_card_div()
                    .p_6()
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .gap_3()
                    .min_h(px(120.0))
                    .border_2()
                    .border_color(glass_border())
                    .cursor_pointer()
                    .hover(|s| s.border_color(glass_border_hover()))
                    .child(div().text_2xl().child("📂"))
                    .child(div().text_sm().text_color(rgb(TEXT_MUTED)).child(t_drop_file.to_string()))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(div().text_xs().text_color(rgb(TEXT_MUTED)).child("Supports: .bin firmware files"))
                            .child(
                                div()
                                    .text_xs()
                                    .px(px(6.0))
                                    .py(px(2.0))
                                    .rounded_sm()
                                    .bg(hsla(0., 0., 0., 0.2))
                                    .text_color(rgb(TEXT_MUTED))
                                    .child("or paste URL"),
                            ),
                    ),
            )
            // Status bar + Flash button
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_4()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .w(px(8.0))
                                    .h(px(8.0))
                                    .rounded_full()
                                    .bg(rgb(0xfbbf24)), // amber dot = idle
                            )
                            .child(
                                div().text_sm().text_color(rgb(TEXT_MUTED)).child(t_ready.to_string()),
                            ),
                    )
                    .child(div().flex_1())
                    // Progress bar placeholder
                    .child(
                        div()
                            .w(px(200.0))
                            .h(px(4.0))
                            .rounded_full()
                            .bg(hsla(0., 0., 0., 0.15))
                            .child(
                                div()
                                    .w(px(0.0))
                                    .h_full()
                                    .rounded_full()
                                    .bg(rgb(primary)),
                            ),
                    )
                    .child(
                        div()
                            .px_6()
                            .py(px(10.0))
                            .rounded_lg()
                            .bg(rgb(GREEN))
                            .text_color(rgb(0xffffff))
                            .cursor_pointer()
                            .hover(|s| s.opacity(0.85))
                            .child(format!("⚡ {}", t_start_flash)),
                    ),
            )
            // Terminal
            .child(
                div()
                    .flex_1()
                    .min_h(px(200.0))
                    .rounded_lg()
                    .bg(rgb(0x0a0a0f))
                    .border_1()
                    .border_color(hsla(0., 0., 0.2, 0.5))
                    .flex()
                    .flex_col()
                    // Terminal header bar
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .px_3()
                            .py(px(6.0))
                            .border_b_1()
                            .border_color(hsla(0., 0., 0.15, 0.5))
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(div().text_xs().text_color(rgb(0x22c55e)).child("●"))
                                    .child(div().text_xs().text_color(rgb(TEXT_MUTED)).child("CONSOLE Output")),
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(TEXT_MUTED))
                                            .cursor_pointer()
                                            .hover(|s| s.text_color(rgb(TEXT_PRIMARY)))
                                            .child("Auto-scroll"),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(TEXT_MUTED))
                                            .cursor_pointer()
                                            .hover(|s| s.text_color(rgb(TEXT_PRIMARY)))
                                            .child("Clear"),
                                    ),
                            ),
                    )
                    // Terminal content
                    .child(
                        div()
                            .flex_1()
                            .p_3()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x64748b))
                                    .flex()
                                    .flex_col()
                                    .gap(px(2.0))
                                    .child("$ Waiting for connection...")
                                    .child("  Select a port and firmware file to begin flashing."),
                            ),
                    ),
            )
            .into_any_element()
    }

    fn render_dumper_tab(&self) -> AnyElement {
        let primary = self.primary();
        div()
            .id("dumper-content")
            .flex_1()
            .flex()
            .flex_col()
            .overflow_hidden()
            .child(
                // Two-panel layout (matching Electron)
                div()
                    .id("dumper-panels")
                    .flex_1()
                    .flex()
                    .flex_row()
                    .p_6()
                    .gap_4()
                    .overflow_y_scroll()
                    // Left panel: Device Info
                    .child(
                        div()
                            .w(px(380.0))
                            .flex()
                            .flex_col()
                            .gap_4()
                            // Device Info card
                            .child(
                                glass_card_div()
                                    .p_5()
                                    .flex()
                                    .flex_col()
                                    .gap_4()
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .justify_between()
                                            .child(
                                                div()
                                                    .flex()
                                                    .items_center()
                                                    .gap_2()
                                                    .child(div().text_color(rgb(TEXT_PRIMARY)).child("Device Info"))
                                                    .child(
                                                        div()
                                                            .w(px(8.0))
                                                            .h(px(8.0))
                                                            .rounded_full()
                                                            .bg(rgb(0xef4444)), // red = disconnected
                                                    ),
                                            )
                                            .child(
                                                div()
                                                    .px_3()
                                                    .py(px(6.0))
                                                    .rounded_lg()
                                                    .bg(rgb(primary))
                                                    .text_xs()
                                                    .text_color(rgb(0xffffff))
                                                    .cursor_pointer()
                                                    .hover(|s| s.opacity(0.85))
                                                    .child("🔍 Detect Info"),
                                            ),
                                    )
                                    // Port selection
                                    .child(Self::control_select("Port", "Select port...", "🔌"))
                                    // Info grid
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_2()
                                            .child(Self::info_row("Detected Chip", "—"))
                                            .child(Self::info_row("Flash ID", "—"))
                                            .child(Self::info_row("Size", "—"))
                                            .child(Self::info_row("MAC Address", "—"))
                                            .child(Self::info_row("Crystal", "—")),
                                    ),
                            ),
                    )
                    // Right panel: Parameters
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .gap_4()
                            // Parameters card
                            .child(
                                glass_card_div()
                                    .p_5()
                                    .flex()
                                    .flex_col()
                                    .gap_4()
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap_2()
                                            .child(div().child("⚙"))
                                            .child(div().text_color(rgb(TEXT_PRIMARY)).child("Parameters")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .gap_3()
                                            .child(Self::control_select("Start Address", "0x000000", "📍"))
                                            .child(Self::control_select("Size", "4 MB", "📏")),
                                    )
                                    .child(Self::control_select("Baud Rate", "460800 (Fast)", "📡"))
                                    // Dump button
                                    .child(
                                        div()
                                            .w_full()
                                            .px_6()
                                            .py(px(12.0))
                                            .rounded_lg()
                                            .bg(rgb(GREEN))
                                            .text_color(rgb(0xffffff))
                                            .cursor_pointer()
                                            .text_center()
                                            .hover(|s| s.opacity(0.85))
                                            .child("⬇ Dump Firmware"),
                                    ),
                            )
                            // Progress card (initially empty)
                            .child(
                                glass_card_div()
                                    .p_5()
                                    .flex()
                                    .flex_col()
                                    .gap_3()
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(TEXT_MUTED))
                                            .child("Progress"),
                                    )
                                    .child(
                                        div()
                                            .w_full()
                                            .h(px(6.0))
                                            .rounded_full()
                                            .bg(hsla(0., 0., 0., 0.15))
                                            .child(
                                                div()
                                                    .w(px(0.0))
                                                    .h_full()
                                                    .rounded_full()
                                                    .bg(rgb(primary)),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .justify_between()
                                            .child(div().text_xs().text_color(rgb(TEXT_MUTED)).child("0%"))
                                            .child(div().text_xs().text_color(rgb(TEXT_MUTED)).child("— kbit/s")),
                                    ),
                            ),
                    ),
            )
            // Console at bottom
            .child(self.render_console("Waiting for dump command..."))
            .into_any_element()
    }

    fn render_analyzer_tab(&self) -> AnyElement {
        let primary = self.primary();
        div()
            .id("analyzer-content")
            .flex_1()
            .flex()
            .flex_col()
            .p_6()
            .gap_4()
            .overflow_y_scroll()
            // Engine selector + file drop
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .child(Self::control_select("Analysis Engine", "JavaScript (Built-in)", "🔬"))
                    .child(
                        div()
                            .px(px(8.0))
                            .py(px(4.0))
                            .rounded_md()
                            .bg(hsla(142. / 360., 0.6, 0.3, 0.3))
                            .text_xs()
                            .text_color(rgb(0x22c55e))
                            .child("No deps required"),
                    ),
            )
            // Drop zone
            .child(
                glass_card_div()
                    .p_8()
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .gap_3()
                    .min_h(px(140.0))
                    .border_2()
                    .border_color(glass_border())
                    .cursor_pointer()
                    .hover(|s| s.border_color(glass_border_hover()))
                    .child(div().text_3xl().child("📄"))
                    .child(div().text_color(rgb(TEXT_PRIMARY)).child("Drop a .bin file to analyze"))
                    .child(div().text_sm().text_color(rgb(TEXT_MUTED)).child("or click to browse")),
            )
            // Analysis results (empty state with placeholder cards)
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_4()
                    // Chip & Image Type card
                    .child(
                        Self::analyzer_result_card("Chip & Image", "🔧", &[
                            ("Chip", "—"),
                            ("Image Type", "—"),
                            ("Flash Size", "—"),
                        ], primary)
                    )
                    // Framework card
                    .child(
                        Self::analyzer_result_card("Framework", "📦", &[
                            ("Framework", "—"),
                            ("Version", "—"),
                        ], primary)
                    )
                    // Build Info card
                    .child(
                        Self::analyzer_result_card("Build Info", "🏗", &[
                            ("Project", "—"),
                            ("IDF Version", "—"),
                            ("Compile Date", "—"),
                        ], primary)
                    )
                    // Extended Header card
                    .child(
                        Self::analyzer_result_card("Extended Header", "📋", &[
                            ("Entry Point", "—"),
                            ("Segments", "—"),
                            ("WP Pin", "—"),
                        ], primary)
                    ),
            )
            // Partition Table section
            .child(
                glass_card_div()
                    .p_4()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(div().child("📋"))
                                    .child(div().text_sm().text_color(rgb(TEXT_PRIMARY)).child("Partition Table")),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(TEXT_MUTED))
                                    .child("Load firmware to analyze"),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .text_xs()
                            .text_color(rgb(TEXT_MUTED))
                            .py(px(4.0))
                            .border_b_1()
                            .border_color(glass_border())
                            .child(div().w(px(100.0)).child("Name"))
                            .child(div().w(px(80.0)).child("Type"))
                            .child(div().w(px(100.0)).child("Offset"))
                            .child(div().w(px(80.0)).child("Size")),
                    )
                    .child(
                        div()
                            .py_4()
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(div().text_xs().text_color(rgb(TEXT_MUTED)).child("No firmware loaded")),
                    ),
            )
            // Analysis Log (collapsible)
            .child(
                glass_card_div()
                    .p_4()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(div().text_xs().child("▶"))
                            .child(div().text_sm().text_color(rgb(TEXT_MUTED)).child("Analysis Log")),
                    ),
            )
            .into_any_element()
    }

    fn render_partition_tab(&self) -> AnyElement {
        let primary = self.primary();
        div()
            .id("partition-content")
            .flex_1()
            .flex()
            .flex_col()
            .p_6()
            .gap_4()
            .overflow_y_scroll()
            // Header with import/export
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(div().child("📋"))
                            .child(div().text_color(rgb(TEXT_PRIMARY)).child("Partition Table Editor")),
                    )
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .child(
                                div()
                                    .px_4()
                                    .py(px(6.0))
                                    .rounded_lg()
                                    .bg(hsla(0., 0., 0., 0.15))
                                    .border_1()
                                    .border_color(glass_border())
                                    .text_sm()
                                    .text_color(rgb(TEXT_SECONDARY))
                                    .cursor_pointer()
                                    .hover(|s| s.bg(hsla(0., 0., 0., 0.25)))
                                    .child("📂 Import CSV"),
                            )
                            .child(
                                div()
                                    .px_4()
                                    .py(px(6.0))
                                    .rounded_lg()
                                    .bg(rgb(primary))
                                    .text_sm()
                                    .text_color(rgb(0xffffff))
                                    .cursor_pointer()
                                    .hover(|s| s.opacity(0.85))
                                    .child("💾 Export .bin"),
                            ),
                    ),
            )
            // Partition table
            .child(
                glass_card_div()
                    .p_4()
                    .flex()
                    .flex_col()
                    .gap_1()
                    // Header row
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .text_xs()
                            .text_color(rgb(TEXT_MUTED))
                            .pb_2()
                            .border_b_1()
                            .border_color(glass_border())
                            .child(div().w(px(120.0)).child("Name"))
                            .child(div().w(px(80.0)).child("Type"))
                            .child(div().w(px(80.0)).child("SubType"))
                            .child(div().w(px(100.0)).child("Offset"))
                            .child(div().w(px(100.0)).child("Size"))
                            .child(div().w(px(80.0)).child("Flags"))
                            .child(div().w(px(40.0))),
                    )
                    // Editable rows
                    .child(Self::partition_edit_row("nvs", "data", "nvs", "0x9000", "0x5000", ""))
                    .child(Self::partition_edit_row("otadata", "data", "ota", "0xe000", "0x2000", ""))
                    .child(Self::partition_edit_row("app0", "app", "ota_0", "0x10000", "0x140000", ""))
                    .child(Self::partition_edit_row("app1", "app", "ota_1", "0x150000", "0x140000", ""))
                    .child(Self::partition_edit_row("spiffs", "data", "spiffs", "0x290000", "0x160000", ""))
                    .child(Self::partition_edit_row("coredump", "data", "coredump", "0x3F0000", "0x10000", "")),
            )
            // Add partition button
            .child(
                div()
                    .w_full()
                    .py_3()
                    .rounded_lg()
                    .border_2()
                    .border_color(glass_border())
                    .flex()
                    .items_center()
                    .justify_center()
                    .gap_2()
                    .cursor_pointer()
                    .hover(|s| s.border_color(glass_border_hover()).bg(hsla(0., 0., 0., 0.05)))
                    .child(div().text_sm().text_color(rgb(TEXT_MUTED)).child("+ Add Partition")),
            )
            // Visual partition map
            .child(
                glass_card_div()
                    .p_4()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(div().text_sm().text_color(rgb(TEXT_PRIMARY)).child("Flash Memory Map"))
                    .child(
                        div()
                            .w_full()
                            .h(px(32.0))
                            .rounded_md()
                            .overflow_hidden()
                            .flex()
                            .child(div().w(px(20.0)).h_full().bg(rgb(0x6366f1)).child(
                                div().text_xs().text_color(rgb(0xffffff)).p(px(2.0)),
                            ))
                            .child(div().w(px(8.0)).h_full().bg(rgb(0xf59e0b)))
                            .child(div().flex_1().h_full().bg(rgb(0x22c55e)).flex().items_center().justify_center().child(
                                div().text_xs().text_color(rgb(0xffffff)).child("app0"),
                            ))
                            .child(div().flex_1().h_full().bg(rgb(0x3b82f6)).flex().items_center().justify_center().child(
                                div().text_xs().text_color(rgb(0xffffff)).child("app1"),
                            ))
                            .child(div().w(px(60.0)).h_full().bg(rgb(0xec4899)).flex().items_center().justify_center().child(
                                div().text_xs().text_color(rgb(0xffffff)).child("spiffs"),
                            ))
                            .child(div().w(px(20.0)).h_full().bg(rgb(0xef4444))),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .gap_3()
                            .child(Self::legend_item("nvs", 0x6366f1))
                            .child(Self::legend_item("otadata", 0xf59e0b))
                            .child(Self::legend_item("app0", 0x22c55e))
                            .child(Self::legend_item("app1", 0x3b82f6))
                            .child(Self::legend_item("spiffs", 0xec4899))
                            .child(Self::legend_item("coredump", 0xef4444)),
                    ),
            )
            .into_any_element()
    }

    // ── Helper widgets ──

    fn render_console(&self, placeholder: &str) -> Div {
        div()
            .h(px(180.0))
            .mx_6()
            .mb_4()
            .rounded_lg()
            .bg(rgb(0x0a0a0f))
            .border_1()
            .border_color(hsla(0., 0., 0.2, 0.5))
            .flex()
            .flex_col()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_3()
                    .py(px(6.0))
                    .border_b_1()
                    .border_color(hsla(0., 0., 0.15, 0.5))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(div().text_xs().text_color(rgb(0x22c55e)).child("●"))
                            .child(div().text_xs().text_color(rgb(TEXT_MUTED)).child("CONSOLE Output")),
                    )
                    .child(
                        div()
                            .flex()
                            .gap_3()
                            .child(div().text_xs().text_color(rgb(TEXT_MUTED)).cursor_pointer().child("Auto-scroll"))
                            .child(div().text_xs().text_color(rgb(TEXT_MUTED)).cursor_pointer().child("Copy"))
                            .child(div().text_xs().text_color(rgb(TEXT_MUTED)).cursor_pointer().child("Clear")),
                    ),
            )
            .child(
                div()
                    .flex_1()
                    .p_3()
                    .child(div().text_xs().text_color(rgb(0x64748b)).child(format!("$ {}", placeholder))),
            )
    }

    fn analyzer_result_card(title: &str, icon: &str, fields: &[(&str, &str)], _primary: u32) -> Div {
        let mut card = glass_card_div()
            .w(px(240.0))
            .p_4()
            .flex()
            .flex_col()
            .gap_3()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(div().child(icon.to_string()))
                    .child(div().text_sm().text_color(rgb(TEXT_PRIMARY)).child(title.to_string())),
            );

        for (label, value) in fields {
            card = card.child(
                div()
                    .flex()
                    .justify_between()
                    .child(div().text_xs().text_color(rgb(TEXT_MUTED)).child(label.to_string()))
                    .child(div().text_xs().text_color(rgb(TEXT_SECONDARY)).child(value.to_string())),
            );
        }

        card
    }

    fn info_row(label: &str, value: &str) -> Div {
        div()
            .flex()
            .justify_between()
            .items_center()
            .py(px(4.0))
            .border_b_1()
            .border_color(hsla(0., 0., 0., 0.05))
            .child(div().text_sm().text_color(rgb(TEXT_MUTED)).child(label.to_string()))
            .child(div().text_sm().text_color(rgb(TEXT_SECONDARY)).child(value.to_string()))
    }

    fn partition_edit_row(name: &str, ptype: &str, subtype: &str, offset: &str, size: &str, flags: &str) -> Div {
        div()
            .flex()
            .gap_2()
            .items_center()
            .py(px(6.0))
            .text_sm()
            .hover(|s| s.bg(hsla(0., 0., 0., 0.08)))
            .rounded_md()
            .child(
                div()
                    .w(px(120.0))
                    .px_2()
                    .py(px(4.0))
                    .rounded_md()
                    .bg(hsla(0., 0., 0., 0.1))
                    .text_color(rgb(TEXT_PRIMARY))
                    .child(name.to_string()),
            )
            .child(
                div()
                    .w(px(80.0))
                    .px_2()
                    .py(px(4.0))
                    .rounded_md()
                    .bg(hsla(0., 0., 0., 0.1))
                    .text_color(rgb(TEXT_SECONDARY))
                    .child(ptype.to_string()),
            )
            .child(
                div()
                    .w(px(80.0))
                    .px_2()
                    .py(px(4.0))
                    .rounded_md()
                    .bg(hsla(0., 0., 0., 0.1))
                    .text_color(rgb(TEXT_SECONDARY))
                    .child(subtype.to_string()),
            )
            .child(
                div()
                    .w(px(100.0))
                    .px_2()
                    .py(px(4.0))
                    .rounded_md()
                    .bg(hsla(0., 0., 0., 0.1))
                    .text_color(rgb(TEXT_SECONDARY))
                    .child(offset.to_string()),
            )
            .child(
                div()
                    .w(px(100.0))
                    .px_2()
                    .py(px(4.0))
                    .rounded_md()
                    .bg(hsla(0., 0., 0., 0.1))
                    .text_color(rgb(TEXT_SECONDARY))
                    .child(size.to_string()),
            )
            .child(
                div()
                    .w(px(80.0))
                    .text_color(rgb(TEXT_MUTED))
                    .child(if flags.is_empty() { "—".to_string() } else { flags.to_string() }),
            )
            .child(
                div()
                    .w(px(40.0))
                    .flex()
                    .justify_center()
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(TEXT_MUTED))
                            .cursor_pointer()
                            .hover(|s| s.text_color(rgb(0xef4444)))
                            .child("🗑"),
                    ),
            )
    }

    fn legend_item(label: &str, color: u32) -> Div {
        div()
            .flex()
            .items_center()
            .gap(px(4.0))
            .child(
                div()
                    .w(px(10.0))
                    .h(px(10.0))
                    .rounded_sm()
                    .bg(rgb(color)),
            )
            .child(div().text_xs().text_color(rgb(TEXT_MUTED)).child(label.to_string()))
    }

    fn info_chip(label: &str, value: &str) -> Div {
        div()
            .flex()
            .flex_col()
            .gap(px(2.0))
            .px_3()
            .py_2()
            .rounded_lg()
            .bg(hsla(0., 0., 0., 0.1))
            .child(div().text_xs().text_color(rgb(TEXT_MUTED)).child(label.to_string()))
            .child(div().text_sm().text_color(rgb(TEXT_SECONDARY)).child(value.to_string()))
    }

    fn control_select(label: &str, value: &str, icon: &str) -> Div {
        div()
            .flex_1()
            .flex()
            .flex_col()
            .gap_1()
            .child(
                div().text_xs().text_color(rgb(TEXT_MUTED)).child(label.to_string()),
            )
            .child(
                glass_card_div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .px_3()
                    .py_2()
                    .cursor_pointer()
                    .hover(|s| s.border_color(glass_border_hover()))
                    .child(div().text_sm().child(icon.to_string()))
                    .child(div().text_sm().text_color(rgb(TEXT_SECONDARY)).child(value.to_string()))
                    .child(div().flex_1())
                    .child(div().text_xs().text_color(rgb(TEXT_MUTED)).child("▼")),
            )
    }
}
