use gpui::*;
use gpui::prelude::FluentBuilder;

use crate::app::{
    AccentColor, AccentMode, FlashCelebrationStyle, LinkOpenMode, SettingsTab, SparkApp,
    ThemePreference,
};
use crate::i18n::Language;
use crate::theme::*;

impl SparkApp {
    pub fn render_settings(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let t = &self.i18n;
        let active_tab = self.settings_tab;
        let primary = self.primary();

        div()
            .flex_1()
            .flex()
            .flex_col()
            .overflow_hidden()
            // Tab bar: Settings | Feedback
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .px_4()
                    .py_2()
                    .border_b_1()
                    .border_color(glass_border())
                    .child(Self::settings_tab_btn(
                        "⚙️",
                        t.t("settings.title"),
                        active_tab == SettingsTab::Settings,
                        cx,
                        SettingsTab::Settings,
                        primary,
                    ))
                    .child(Self::settings_tab_btn(
                        "💬",
                        t.t("settings.feedback"),
                        active_tab == SettingsTab::Feedback,
                        cx,
                        SettingsTab::Feedback,
                        primary,
                    )),
            )
            // Content
            .child(if active_tab == SettingsTab::Settings {
                self.render_settings_content(cx).into_any_element()
            } else {
                self.render_feedback_content().into_any_element()
            })
    }

    fn settings_tab_btn(
        icon: &str,
        label: &str,
        active: bool,
        cx: &mut Context<Self>,
        tab: SettingsTab,
        primary: u32,
    ) -> Stateful<Div> {
        let mut btn = div()
            .id(SharedString::from(format!("settings-tab-{:?}", tab)))
            .flex()
            .items_center()
            .gap_2()
            .px_4()
            .py(px(10.0))
            .rounded_xl()
            .text_sm()
            .cursor_pointer();

        if active {
            btn = btn
                .bg(hsla(270. / 360., 0.5, 0.5, 0.15))
                .text_color(rgb(primary));
        } else {
            btn = btn
                .text_color(rgb(TEXT_MUTED))
                .hover(|s| s.bg(hsla(0., 0., 0.5, 0.05)));
        }

        btn = btn.child(icon.to_string()).child(label.to_string());
        btn = btn.on_click(cx.listener(move |this, _, _, cx| {
            this.settings_tab = tab;
            cx.notify();
        }));

        btn
    }

    fn render_settings_content(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let t = &self.i18n;
        let primary = self.primary();

        let mut content = div()
            .max_w(px(680.0))
            .flex()
            .flex_col()
            .gap_4();

        // ── Language ──
        {
            let current_lang = self.i18n.language;
            let mut lang_btns = div().flex().items_center().gap_2();
            for lang in Language::ALL {
                let is_current = lang == current_lang;
                let mut btn = div()
                    .id(SharedString::from(format!("lang-{}", lang.code())))
                    .px_3()
                    .py(px(6.0))
                    .rounded_lg()
                    .text_sm()
                    .cursor_pointer();

                if is_current {
                    btn = btn.bg(rgb(self.primary())).text_color(rgb(0xffffff));
                } else {
                    btn = btn
                        .bg(hsla(0., 0., 0., 0.2))
                        .border_1()
                        .border_color(glass_border())
                        .text_color(rgb(TEXT_SECONDARY))
                        .hover(|s| s.bg(hsla(0., 0., 0., 0.3)));
                }

                btn = btn.child(lang.display_name().to_string());
                btn = btn.on_click(cx.listener(move |this, _, _, cx| {
                    this.set_language(lang, cx);
                }));
                lang_btns = lang_btns.child(btn);
            }

            content = content.child(Self::setting_row_custom(
                "🌐",
                t.t("settings.language"),
                &format!(
                    "Current: {} ({})",
                    current_lang.display_name(),
                    current_lang.code()
                ),
                lang_btns,
            ));
        }

        // ── Theme ──
        {
            let current = self.theme_preference;
            let options: [(ThemePreference, &str); 3] = [
                (ThemePreference::System, t.t("settings.theme_system")),
                (ThemePreference::Light, t.t("settings.theme_light")),
                (ThemePreference::Dark, t.t("settings.theme_dark")),
            ];
            let mut btns = div().flex().items_center().gap_1().p(px(2.0)).rounded_lg().bg(hsla(0., 0., 0., 0.15));
            for (pref, label) in options {
                let is_sel = current == pref;
                let mut btn = div()
                    .id(SharedString::from(format!("theme-{:?}", pref)))
                    .px_3()
                    .py(px(6.0))
                    .rounded_md()
                    .text_sm()
                    .cursor_pointer();
                if is_sel {
                    btn = btn.bg(rgb(self.primary())).text_color(rgb(0xffffff));
                } else {
                    btn = btn
                        .text_color(rgb(TEXT_MUTED))
                        .hover(|s| s.text_color(rgb(TEXT_PRIMARY)));
                }
                btn = btn.child(label.to_string());
                btn = btn.on_click(cx.listener(move |this, _, _, cx| {
                    this.theme_preference = pref;
                    cx.notify();
                }));
                btns = btns.child(btn);
            }
            content = content.child(Self::setting_row_custom(
                "🌙",
                t.t("settings.theme"),
                "",
                btns,
            ));
        }

        // ── Accent Color ──
        {
            let accent_mode = self.accent_mode;
            let accent_color = self.accent_color;

            // Mode toggle
            let mut mode_btns = div().flex().items_center().gap_1().p(px(2.0)).rounded_lg().bg(hsla(0., 0., 0., 0.15));
            let mode_options: [(AccentMode, &str); 2] = [
                (AccentMode::Rotating, t.t("settings.accent_rotating")),
                (AccentMode::Fixed, t.t("settings.accent_fixed")),
            ];
            for (mode, label) in mode_options {
                let is_sel = accent_mode == mode;
                let mut btn = div()
                    .id(SharedString::from(format!("accent-mode-{:?}", mode)))
                    .px_3()
                    .py(px(6.0))
                    .rounded_md()
                    .text_sm()
                    .cursor_pointer();
                if is_sel {
                    btn = btn.bg(rgb(self.primary())).text_color(rgb(0xffffff));
                } else {
                    btn = btn.text_color(rgb(TEXT_MUTED)).hover(|s| s.text_color(rgb(TEXT_PRIMARY)));
                }
                btn = btn.child(label.to_string());
                btn = btn.on_click(cx.listener(move |this, _, _, cx| {
                    this.accent_mode = mode;
                    cx.notify();
                }));
                mode_btns = mode_btns.child(btn);
            }

            let mut accent_section = div().flex().flex_col().gap_3().child(mode_btns);

            if accent_mode == AccentMode::Rotating {
                accent_section = accent_section.child(
                    div()
                        .text_xs()
                        .text_color(rgb(TEXT_MUTED))
                        .child(t.t("settings.accent_rotating_hint").to_string()),
                );
            } else {
                // Color picker
                let mut color_row = div().flex().gap_2();
                for color in AccentColor::ALL {
                    let is_sel = accent_color == color;
                    let mut dot = div()
                        .id(SharedString::from(format!("accent-{}", color.id())))
                        .w(px(28.0))
                        .h(px(28.0))
                        .rounded_full()
                        .bg(rgb(color.hex()))
                        .cursor_pointer()
                        .hover(|s| s.opacity(0.8));
                    if is_sel {
                        dot = dot.border_2().border_color(rgb(0xffffff));
                    }
                    dot = dot.on_click(cx.listener(move |this, _, _, cx| {
                        this.accent_color = color;
                        this.save_settings();
                        cx.notify();
                    }));
                    color_row = color_row.child(dot);
                }
                accent_section = accent_section.child(color_row);
            }

            content = content.child(Self::setting_row_custom(
                "🎨",
                t.t("settings.accent"),
                "",
                accent_section,
            ));
        }

        // ── Link Open Mode ──
        {
            let current = self.link_open_mode;
            let options: [(LinkOpenMode, &str); 2] = [
                (LinkOpenMode::Internal, t.t("settings.link_internal")),
                (LinkOpenMode::External, t.t("settings.link_external")),
            ];
            let mut btns = div().flex().items_center().gap_1().p(px(2.0)).rounded_lg().bg(hsla(0., 0., 0., 0.15));
            for (mode, label) in options {
                let is_sel = current == mode;
                let mut btn = div()
                    .id(SharedString::from(format!("link-{:?}", mode)))
                    .px_3()
                    .py(px(6.0))
                    .rounded_md()
                    .text_sm()
                    .cursor_pointer();
                if is_sel {
                    btn = btn.bg(rgb(self.primary())).text_color(rgb(0xffffff));
                } else {
                    btn = btn.text_color(rgb(TEXT_MUTED)).hover(|s| s.text_color(rgb(TEXT_PRIMARY)));
                }
                btn = btn.child(label.to_string());
                btn = btn.on_click(cx.listener(move |this, _, _, cx| {
                    this.link_open_mode = mode;
                    cx.notify();
                }));
                btns = btns.child(btn);
            }
            content = content.child(Self::setting_row_with_hint(
                "🔗",
                t.t("settings.link_open_mode"),
                t.t("settings.link_hint"),
                btns,
            ));
        }

        // ── Glass Effect ──
        {
            let enabled = self.glass_enabled;
            let toggle = Self::toggle_switch("glass-toggle", enabled, cx, |this, cx| {
                this.glass_enabled = !this.glass_enabled;
                this.save_settings();
                cx.notify();
            }, primary);
            let label = if enabled {
                t.t("settings.glass_on")
            } else {
                t.t("settings.glass_off")
            };
            let control = div()
                .flex()
                .items_center()
                .gap_3()
                .child(toggle)
                .child(div().text_sm().text_color(rgb(TEXT_SECONDARY)).child(label.to_string()));

            content = content.child(Self::setting_row_with_hint(
                "✨",
                t.t("settings.glass"),
                t.t("settings.glass_hint"),
                control,
            ));
        }

        // ── Sound ──
        {
            let enabled = self.sound_enabled;
            let toggle = Self::toggle_switch("sound-toggle", enabled, cx, |this, cx| {
                this.sound_enabled = !this.sound_enabled;
                this.save_settings();
                cx.notify();
            }, primary);
            let label = if enabled {
                t.t("settings.sound_on")
            } else {
                t.t("settings.sound_off")
            };
            let control = div()
                .flex()
                .items_center()
                .gap_3()
                .child(toggle)
                .child(div().text_sm().text_color(rgb(TEXT_SECONDARY)).child(label.to_string()));

            content = content.child(Self::setting_row_with_hint(
                "🔊",
                t.t("settings.sound"),
                t.t("settings.sound_hint"),
                control,
            ));
        }

        // ── Flash Celebration Style ──
        {
            let current = self.flash_celebration_style;
            let mut btns = div()
                .flex()
                .flex_wrap()
                .gap_1()
                .p(px(2.0))
                .rounded_lg()
                .bg(hsla(0., 0., 0., 0.15));
            for style in FlashCelebrationStyle::ALL {
                let is_sel = current == style;
                let mut btn = div()
                    .id(SharedString::from(format!("flash-{}", style.label())))
                    .px_3()
                    .py(px(6.0))
                    .rounded_md()
                    .text_xs()
                    .cursor_pointer();
                if is_sel {
                    btn = btn.bg(rgb(self.primary())).text_color(rgb(0xffffff));
                } else {
                    btn = btn
                        .text_color(rgb(TEXT_MUTED))
                        .hover(|s| s.text_color(rgb(TEXT_PRIMARY)));
                }
                btn = btn.child(style.label().to_string());
                btn = btn.on_click(cx.listener(move |this, _, _, cx| {
                    this.flash_celebration_style = style;
                    cx.notify();
                }));
                btns = btns.child(btn);
            }
            content = content.child(Self::setting_row_with_hint(
                "🎆",
                t.t("settings.flash_style"),
                t.t("settings.flash_style_hint"),
                btns,
            ));
        }

        // ── Easter Eggs ──
        content = content.child(
            glass_card_div()
                .p_4()
                .hover(|s| s.border_color(glass_border_hover()))
                .flex()
                .flex_col()
                .gap_3()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_3()
                        .child(div().child("⚡"))
                        .child(
                            div()
                                .text_sm()
                                .text_color(rgb(TEXT_PRIMARY))
                                .child(t.t("settings.easter_eggs").to_string()),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_2()
                        .text_sm()
                        .text_color(rgb(TEXT_MUTED))
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_2()
                                .child(
                                    div()
                                        .px(px(6.0))
                                        .py(px(2.0))
                                        .rounded_md()
                                        .bg(hsla(0., 0., 0., 0.2))
                                        .text_xs()
                                        .child("↑↑↓↓←→←→BA"),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .child(t.t("settings.easter_konami").to_string()),
                                ),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_2()
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(rgb(GREEN))
                                        .child(t.t("settings.easter_flash").to_string()),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .child(t.t("settings.easter_flash_hint").to_string()),
                                ),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_2()
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(rgb(GREEN))
                                        .child(t.t("settings.easter_device").to_string()),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .child(t.t("settings.easter_device_hint").to_string()),
                                ),
                        ),
                ),
        );

        // ── Check Update ──
        content = content.child(Self::setting_row_custom(
            "🔄",
            t.t("settings.check_update"),
            &format!("v{}", env!("CARGO_PKG_VERSION")),
            div()
                .id("check-update-btn")
                .px_4()
                .py(px(8.0))
                .rounded_lg()
                .bg(rgb(self.primary()))
                .text_sm()
                .text_color(rgb(0xffffff))
                .cursor_pointer()
                .hover(|s| s.opacity(0.85))
                .child(t.t("settings.check_now").to_string()),
        ));

        // ── Download Cache ──
        content = content.child(Self::setting_row_with_hint(
            "💾",
            t.t("settings.cache"),
            t.t("settings.cache_hint"),
            div()
                .flex()
                .items_center()
                .gap_3()
                .child(
                    div()
                        .text_xs()
                        .text_color(rgb(TEXT_MUTED))
                        .child(t.t("settings.cache_empty").to_string()),
                )
                .child(
                    div()
                        .id("cache-clear-btn")
                        .px_4()
                        .py(px(6.0))
                        .rounded_lg()
                        .bg(hsla(0., 0.7, 0.5, 0.1))
                        .text_sm()
                        .text_color(rgb(RED))
                        .cursor_pointer()
                        .hover(|s| s.bg(hsla(0., 0.7, 0.5, 0.2)))
                        .child(format!("🗑 {}", t.t("settings.cache_clear"))),
                ),
        ));

        // ── Advanced (collapsible) ──
        {
            let expanded = self.advanced_expanded;
            let chevron = if expanded { "▼" } else { "▶" };

            let mut advanced = div().flex().flex_col().gap_3();

            // Header (clickable to expand/collapse)
            advanced = advanced.child(
                div()
                    .id("advanced-toggle")
                    .flex()
                    .items_center()
                    .gap_2()
                    .px_4()
                    .py_3()
                    .rounded_xl()
                    .cursor_pointer()
                    .hover(|s| s.bg(hsla(0., 0., 0.5, 0.05)))
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.advanced_expanded = !this.advanced_expanded;
                        cx.notify();
                    }))
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(self.primary()))
                            .child(chevron.to_string()),
                    )
                    .child(div().child("🔧"))
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(TEXT_PRIMARY))
                            .child(t.t("settings.advanced").to_string()),
                    ),
            );

            if expanded {
                let mut advanced_items = div().pl(px(32.0)).flex().flex_col().gap_3();

                // Firmware Manifest
                advanced_items = advanced_items.child(
                    glass_card_div()
                        .p_4()
                        .hover(|s| s.border_color(glass_border_hover()))
                        .flex()
                        .flex_col()
                        .gap_2()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_2()
                                .child(div().text_color(rgb(self.primary())).child("📄"))
                                .child(
                                    div()
                                        .text_sm()
                                        .text_color(rgb(TEXT_PRIMARY))
                                        .child(t.t("settings.manifest_file").to_string()),
                                ),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(rgb(TEXT_MUTED))
                                .child(t.t("settings.manifest_hint").to_string()),
                        )
                        .child(
                            div()
                                .flex()
                                .gap_2()
                                .child(
                                    div()
                                        .id("manifest-select-btn")
                                        .flex()
                                        .items_center()
                                        .gap_2()
                                        .px_4()
                                        .py(px(6.0))
                                        .rounded_lg()
                                        .bg(hsla(270. / 360., 0.4, 0.5, 0.1))
                                        .text_sm()
                                        .text_color(rgb(self.primary()))
                                        .cursor_pointer()
                                        .hover(|s| s.bg(hsla(270. / 360., 0.4, 0.5, 0.2)))
                                        .child("📂")
                                        .child(t.t("settings.manifest_select").to_string()),
                                ),
                        ),
                );

                // Canary Channel
                {
                    let enabled = self.canary_update;
                    let toggle =
                        Self::toggle_switch("canary-toggle", enabled, cx, |this, cx| {
                            this.canary_update = !this.canary_update;
                            cx.notify();
                        }, primary);
                    advanced_items = advanced_items.child(
                        glass_card_div()
                            .p_4()
                            .hover(|s| s.border_color(glass_border_hover()))
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(2.0))
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap_2()
                                            .child(div().text_color(rgb(self.primary())).child("⚡"))
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(TEXT_PRIMARY))
                                                    .child(t.t("settings.canary").to_string()),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(TEXT_MUTED))
                                            .child(t.t("settings.canary_hint").to_string()),
                                    ),
                            )
                            .child(toggle),
                    );
                }

                // Developer Mode
                {
                    let enabled = self.developer_mode;
                    let toggle =
                        Self::toggle_switch("dev-mode-toggle", enabled, cx, |this, cx| {
                            this.developer_mode = !this.developer_mode;
                            this.save_settings();
                            cx.notify();
                        }, primary);
                    advanced_items = advanced_items.child(
                        glass_card_div()
                            .p_4()
                            .hover(|s| s.border_color(glass_border_hover()))
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(2.0))
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap_2()
                                            .child(div().text_color(rgb(self.primary())).child("⚡"))
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(TEXT_PRIMARY))
                                                    .child(
                                                        t.t("settings.developer_mode").to_string(),
                                                    ),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(TEXT_MUTED))
                                            .child(t.t("settings.developer_hint").to_string()),
                                    ),
                            )
                            .child(toggle),
                    );
                }

                advanced = advanced.child(advanced_items);
            }

            content = content.child(advanced);
        }

        div()
            .id("settings-page")
            .flex_1()
            .overflow_y_scroll()
            .p_6()
            .child(content)
    }

    fn render_feedback_content(&self) -> impl IntoElement {
        div()
            .flex_1()
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_4()
                    .child(div().text_3xl().child("💬"))
                    .child(
                        div()
                            .text_color(rgb(TEXT_PRIMARY))
                            .child(self.i18n.t("settings.feedback").to_string()),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(TEXT_MUTED))
                            .child("Coming soon..."),
                    ),
            )
    }

    // ── Reusable setting row builders ──

    fn setting_row_custom(icon: &str, name: &str, description: &str, control: impl IntoElement) -> Div {
        glass_card_div()
            .p_4()
            .hover(|s| s.border_color(glass_border_hover()))
            .flex()
            .items_center()
            .justify_between()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .child(div().child(icon.to_string()))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(2.0))
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(TEXT_PRIMARY))
                                    .child(name.to_string()),
                            )
                            .when(!description.is_empty(), |d: Div| {
                                d.child(
                                    div()
                                        .text_xs()
                                        .text_color(rgb(TEXT_MUTED))
                                        .child(description.to_string()),
                                )
                            }),
                    ),
            )
            .child(control.into_any_element())
    }

    fn setting_row_with_hint(
        icon: &str,
        name: &str,
        hint: &str,
        control: impl IntoElement,
    ) -> Div {
        glass_card_div()
            .p_4()
            .hover(|s| s.border_color(glass_border_hover()))
            .flex()
            .flex_col()
            .gap_3()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            .child(div().child(icon.to_string()))
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(TEXT_PRIMARY))
                                    .child(name.to_string()),
                            ),
                    )
                    .child(control.into_any_element()),
            )
            .when(!hint.is_empty(), |d: Div| {
                d.child(
                    div()
                        .text_xs()
                        .text_color(rgb(TEXT_MUTED))
                        .child(hint.to_string()),
                )
            })
    }

    fn toggle_switch(
        id: &'static str,
        enabled: bool,
        cx: &mut Context<Self>,
        on_click: fn(&mut Self, &mut Context<Self>),
        primary: u32,
    ) -> Stateful<Div> {
        let (track_bg, knob_offset) = if enabled {
            (rgb(primary), px(18.0))
        } else {
            (rgb(TEXT_MUTED), px(2.0))
        };

        div()
            .id(SharedString::from(id))
            .w(px(40.0))
            .h(px(22.0))
            .rounded_full()
            .bg(track_bg)
            .cursor_pointer()
            .child(
                div()
                    .w(px(18.0))
                    .h(px(18.0))
                    .mt(px(2.0))
                    .ml(knob_offset)
                    .rounded_full()
                    .bg(rgb(0xffffff))
                    .shadow_sm(),
            )
            .on_click(cx.listener(move |this, _, _, cx| {
                on_click(this, cx);
            }))
    }
}
