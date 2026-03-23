use gpui::*;

use crate::app::SparkApp;
use crate::theme::*;

struct CommunityLink {
    icon: &'static str,
    name: &'static str,
    description: &'static str,
    url: &'static str,
    gradient_from: u32,
    gradient_to: u32,
}

const LINKS: &[CommunityLink] = &[
    CommunityLink { icon: "🌐", name: "Official Website", description: "Product catalog and news", url: "lilygo.cc", gradient_from: GREEN, gradient_to: 0x0d9488 },
    CommunityLink { icon: "🐙", name: "GitHub", description: "Source code and examples", url: "github.com/Xinyuan-LilyGO", gradient_from: 0x475569, gradient_to: 0x1e293b },
    CommunityLink { icon: "📖", name: "Wiki", description: "Documentation and tutorials", url: "wiki.lilygo.cc", gradient_from: AMBER, gradient_to: 0xea580c },
    CommunityLink { icon: "🛍", name: "Taobao Store", description: "Official Taobao shop", url: "shop140839766.taobao.com", gradient_from: 0xf97316, gradient_to: 0xe11d48 },
    CommunityLink { icon: "🏪", name: "AliExpress", description: "International store", url: "lilygo.aliexpress.com", gradient_from: RED, gradient_to: 0xe11d48 },
    CommunityLink { icon: "👥", name: "Community Forum", description: "Discussions and support", url: "community.lilygo.cc", gradient_from: 0x8b5cf6, gradient_to: 0x7c3aed },
];

impl SparkApp {
    pub fn render_community(&self) -> impl IntoElement {
        div()
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
                    .child(page_header_with_primary("👥", self.i18n.t("community.title"), self.i18n.t("community.subtitle"), self.primary()))
            )
            // Link grid
            .child(
                div()
                    .id("community-page")
                    .flex_1()
                    .overflow_y_scroll()
                    .p_6()
                    .child(
                        {
                            let mut grid = div()
                                .flex()
                                .flex_wrap()
                                .gap_4();

                            for link in LINKS {
                                grid = grid.child(Self::community_link_card(link));
                            }

                            grid
                        },
                    ),
            )
    }

    fn community_link_card(link: &CommunityLink) -> Div {
        glass_card_div()
            .w(px(300.0))
            .flex()
            .items_center()
            .gap_4()
            .p_4()
            .overflow_hidden()
            .cursor_pointer()
            .hover(|s| s.border_color(glass_border_hover()).shadow_xl())
            // Accent left bar
            .child(
                div()
                    .absolute()
                    .left_0()
                    .top_0()
                    .bottom_0()
                    .w(px(3.0))
                    .bg(linear_gradient(
                        180.,
                        linear_color_stop(rgb(link.gradient_from), 0.),
                        linear_color_stop(rgb(link.gradient_to), 1.),
                    )),
            )
            // Icon
            .child(
                div()
                    .w(px(48.0))
                    .h(px(48.0))
                    .rounded_xl()
                    .flex()
                    .items_center()
                    .justify_center()
                    .bg(linear_gradient(
                        135.,
                        linear_color_stop(rgb(link.gradient_from), 0.),
                        linear_color_stop(rgb(link.gradient_to), 1.),
                    ))
                    .shadow_md()
                    .child(
                        div().text_color(rgb(0xffffff)).child(link.icon.to_string()),
                    ),
            )
            // Text
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .gap(px(2.0))
                    .child(
                        div().text_sm().text_color(rgb(TEXT_PRIMARY)).child(link.name.to_string()),
                    )
                    .child(
                        div().text_xs().text_color(rgb(TEXT_SECONDARY)).child(link.description.to_string()),
                    )
                    .child(
                        div().text_xs().text_color(rgb(TEXT_MUTED)).child(link.url.to_string()),
                    ),
            )
            // Arrow
            .child(
                div().text_color(rgb(TEXT_MUTED)).child("↗"),
            )
    }
}
