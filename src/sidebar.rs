use gpui::*;
use gpui::prelude::FluentBuilder;

use crate::app::SparkApp;
use crate::pages::Page;
use crate::theme::*;

impl SparkApp {
    pub fn render_sidebar(&mut self, cx: &mut Context<Self>) -> Div {
        let current = self.current_page;

        let mut sidebar = div()
            .w(px(220.0))
            .h_full()
            .flex_none()
            .flex()
            .flex_col()
            .bg(glass_sidebar())
            .border_r_1()
            .border_color(glass_border());

        // Logo header - matches Electron: gradient icon + title + subtitle
        sidebar = sidebar.child(
            div()
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .h(px(96.0))
                .border_b_1()
                .border_color(glass_border())
                .child(
                    div()
                        .flex()
                        .items_center()
                        .child(
                            div()
                                .w(px(40.0))
                                .h(px(40.0))
                                .flex()
                                .items_center()
                                .justify_center()
                                .rounded_xl()
                                .bg(linear_gradient(
                                    135.,
                                    linear_color_stop(rgb(PRIMARY), 0.),
                                    linear_color_stop(rgb(0x7c3aed), 1.),
                                ))
                                .shadow(vec![BoxShadow {
                                    color: hsla(270. / 360., 0.7, 0.5, 0.3),
                                    offset: point(px(0.), px(2.)),
                                    blur_radius: px(8.),
                                    spread_radius: px(0.),
                                }])
                                .child(
                                    div().text_color(rgb(0xffffff)).child("⚡"),
                                ),
                        )
                        .child(
                            div()
                                .ml_3()
                                .text_color(rgb(TEXT_PRIMARY))
                                .child("LILYGO Spark"),
                        ),
                )
                .child(
                    div()
                        .mt(px(6.0))
                        .text_xs()
                        .text_color(rgb(TEXT_MUTED))
                        .child("Made with 🤖 AI & ❤️ Love"),
                ),
        );

        // Nav items - 8 items matching Electron sidebar
        sidebar = sidebar.child(
            {
                let mut nav = div()
                    .id("sidebar-nav")
                    .flex_1()
                    .overflow_y_scroll()
                    .px(px(12.0))
                    .pt(px(12.0))
                    .flex()
                    .flex_col()
                    .gap(px(4.0));

                for page in Page::ALL {
                    let is_active = current == page;

                    let mut item = div()
                        .id(SharedString::from(format!("nav-{}", page.id())))
                        .w_full()
                        .flex()
                        .items_center()
                        .rounded_2xl()
                        .px_3()
                        .py_3()
                        .cursor_pointer()
                        .hover(|s| {
                            s.bg(hsla(0., 0., 0.5, 0.08))
                        });

                    if is_active {
                        item = item
                            .bg(hsla(270. / 360., 0.5, 0.5, 0.10))
                            .text_color(rgb(PRIMARY))
                            .shadow(vec![BoxShadow {
                                color: hsla(270. / 360., 0.7, 0.5, 0.15),
                                offset: point(px(0.), px(0.)),
                                blur_radius: px(15.),
                                spread_radius: px(-3.),
                            }]);
                    } else {
                        item = item.text_color(rgb(TEXT_MUTED));
                    }

                    // Left active indicator bar
                    item = item.child(
                        div()
                            .w(px(3.0))
                            .rounded_full()
                            .bg(rgb(PRIMARY))
                            .when(!is_active, |d: Div| d.h(px(0.)).opacity(0.))
                            .when(is_active, |d: Div| d.h(px(32.0)).opacity(1.))
                            .mr(px(-6.0)),
                    );

                    // Icon
                    item = item.child(
                        div()
                            .ml(px(10.0))
                            .child(page.icon().to_string()),
                    );

                    // Label
                    item = item.child(
                        div()
                            .ml_3()
                            .text_sm()
                            .child(page.label().to_string()),
                    );

                    item = item.on_click(cx.listener(move |this, _, _, cx| {
                        this.navigate(page, cx);
                    }));

                    nav = nav.child(item);
                }

                nav
            },
        );

        // Footer - login button + version (matching Electron)
        sidebar = sidebar.child(
            div()
                .p_3()
                .border_t_1()
                .border_color(glass_border())
                .flex()
                .flex_col()
                .gap(px(10.0))
                // GitHub login button placeholder
                .child(
                    div()
                        .id("github-login-btn")
                        .w_full()
                        .flex()
                        .items_center()
                        .justify_center()
                        .gap_2()
                        .px_3()
                        .py(px(10.0))
                        .rounded_xl()
                        .bg(hsla(0., 0., 0.3, 0.2))
                        .border_1()
                        .border_color(glass_border())
                        .cursor_pointer()
                        .hover(|s| s.bg(hsla(0., 0., 0.3, 0.3)))
                        .child(
                            div().text_sm().child("🐙"),
                        )
                        .child(
                            div()
                                .text_sm()
                                .text_color(rgb(TEXT_SECONDARY))
                                .child("Login with GitHub"),
                        ),
                )
                // Version
                .child(
                    div()
                        .flex()
                        .justify_center()
                        .child(
                            div()
                                .text_xs()
                                .text_color(rgb(TEXT_MUTED))
                                .child("v0.1.0"),
                        ),
                ),
        );

        sidebar
    }
}
