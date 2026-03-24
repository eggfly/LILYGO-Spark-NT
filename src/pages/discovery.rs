use gpui::*;

use crate::app::{NewsItem, SparkApp};
use crate::theme::*;

impl SparkApp {
    pub fn render_discovery(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let title = self.i18n.t("discovery.title").to_string();
        let subtitle = self.i18n.t("discovery.subtitle").to_string();
        let primary = self.primary();

        // Auto-load news on first visit
        if self.news_items.is_empty() && !self.news_loading && self.news_error.is_none() {
            self.load_news(cx);
        }

        let mut page = div()
            .id("discovery-page")
            .flex_1()
            .flex()
            .flex_col()
            .overflow_hidden()
            // Header
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
                                                linear_color_stop(rgb(primary), 0.),
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
                            .id("refresh-news-btn")
                            .px_3()
                            .py_2()
                            .rounded_lg()
                            .cursor_pointer()
                            .text_color(rgb(TEXT_MUTED))
                            .hover(|s| s.bg(hsla(0., 0., 0.5, 0.1)).text_color(rgb(TEXT_PRIMARY)))
                            .child("🔄 Refresh")
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.news_items.clear();
                                this.news_error = None;
                                this.load_news(cx);
                            })),
                    ),
            );

        // Content
        if self.news_loading {
            // Loading skeletons
            let mut grid = div().flex().flex_wrap().gap_6();
            for i in 0..6 {
                grid = grid.child(Self::skeleton_card(i));
            }
            page = page.child(
                div()
                    .id("discovery-loading")
                    .flex_1()
                    .overflow_y_scroll()
                    .p_6()
                    .child(grid),
            );
        } else if let Some(err) = &self.news_error {
            // Error state with retry
            let err_msg = err.clone();
            page = page.child(
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
                            .gap_3()
                            .child(div().text_2xl().child("⚠️"))
                            .child(div().text_color(rgb(0xef4444)).child(err_msg))
                            .child(
                                div()
                                    .id("retry-news")
                                    .px_4()
                                    .py_2()
                                    .rounded_lg()
                                    .bg(rgb(primary))
                                    .text_color(rgb(0xffffff))
                                    .cursor_pointer()
                                    .hover(|s| s.opacity(0.85))
                                    .child("Retry")
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.news_error = None;
                                        this.load_news(cx);
                                    })),
                            ),
                    ),
            );
        } else {
            // Real news items (or empty fallback)
            let items = &self.news_items;
            let mut grid = div().flex().flex_wrap().gap_6();
            for (idx, item) in items.iter().enumerate() {
                grid = grid.child(Self::dynamic_news_card(idx, item));
            }
            page = page.child(
                div()
                    .id("discovery-scroll")
                    .flex_1()
                    .overflow_y_scroll()
                    .p_6()
                    .child(grid),
            );
        }

        page
    }

    fn source_color(source: &str) -> u32 {
        match source {
            "Hackaday" => 0x1a1a2e,
            "CNX Software" => 0x38bdf8,
            "Adafruit" => 0xbe185d,
            "Reddit" => 0xea580c,
            "GitHub" => 0x374151,
            _ => 0x6366f1,
        }
    }

    fn dynamic_news_card(idx: usize, item: &NewsItem) -> Stateful<Div> {
        let source_color = Self::source_color(&item.source);
        let url = item.url.clone();
        let tags: Vec<String> = item.tags.clone();

        let mut card = glass_card_div()
            .id(SharedString::from(format!("news-{}", idx)))
            .w(px(320.0))
            .flex()
            .flex_col()
            .overflow_hidden()
            .cursor_pointer()
            .hover(|s| s.border_color(glass_border_hover()).shadow_xl())
            .on_click(move |_, _, cx| {
                let _ = open::that(&url);
                cx.stop_propagation();
            })
            // Image placeholder
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
                            .child(item.source.clone()),
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
                            .child(item.title.clone()),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(TEXT_MUTED))
                            .child(item.summary.clone()),
                    ),
            );

        // Footer
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
                    .child(item.date.clone()),
            );

        for tag in &tags {
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

        footer = footer.child(div().flex_1()).child(
            div().text_xs().text_color(rgb(TEXT_MUTED)).child("↗"),
        );

        card = card.child(footer);
        card
    }

    fn skeleton_card(idx: usize) -> Div {
        glass_card_div()
            .w(px(320.0))
            .flex()
            .flex_col()
            .overflow_hidden()
            .child(
                div()
                    .h(px(180.0))
                    .w_full()
                    .bg(hsla(220. / 360., 0.1, 0.08, 0.6)),
            )
            .child(
                div()
                    .p(px(20.0))
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(
                        div()
                            .h(px(16.0))
                            .w(px(200.0 + (idx as f32 * 20.0) % 60.0))
                            .rounded_md()
                            .bg(hsla(0., 0., 0.3, 0.2)),
                    )
                    .child(
                        div()
                            .h(px(12.0))
                            .w(px(260.0))
                            .rounded_md()
                            .bg(hsla(0., 0., 0.3, 0.15)),
                    )
                    .child(
                        div()
                            .h(px(12.0))
                            .w(px(180.0))
                            .rounded_md()
                            .bg(hsla(0., 0., 0.3, 0.1)),
                    ),
            )
    }
}
