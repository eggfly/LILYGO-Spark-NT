use gpui::*;

// ─── Primary colors (matching Electron dark theme) ───
pub const PRIMARY: u32 = 0xa855f7; // purple-500 (dark mode primary)
pub const PRIMARY_HOVER: u32 = 0x9333ea;
pub const TEXT_PRIMARY: u32 = 0xeeeeee;
pub const TEXT_SECONDARY: u32 = 0xb0b0c0;
pub const TEXT_MUTED: u32 = 0x71717a; // zinc-500
pub const BG_BASE: u32 = 0x202225; // dark bg
pub const BG_SURFACE: u32 = 0x2f3136; // card bg
pub const BORDER_COLOR: u32 = 0x3f3f46; // zinc-700
pub const GREEN: u32 = 0x10b981; // emerald-500
pub const AMBER: u32 = 0xf59e0b;
pub const RED: u32 = 0xef4444;

// ─── Glass helpers ───
pub fn glass_sidebar() -> Hsla {
    hsla(240. / 360., 0.15, 0.10, 0.85)
}

pub fn glass_card() -> Hsla {
    hsla(240. / 360., 0.1, 0.18, 0.80)
}

pub fn glass_border() -> Hsla {
    hsla(240. / 360., 0.1, 0.3, 0.30)
}

pub fn glass_border_hover() -> Hsla {
    hsla(270. / 360., 0.6, 0.6, 0.4)
}

pub fn card_shadow() -> BoxShadow {
    BoxShadow {
        color: hsla(0., 0., 0., 0.3),
        offset: point(px(0.), px(2.)),
        blur_radius: px(12.),
        spread_radius: px(0.),
    }
}

pub fn glass_card_div() -> Div {
    div()
        .rounded_xl()
        .bg(glass_card())
        .border_1()
        .border_color(glass_border())
        .shadow(vec![card_shadow()])
}

pub fn page_header_with_primary(icon: &str, title: &str, subtitle: &str, primary: u32) -> Div {
    div()
        .flex()
        .flex_col()
        .gap_2()
        .pb_4()
        .border_b_1()
        .border_color(glass_border())
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
                            div()
                                .text_color(rgb(0xffffff))
                                .child(icon.to_string()),
                        ),
                )
                .child(
                    div()
                        .text_2xl()
                        .text_color(rgb(TEXT_PRIMARY))
                        .child(title.to_string()),
                ),
        )
        .child(
            div()
                .text_sm()
                .text_color(rgb(TEXT_MUTED))
                .child(subtitle.to_string()),
        )
}
