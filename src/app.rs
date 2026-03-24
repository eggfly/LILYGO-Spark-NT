use std::time::Duration;

use gpui::*;
use serde::{Deserialize, Serialize};

use crate::i18n::{I18n, Language};
use crate::manifest::{self, FlatProduct, FirmwareItem, Manifest};
use crate::pages::Page;

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

impl SparkApp {
    /// Returns the currently active accent/primary color hex value.
    /// This respects the user's accent_color setting instead of the hardcoded PRIMARY constant.
    pub fn primary(&self) -> u32 {
        self.accent_color.hex()
    }

    /// Returns the primary color as an Hsla with custom alpha, useful for bg tints.
    pub fn primary_alpha(&self, alpha: f32) -> Hsla {
        let c = self.accent_color.hex();
        let [_, r, g, b] = c.to_be_bytes();
        // Simple RGB to HSL-ish: just use rgba for tinting
        let r = r as f32 / 255.0;
        let g = g as f32 / 255.0;
        let b = b as f32 / 255.0;
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let l = (max + min) / 2.0;
        let s = if max == min {
            0.0
        } else if l < 0.5 {
            (max - min) / (max + min)
        } else {
            (max - min) / (2.0 - max - min)
        };
        let h = if max == min {
            0.0
        } else if max == r {
            ((g - b) / (max - min)) / 6.0
        } else if max == g {
            (2.0 + (b - r) / (max - min)) / 6.0
        } else {
            (4.0 + (r - g) / (max - min)) / 6.0
        };
        let h = if h < 0.0 { h + 1.0 } else { h };
        hsla(h, s, l, alpha)
    }
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
    // Firmware Lab state
    pub active_lab_tab: usize, // 0=Burner, 1=Dumper, 2=Analyzer, 3=Partition
    // Embedded Tools state
    pub active_tool_idx: usize, // 0-11 tool index
    // Discovery state
    pub news_items: Vec<NewsItem>,
    pub news_loading: bool,
    pub news_error: Option<String>,
    // Animation
    pub page_transition_id: usize,
}

#[derive(Clone)]
pub struct NewsItem {
    pub title: String,
    pub summary: String,
    pub source: String,
    pub url: String,
    pub date: String,
    pub tags: Vec<String>,
}

/// Persisted settings that survive app restart
#[derive(Serialize, Deserialize, Default)]
struct PersistedSettings {
    language: Option<String>,
    accent_color: Option<String>,
    glass_enabled: Option<bool>,
    sound_enabled: Option<bool>,
    developer_mode: Option<bool>,
}

impl PersistedSettings {
    fn config_path() -> std::path::PathBuf {
        let dir = dirs_next().unwrap_or_else(|| std::path::PathBuf::from("."));
        dir.join("settings.json")
    }

    fn load() -> Self {
        let path = Self::config_path();
        std::fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    fn save(&self) {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(&path, json);
        }
    }
}

fn dirs_next() -> Option<std::path::PathBuf> {
    #[cfg(target_os = "macos")]
    {
        std::env::var("HOME").ok().map(|h| {
            std::path::PathBuf::from(h)
                .join("Library")
                .join("Application Support")
                .join("LILYGO Spark NG")
        })
    }
    #[cfg(target_os = "windows")]
    {
        std::env::var("APPDATA")
            .ok()
            .map(|h| std::path::PathBuf::from(h).join("LILYGO Spark NG"))
    }
    #[cfg(target_os = "linux")]
    {
        std::env::var("HOME")
            .ok()
            .map(|h| std::path::PathBuf::from(h).join(".config").join("lilygo-spark-ng"))
    }
}

