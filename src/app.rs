use gpui::*;

use crate::i18n::{I18n, Language};
use crate::manifest::{self, FlatProduct, FirmwareItem, Manifest};
use crate::pages::Page;
use crate::theme::*;

// Settings enums
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ThemePreference {
    System,
    Light,
    Dark,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AccentMode {
    Rotating,
    Fixed,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AccentColor {
    Blue,
    Orange,
    Amber,
    Emerald,
    Cyan,
    Sky,
    Violet,
    Rose,
}

impl AccentColor {
    pub const ALL: [AccentColor; 8] = [
        AccentColor::Blue, AccentColor::Orange, AccentColor::Amber, AccentColor::Emerald,
        AccentColor::Cyan, AccentColor::Sky, AccentColor::Violet, AccentColor::Rose,
    ];
    pub fn hex(&self) -> u32 {
        match self {
            AccentColor::Blue => 0x3b82f6,
            AccentColor::Orange => 0xf97316,
            AccentColor::Amber => 0xf59e0b,
            AccentColor::Emerald => 0x10b981,
            AccentColor::Cyan => 0x06b6d4,
            AccentColor::Sky => 0x0ea5e9,
            AccentColor::Violet => 0x8b5cf6,
            AccentColor::Rose => 0xf43f5e,
        }
    }
    pub fn id(&self) -> &'static str {
        match self {
            AccentColor::Blue => "blue", AccentColor::Orange => "orange",
            AccentColor::Amber => "amber", AccentColor::Emerald => "emerald",
            AccentColor::Cyan => "cyan", AccentColor::Sky => "sky",
            AccentColor::Violet => "violet", AccentColor::Rose => "rose",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum LinkOpenMode {
    Internal,
    External,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum FlashCelebrationStyle {
    Fireworks,
    Hacker,
    Minimal,
    Neon,
    Terminal,
    Gradient,
}

impl FlashCelebrationStyle {
    pub const ALL: [FlashCelebrationStyle; 6] = [
        FlashCelebrationStyle::Fireworks, FlashCelebrationStyle::Hacker,
        FlashCelebrationStyle::Minimal, FlashCelebrationStyle::Neon,
        FlashCelebrationStyle::Terminal, FlashCelebrationStyle::Gradient,
    ];
    pub fn label(&self) -> &'static str {
        match self {
            FlashCelebrationStyle::Fireworks => "Fireworks",
            FlashCelebrationStyle::Hacker => "Hacker",
            FlashCelebrationStyle::Minimal => "Minimal",
            FlashCelebrationStyle::Neon => "Neon",
            FlashCelebrationStyle::Terminal => "Terminal",
            FlashCelebrationStyle::Gradient => "Gradient",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SettingsTab {
    Settings,
    Feedback,
}

pub struct SparkApp {
    pub current_page: Page,
    pub manifest: Manifest,
    pub flat_products: Vec<FlatProduct>,
    pub selected_product_idx: Option<usize>,
    pub selected_firmwares: Vec<FirmwareItem>,
    pub search_query: String,
    pub only_with_firmware: bool,
    pub manifest_loading: bool,
    pub manifest_error: Option<String>,
    pub i18n: I18n,
    // Settings state
    pub settings_tab: SettingsTab,
    pub theme_preference: ThemePreference,
    pub accent_mode: AccentMode,
    pub accent_color: AccentColor,
    pub link_open_mode: LinkOpenMode,
    pub glass_enabled: bool,
    pub sound_enabled: bool,
    pub flash_celebration_style: FlashCelebrationStyle,
    pub advanced_expanded: bool,
    pub developer_mode: bool,
    pub canary_update: bool,
}

impl SparkApp {
    pub fn new() -> Self {
        Self {
            current_page: Page::FirmwareCenter,
            manifest: Manifest::default(),
            flat_products: Vec::new(),
            selected_product_idx: None,
            selected_firmwares: Vec::new(),
            search_query: String::new(),
            only_with_firmware: true,
            manifest_loading: true,
            manifest_error: None,
            i18n: I18n::new(Language::from_system()),
            settings_tab: SettingsTab::Settings,
            theme_preference: ThemePreference::System,
            accent_mode: AccentMode::Rotating,
            accent_color: AccentColor::Violet,
            link_open_mode: LinkOpenMode::Internal,
            glass_enabled: true,
            sound_enabled: true,
            flash_celebration_style: FlashCelebrationStyle::Fireworks,
            advanced_expanded: false,
            developer_mode: false,
            canary_update: false,
        }
    }

    pub fn navigate(&mut self, page: Page, cx: &mut Context<Self>) {
        self.current_page = page;
        cx.notify();
    }

    pub fn load_manifest(&mut self, cx: &mut Context<Self>) {
        self.manifest_loading = true;
        self.manifest_error = None;
        cx.notify();

        // Try to load from the sibling LILYGO-Spark project
        let manifest_paths = [
            "../LILYGO-Spark/firmware_manifest.json".to_string(),
            "firmware_manifest.json".to_string(),
        ];

        let mut loaded = false;
        for path in &manifest_paths {
            match manifest::load_manifest_from_file(path) {
                Ok(m) => {
                    log::info!("Loaded manifest from {} ({} product groups, {} firmwares)",
                        path, m.product_list.len(), m.firmware_list.len());
                    self.flat_products = m.flat_products();
                    log::info!("Flattened to {} products", self.flat_products.len());
                    self.manifest = m;
                    self.manifest_loading = false;

                    // Select first product
                    if !self.flat_products.is_empty() {
                        self.select_product(0);
                    }
                    loaded = true;
                    break;
                }
                Err(e) => {
                    log::warn!("Failed to load from {}: {}", path, e);
                }
            }
        }

        if !loaded {
            self.manifest_loading = false;
            self.manifest_error = Some("Could not find firmware_manifest.json".to_string());
            log::error!("Failed to load manifest from any path");
        }

        cx.notify();
    }

    pub fn select_product(&mut self, idx: usize) {
        self.selected_product_idx = Some(idx);
        if let Some(product) = self.flat_products.get(idx) {
            self.selected_firmwares = self.manifest.firmware_for_product(&product.product_id);
            log::info!("Selected product: {} ({} firmwares)", product.name, self.selected_firmwares.len());
        }
    }

    pub fn set_language(&mut self, language: Language, cx: &mut Context<Self>) {
        self.i18n.set_language(language);
        cx.notify();
    }

    pub fn filtered_products(&self) -> Vec<(usize, &FlatProduct)> {
        let q = self.search_query.to_lowercase();
        self.flat_products.iter().enumerate().filter(|(_, p)| {
            // Search filter
            let matches_search = q.is_empty()
                || p.name.to_lowercase().contains(&q)
                || p.mcu.to_lowercase().contains(&q)
                || p.series_name.as_ref().is_some_and(|s| s.to_lowercase().contains(&q));

            // Firmware filter
            let has_firmware = !self.only_with_firmware || {
                !p.bin_files.is_empty()
                    || self.manifest.firmware_list.iter().any(|f| f.supported_product_ids.contains(&p.product_id))
            };

            matches_search && has_firmware
        }).collect()
    }
}

impl Render for SparkApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let content: AnyElement = match self.current_page {
            Page::Discovery => self.render_discovery().into_any_element(),
            Page::FirmwareCenter => self.render_firmware_center(cx).into_any_element(),
            Page::FirmwareLab => self.render_firmware_lab().into_any_element(),
            Page::SerialTools => self.render_serial_tools().into_any_element(),
            Page::EmbeddedTools => self.render_embedded_tools().into_any_element(),
            Page::Community => self.render_community().into_any_element(),
            Page::SparkLab => self.render_spark_lab().into_any_element(),
            Page::Settings => self.render_settings(cx).into_any_element(),
        };

        let mut root = div()
            .size_full()
            .flex()
            .flex_col()
            .bg(linear_gradient(
                135.,
                linear_color_stop(hsla(240. / 360., 0.15, 0.10, 1.0), 0.),
                linear_color_stop(hsla(260. / 360., 0.12, 0.12, 1.0), 1.0),
            ));

        // On Windows, add custom window control buttons at the top
        #[cfg(target_os = "windows")]
        {
            root = root.child(self.render_windows_titlebar(cx));
        }

        root = root.child(
            div()
                .flex_1()
                .flex()
                .flex_row()
                .overflow_hidden()
                .child(self.render_sidebar(cx))
                .child(content),
        );

        root
    }
}

#[cfg(target_os = "windows")]
impl SparkApp {
    fn render_windows_titlebar(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("windows-titlebar")
            .w_full()
            .h(px(36.0))
            .flex()
            .items_center()
            .justify_between()
            .window_control_area(WindowControlArea::Drag)
            // Title on left
            .child(
                div()
                    .pl(px(12.0))
                    .text_xs()
                    .text_color(rgb(TEXT_MUTED))
                    .child("LILYGO Spark NT"),
            )
            // Window control buttons on right
            .child(
                div()
                    .flex()
                    .items_center()
                    .h_full()
                    // Minimize
                    .child(
                        div()
                            .id("win-minimize")
                            .w(px(46.0))
                            .h_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .cursor_pointer()
                            .hover(|s| s.bg(hsla(0., 0., 0.5, 0.1)))
                            .window_control_area(WindowControlArea::Min)
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(TEXT_MUTED))
                                    .child("─"),
                            ),
                    )
                    // Maximize
                    .child(
                        div()
                            .id("win-maximize")
                            .w(px(46.0))
                            .h_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .cursor_pointer()
                            .hover(|s| s.bg(hsla(0., 0., 0.5, 0.1)))
                            .window_control_area(WindowControlArea::Max)
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(TEXT_MUTED))
                                    .child("□"),
                            ),
                    )
                    // Close
                    .child(
                        div()
                            .id("win-close")
                            .w(px(46.0))
                            .h_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .cursor_pointer()
                            .hover(|s| s.bg(hsla(0., 0.7, 0.5, 0.8)))
                            .window_control_area(WindowControlArea::Close)
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(TEXT_MUTED))
                                    .child("✕"),
                            ),
                    ),
            )
    }
}