impl SparkApp {
    pub fn new() -> Self {
        let saved = PersistedSettings::load();

        let language = saved.language.as_deref()
            .and_then(|code| Language::ALL.iter().find(|l| l.code() == code).copied())
            .unwrap_or_else(Language::from_system);

        let accent_color = saved.accent_color.as_deref()
            .and_then(|id| AccentColor::ALL.iter().find(|c| c.id() == id).copied())
            .unwrap_or(AccentColor::Violet);

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
            i18n: I18n::new(language),
            settings_tab: SettingsTab::Settings,
            theme_preference: ThemePreference::System,
            accent_mode: AccentMode::Rotating,
            accent_color,
            link_open_mode: LinkOpenMode::Internal,
            glass_enabled: saved.glass_enabled.unwrap_or(true),
            sound_enabled: saved.sound_enabled.unwrap_or(true),
            flash_celebration_style: FlashCelebrationStyle::Fireworks,
            advanced_expanded: false,
            developer_mode: saved.developer_mode.unwrap_or(false),
            canary_update: false,
            active_lab_tab: 0,
            active_tool_idx: 0,
            news_items: Vec::new(),
            news_loading: false,
            news_error: None,
            page_transition_id: 0,
        }
    }

    pub fn navigate(&mut self, page: Page, cx: &mut Context<Self>) {
        self.current_page = page;
        self.page_transition_id += 1;
        cx.notify();
    }

    pub fn load_news(&mut self, cx: &mut Context<Self>) {
        self.news_loading = true;
        self.news_error = None;
        cx.notify();

        cx.spawn(async |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            let feeds = [
                ("Hackaday", "https://hackaday.com/category/esp32/feed/"),
                ("CNX Software", "https://www.cnx-software.com/feed/"),
                ("Adafruit", "https://blog.adafruit.com/feed/"),
            ];

            let client = reqwest::Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .unwrap();

            let mut all_items: Vec<NewsItem> = Vec::new();
            for (source, url) in &feeds {
                match client.get(*url).send().await {
                    Ok(resp) => {
                        if let Ok(text) = resp.text().await {
                            let items = parse_rss_items(&text, source);
                            all_items.extend(items.into_iter().take(5));
                        }
                    }
                    Err(e) => {
                        log::warn!("Failed to fetch {}: {}", source, e);
                    }
                }
            }

            all_items.sort_by(|a, b| b.date.cmp(&a.date));

            let _ = this.update(cx, |this, cx| {
                this.news_loading = false;
                if all_items.is_empty() {
                    this.news_error = Some("No news found".to_string());
                } else {
                    log::info!("Loaded {} news items", all_items.len());
                    this.news_items = all_items;
                }
                cx.notify();
            });
        }).detach();
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
        self.save_settings();
        cx.notify();
    }

    pub fn save_settings(&self) {
        let settings = PersistedSettings {
            language: Some(self.i18n.language.code().to_string()),
            accent_color: Some(self.accent_color.id().to_string()),
            glass_enabled: Some(self.glass_enabled),
            sound_enabled: Some(self.sound_enabled),
            developer_mode: Some(self.developer_mode),
        };
        settings.save();
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
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let page_content = match self.current_page {
            Page::Discovery => self.render_discovery(cx).into_any_element(),
            Page::FirmwareCenter => self.render_firmware_center(cx).into_any_element(),
            Page::FirmwareLab => self.render_firmware_lab(cx).into_any_element(),
            Page::SerialTools => self.render_serial_tools().into_any_element(),
            Page::EmbeddedTools => self.render_embedded_tools(cx).into_any_element(),
            Page::Community => self.render_community().into_any_element(),
            Page::SparkLab => self.render_spark_lab().into_any_element(),
            Page::Settings => self.render_settings(cx).into_any_element(),
        };

        // Wrap content with fade-in animation on page change
        let transition_id = self.page_transition_id;
        let content = div()
            .size_full()
            .child(page_content)
            .with_animation(
                SharedString::from(format!("page-fade-{}", transition_id)),
                Animation::new(Duration::from_millis(150)).with_easing(ease_in_out),
                |div, delta| div.opacity(delta),
            )
            .into_any_element();

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
            root = root.child(self.render_windows_titlebar(window, cx));
        }
        #[cfg(not(target_os = "windows"))]
        let _ = window;

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
    fn windows_icon_font() -> &'static str {
        use std::sync::OnceLock;
        static FONT: OnceLock<&'static str> = OnceLock::new();
        *FONT.get_or_init(|| {
            use windows::Wdk::System::SystemServices::RtlGetVersion;
            let mut version = unsafe { std::mem::zeroed() };
            let status = unsafe { RtlGetVersion(&mut version) };
            if status.is_ok() && version.dwBuildNumber >= 22000 {
                "Segoe Fluent Icons"
            } else {
                "Segoe MDL2 Assets"
            }
        })
    }

    fn render_windows_titlebar(&self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let is_maximized = window.is_maximized();
        let button_height = px(36.0);
        let icon_font = Self::windows_icon_font();

        let caption_button = |id: &'static str, icon: &'static str, control_area: WindowControlArea, is_close: bool| {
            let mut btn = div()
                .id(id)
                .w(px(36.0))
                .h(button_height)
                .flex()
                .items_center()
                .justify_center()
                .cursor_pointer()
                .occlude()
                .window_control_area(control_area);

            if is_close {
                btn = btn.hover(|s| {
                    s.bg(rgba(0xe81123ff))
                        .text_color(rgb(0xffffff))
                });
            } else {
                btn = btn.hover(|s| s.bg(hsla(0., 0., 0.5, 0.1)));
            }

            btn.child(
                div()
                    .font_family(icon_font)
                    .text_size(px(10.0))
                    .text_color(rgb(TEXT_SECONDARY))
                    .child(icon),
            )
        };

        let maximize_icon = if is_maximized { "\u{e923}" } else { "\u{e922}" };

        div()
            .id("windows-titlebar")
            .w_full()
            .h(button_height)
            .flex()
            .items_center()
            .justify_between()
            .window_control_area(WindowControlArea::Drag)
            .child(
                div()
                    .pl(px(12.0))
                    .text_xs()
                    .text_color(rgb(TEXT_MUTED))
                    .child("LILYGO Spark NG"),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .h_full()
                    .child(caption_button("win-minimize", "\u{e921}", WindowControlArea::Min, false))
                    .child(caption_button("win-maximize", maximize_icon, WindowControlArea::Max, false))
                    .child(caption_button("win-close", "\u{e8bb}", WindowControlArea::Close, true)),
            )
    }
}

/// Simple RSS XML parser - extracts items from RSS 2.0 feed XML
fn parse_rss_items(xml: &str, source: &str) -> Vec<NewsItem> {
    let mut items = Vec::new();
    // Simple tag-based parsing (no XML crate needed)
    for item_block in xml.split("<item>").skip(1) {
        let end = item_block.find("</item>").unwrap_or(item_block.len());
        let block = &item_block[..end];

        let title = extract_tag(block, "title").unwrap_or_default();
        let link = extract_tag(block, "link").unwrap_or_default();
        let desc = extract_tag(block, "description").unwrap_or_default();
        let pub_date = extract_tag(block, "pubDate").unwrap_or_default();

        // Extract categories as tags
        let mut tags = Vec::new();
        for cat_block in block.split("<category").skip(1) {
            if let Some(end) = cat_block.find("</category>") {
                let cat = &cat_block[..end];
                if let Some(start) = cat.find('>') {
                    let tag = &cat[start + 1..];
                    if !tag.is_empty() && tags.len() < 2 {
                        tags.push(tag.to_string());
                    }
                }
            }
        }

        // Clean HTML from description
        let summary = strip_html(&desc);
        let summary = if summary.len() > 150 {
            format!("{}...", &summary[..summary.char_indices().nth(150).map(|(i, _)| i).unwrap_or(summary.len())])
        } else {
            summary
        };

        // Parse date to short format
        let date = parse_rss_date(&pub_date);

        if !title.is_empty() {
            items.push(NewsItem {
                title: decode_html_entities(&title),
                summary,
                source: source.to_string(),
                url: link,
                date,
                tags,
            });
        }
    }
    items
}

fn extract_tag(xml: &str, tag: &str) -> Option<String> {
    let open = format!("<{}", tag);
    let close = format!("</{}>", tag);
    // Handle CDATA sections
    if let Some(start_pos) = xml.find(&open) {
        let after_open = &xml[start_pos..];
        if let Some(gt) = after_open.find('>') {
            let content_start = start_pos + gt + 1;
            if let Some(end_pos) = xml[content_start..].find(&close) {
                let content = &xml[content_start..content_start + end_pos];
                // Strip CDATA wrapper
                let content = content
                    .trim()
                    .strip_prefix("<![CDATA[")
                    .and_then(|s| s.strip_suffix("]]>"))
                    .unwrap_or(content);
                return Some(content.to_string());
            }
        }
    }
    None
}

fn strip_html(html: &str) -> String {
    let mut result = String::new();
    let mut in_tag = false;
    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(ch),
            _ => {}
        }
    }
    decode_html_entities(&result).trim().to_string()
}

fn decode_html_entities(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#039;", "'")
        .replace("&apos;", "'")
        .replace("&#8217;", "\u{2019}")
        .replace("&#8216;", "\u{2018}")
        .replace("&#8220;", "\u{201C}")
        .replace("&#8221;", "\u{201D}")
        .replace("&#8230;", "\u{2026}")
}

fn parse_rss_date(date_str: &str) -> String {
    // RFC 2822 format: "Mon, 20 Mar 2025 12:00:00 +0000"
    let parts: Vec<&str> = date_str.split_whitespace().collect();
    if parts.len() >= 4 {
        format!("{} {} {}", parts[1], parts[2], parts[3])
    } else {
        date_str.to_string()
    }
}
