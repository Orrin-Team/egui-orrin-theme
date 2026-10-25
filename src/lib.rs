//! The [Orrin](https://github.com/Orrin-Team/orrin) editor's look for egui: nine
//! base greys plus an accent, a few diagnostic hues, and the values derived from
//! them.
//!
//! A *theme* is exactly those nine colours — everything else in [`Visuals`] is
//! derived from them, which is what keeps a user theme to ten lines of TOML
//! instead of a copy of egui's style tree.
//!
//! ```no_run
//! # let ctx = egui::Context::default();
//! egui_orrin_theme::install_fonts(&ctx);
//! egui_orrin_theme::apply(&ctx, &egui_orrin_theme::Theme::default());
//! ```

use egui::{Color32, CornerRadius, Stroke, Visuals};
use serde::{Deserialize, Serialize};

#[cfg(feature = "icons")]
pub mod icons;
#[cfg(feature = "toml")]
mod set;

#[cfg(feature = "toml")]
pub use set::{LoadError, ThemeSet};

/// Saturated hue carries meaning here and nowhere else: a build state, a log
/// severity, a profiler lane. These are deliberately *not* part of a [`Theme`] —
/// green means a good build in every theme, and a palette that could recolour
/// them could break what they say.
pub const OK: Color32 = Color32::from_rgb(0x5F, 0xB5, 0x5A);
pub const PENDING: Color32 = Color32::from_rgb(0xE8, 0xB3, 0x4A);
pub const ERROR: Color32 = Color32::from_rgb(0xFF, 0x7B, 0x72);

/// Zebra wash for list rows. Far weaker than the faint fill, which at this
/// density reads as two separate lists rather than one striped one. Premultiplied
/// so it darkens whatever surface it lands on, in any theme.
pub const ROW_BAND: Color32 = Color32::from_rgba_premultiplied(5, 5, 5, 5);

/// The rim of anything floating: a faint light line, the edge of a pane of
/// glass, rather than a border drawn around it.
pub const GLASS_EDGE: Color32 = Color32::from_rgba_premultiplied(18, 18, 20, 18);

/// The height of every button, field and toggle.
pub const CONTROL_HEIGHT: f32 = 22.0;

pub const BUILT_IN: &str = "orrin-dark";

/// One palette. Serialised as a `*.toml` file, so every field is a plain
/// `[r, g, b]` triple.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub text: [u8; 3],
    pub panel: [u8; 3],
    pub window: [u8; 3],
    pub extreme: [u8; 3],
    pub faint: [u8; 3],
    pub widget: [u8; 3],
    pub widget_hover: [u8; 3],
    pub outline: [u8; 3],
    pub accent: [u8; 3],
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            name: BUILT_IN.to_owned(),
            text: [213, 215, 220],
            panel: [20, 21, 25],
            window: [26, 27, 32],
            extreme: [15, 16, 19],
            faint: [24, 25, 30],
            widget: [31, 33, 38],
            widget_hover: [42, 44, 51],
            outline: [35, 37, 43],
            accent: [77, 141, 255],
        }
    }
}

/// What the host can do that the palette cannot know about.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Options {
    /// How much of a window, menu or popup is its own colour.
    ///
    /// Opaque by default, because egui draws nothing behind a translucent
    /// window but the raw content, which reads as a smudge. Orrin blurs the
    /// scene behind its popups and runs at 0.66 — low enough that the frosting
    /// reads as glass; at 0.8 it read as a slightly odd grey. Only lower this
    /// if your renderer does the same.
    pub window_opacity: f32,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            window_opacity: 1.0,
        }
    }
}

fn rgb([r, g, b]: [u8; 3]) -> Color32 {
    Color32::from_rgb(r, g, b)
}

impl Theme {
    pub fn accent(&self) -> Color32 {
        rgb(self.accent)
    }

    /// Selected row / active tab.
    ///
    /// Opaque, and most of the way to the panel: a tint of the accent rather
    /// than the accent itself, which outshouted everything beside it.
    pub fn select_fill(&self) -> Color32 {
        rgb(self.panel).lerp_to_gamma(self.accent(), 0.32)
    }

    /// Pressed widget / held mode.
    pub fn widget_active(&self) -> Color32 {
        self.accent().linear_multiply(0.70)
    }

    /// Hover lifts in grey. The accent is kept for what is selected, focused
    /// or pressed; a blue ring on whatever the pointer crossed made every
    /// control look selected for an instant.
    pub fn hover_stroke(&self) -> Stroke {
        Stroke::new(
            1.0_f32,
            rgb(self.widget_hover).lerp_to_gamma(rgb(self.text), 0.18),
        )
    }

    /// The faint outline an idle control keeps, so an unchecked box or an
    /// empty field still has an edge against a surface of its own value.
    pub fn idle_stroke(&self) -> Stroke {
        Stroke::new(1.0_f32, rgb(self.widget_hover))
    }

    /// Menu and status bars: between the panel and the field wells, so the
    /// window's frame recedes behind the tools without reading as an input.
    /// Derived rather than a tenth theme colour, so every existing theme file
    /// still parses.
    pub fn bar_rgb(&self) -> [u8; 3] {
        let mix = |a: u8, b: u8| ((a as u16 + b as u16) / 2) as u8;
        [
            mix(self.panel[0], self.extreme[0]),
            mix(self.panel[1], self.extreme[1]),
            mix(self.panel[2], self.extreme[2]),
        ]
    }

    pub fn bar(&self) -> Color32 {
        rgb(self.bar_rgb())
    }

    pub fn visuals(&self) -> Visuals {
        self.visuals_with(&Options::default())
    }

    pub fn visuals_with(&self, options: &Options) -> Visuals {
        // Dark as the base even for a light theme: every value this system cares
        // about is overridden below, and the leftovers (shadows, code hues) read
        // better dark than egui's light defaults do against these fills.
        let mut v = Visuals::dark();

        v.override_text_color = Some(rgb(self.text));
        v.panel_fill = rgb(self.panel);
        v.window_fill = rgb(self.window).gamma_multiply(options.window_opacity);
        v.window_stroke = Stroke::new(1.0_f32, GLASS_EDGE);
        v.extreme_bg_color = rgb(self.extreme);
        v.faint_bg_color = rgb(self.faint);
        v.hyperlink_color = self.accent();

        v.window_corner_radius = CornerRadius::same(8);
        v.menu_corner_radius = CornerRadius::same(6);

        v.selection.bg_fill = self.select_fill();
        v.selection.stroke = Stroke::new(1.0_f32, self.accent());

        let radius = CornerRadius::same(5);

        // egui's per-state widget styling. Note `noninteractive` = panels/labels,
        // `inactive` = idle interactive controls (the two are easy to confuse).
        v.widgets.noninteractive.bg_fill = rgb(self.panel);
        v.widgets.noninteractive.weak_bg_fill = rgb(self.panel);
        // Every hairline inside a panel — separators, a well's edge, a section's
        // foot — at half the outline's step from the panel: there to divide,
        // not to be looked at.
        v.widgets.noninteractive.bg_stroke = Stroke::new(
            1.0_f32,
            rgb(self.panel).lerp_to_gamma(rgb(self.outline), 0.5),
        );
        v.widgets.noninteractive.corner_radius = radius;

        v.widgets.inactive.bg_fill = rgb(self.widget);
        v.widgets.inactive.weak_bg_fill = rgb(self.widget);
        v.widgets.inactive.bg_stroke = self.idle_stroke();
        v.widgets.inactive.corner_radius = radius;

        v.widgets.hovered.bg_fill = rgb(self.widget_hover);
        v.widgets.hovered.weak_bg_fill = rgb(self.widget_hover);
        v.widgets.hovered.bg_stroke = self.hover_stroke();
        v.widgets.hovered.corner_radius = radius;

        v.widgets.active.bg_fill = self.widget_active();
        v.widgets.active.weak_bg_fill = self.widget_active();
        v.widgets.active.bg_stroke = Stroke::new(1.0_f32, self.accent());
        v.widgets.active.corner_radius = radius;

        v.widgets.open.bg_fill = rgb(self.widget);
        v.widgets.open.weak_bg_fill = rgb(self.widget);
        v.widgets.open.corner_radius = radius;

        v
    }
}

/// Applies `theme` to every style variant, so it sticks regardless of the OS
/// preference.
pub fn apply(ctx: &egui::Context, theme: &Theme) {
    apply_with(ctx, theme, &Options::default());
}

pub fn apply_with(ctx: &egui::Context, theme: &Theme, options: &Options) {
    ctx.all_styles_mut(|style| {
        style.visuals = theme.visuals_with(options);
        style.spacing.item_spacing = egui::vec2(6.0, 4.0);
        style.spacing.button_padding = egui::vec2(8.0, 3.0);
        // Every control is one height, so a row of mixed widgets lines up and a
        // field is a comfortable target rather than egui's 18px default.
        style.spacing.interact_size.y = CONTROL_HEIGHT;
        // Bars float over the content, invisible until the pointer nears them
        // and thin when it does: a list that is not being scrolled should not
        // carry a track down its side.
        style.spacing.scroll = egui::style::ScrollStyle {
            bar_width: 6.0,
            ..egui::style::ScrollStyle::floating()
        };
        style.spacing.window_margin = egui::Margin::same(8);
        style.spacing.indent = INDENT;
        style.text_styles = text_styles();
        style.animation_time = COLLAPSE_SECONDS;
    });
}

/// Disclosure triangles, and nothing else — hover, press and selection are
/// instant fill swaps, because a frame-accurate instrument must not lag the
/// pointer. egui's default is 83ms; the design system asks for 120.
const COLLAPSE_SECONDS: f32 = 0.120;

/// Disclosure bodies indent by this much.
const INDENT: f32 = 14.0;

/// Four sizes, no more: weight and colour do the emphasis work, size never
/// does. Monospace sits a step below body because JetBrains Mono runs wider and
/// taller than the proportional face at the same nominal size.
fn text_styles() -> std::collections::BTreeMap<egui::TextStyle, egui::FontId> {
    use egui::{FontFamily::Monospace, FontFamily::Proportional, FontId, TextStyle};
    [
        (TextStyle::Small, FontId::new(10.5, Proportional)),
        (TextStyle::Body, FontId::new(12.5, Proportional)),
        (TextStyle::Button, FontId::new(12.5, Proportional)),
        (TextStyle::Monospace, FontId::new(11.5, Monospace)),
        (TextStyle::Heading, FontId::new(15.0, Proportional)),
    ]
    .into()
}

#[cfg(feature = "fonts")]
const MONO_FONT: &str = "JetBrainsMono";

/// Installs the theme's fonts on top of egui's defaults and sets them on `ctx`.
/// Use [`add_fonts`] instead to merge them into definitions of your own.
pub fn install_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    add_fonts(&mut fonts);
    ctx.set_fonts(fonts);
}

/// Every figure, path and log line is set in JetBrains Mono (OFL). Only the
/// regular weight is embedded: emphasis is colour, and each extra weight is
/// another 270 KB parsed at start.
///
/// Placed first in the monospace family rather than replacing it, so a glyph
/// the face lacks still falls back to egui's own Hack and emoji fonts.
#[allow(unused_variables, reason = "both fonts are optional features")]
pub fn add_fonts(fonts: &mut egui::FontDefinitions) {
    #[cfg(feature = "icons")]
    icons::add_font(fonts);
    #[cfg(feature = "fonts")]
    {
        fonts.font_data.insert(
            MONO_FONT.to_owned(),
            std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
                "../assets/fonts/JetBrainsMono-Regular.ttf"
            ))),
        );
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .insert(0, MONO_FONT.to_owned());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub(crate) const VOID: &str = include_str!("../themes/void.toml");

    #[test]
    fn a_theme_is_ten_lines_of_toml() {
        let theme: Theme = toml::from_str(VOID).expect("parses");
        assert_eq!(theme.name, "Void");
        assert_eq!(theme.accent(), Color32::from_rgb(140, 120, 255));
        assert_eq!(VOID.trim().lines().count(), 10);
    }

    /// The accent alphas are the whole state model: a theme that only swaps the
    /// accent must move selection, hover and press with it.
    #[test]
    fn every_accent_alpha_follows_the_theme() {
        let theme: Theme = toml::from_str(VOID).unwrap();
        let visuals = theme.visuals();
        assert_eq!(visuals.selection.bg_fill, theme.select_fill());
        assert_eq!(visuals.widgets.active.bg_fill, theme.widget_active());
        assert_eq!(visuals.widgets.hovered.bg_stroke, theme.hover_stroke());
        assert_ne!(theme.select_fill(), Theme::default().select_fill());
    }

    /// Plain egui has nothing to frost behind a window, so translucency is the
    /// host's call rather than the theme's.
    #[test]
    fn windows_are_opaque_unless_the_host_asks_for_glass() {
        let theme = Theme::default();
        assert_eq!(theme.visuals().window_fill.a(), 255);

        let glass = theme.visuals_with(&Options {
            window_opacity: 0.66,
        });
        assert!(glass.window_fill.a() < 200, "{:?}", glass.window_fill);

        let ctx = egui::Context::default();
        apply(&ctx, &theme);
        assert_eq!(ctx.style().visuals.window_fill.a(), 255);
    }

    /// Four sizes across five styles — weight and colour do the emphasis work,
    /// size never does. Pinned so a version bump cannot quietly edit the design.
    #[test]
    fn the_type_scale_is_four_sizes() {
        let ctx = egui::Context::default();
        apply(&ctx, &Theme::default());

        let mut unique: Vec<f32> = ctx.style().text_styles.values().map(|f| f.size).collect();
        unique.sort_by(f32::total_cmp);
        unique.dedup();

        assert_eq!(unique, vec![10.5, 11.5, 12.5, 15.0]);
        assert_eq!(
            ctx.style().text_styles[&egui::TextStyle::Monospace].family,
            egui::FontFamily::Monospace
        );
    }

    /// Figures are JetBrains Mono rather than egui's Hack, and falling back to
    /// the default would go unnoticed in every other test.
    #[cfg(feature = "fonts")]
    #[test]
    fn monospace_is_jetbrains_mono() {
        let ctx = egui::Context::default();
        install_fonts(&ctx);
        // Fonts set on a context take effect at the start of its next pass.
        let _ = ctx.run(Default::default(), |_| {});

        let first = ctx.fonts(|fonts| {
            fonts
                .lock()
                .fonts
                .definitions()
                .families
                .get(&egui::FontFamily::Monospace)
                .and_then(|family| family.first().cloned())
        });
        assert_eq!(first.as_deref(), Some(MONO_FONT));
    }

    /// A selection is a tint, not a flood: opaque, recognisably the accent's
    /// hue, and far nearer the panel's value than the accent's.
    #[test]
    fn a_selection_is_a_quiet_tint_of_the_accent() {
        let theme = Theme::default();
        let fill = theme.select_fill();
        let accent = theme.accent();
        assert_eq!(fill.a(), 255);
        assert!(fill.b() > fill.r(), "{fill:?} has lost the accent's hue");
        let luma =
            |c: Color32| 0.2126 * c.r() as f32 + 0.7152 * c.g() as f32 + 0.0722 * c.b() as f32;
        assert!(
            luma(fill) < luma(accent) * 0.6,
            "{fill:?} is nearly the accent"
        );
    }

    #[test]
    fn hover_is_neutral_and_idle_controls_are_outlined() {
        let visuals = Theme::default().visuals();
        let hover = visuals.widgets.hovered.bg_stroke.color;
        let spread =
            hover.r().max(hover.g()).max(hover.b()) - hover.r().min(hover.g()).min(hover.b());
        assert!(spread < 16, "the hover outline {hover:?} is tinted");
        // An unchecked checkbox in a menu is otherwise a box the same value
        // as the menu it sits in.
        assert!(visuals.widgets.inactive.bg_stroke.width > 0.0);
        assert_ne!(
            visuals.widgets.inactive.bg_stroke.color,
            rgb(Theme::default().window)
        );
    }

    #[test]
    fn scroll_bars_are_hidden_until_needed() {
        let ctx = egui::Context::default();
        apply(&ctx, &Theme::default());
        let scroll = ctx.style().spacing.scroll;
        assert!(scroll.floating);
        assert_eq!(scroll.dormant_handle_opacity, 0.0);
        assert_eq!(scroll.dormant_background_opacity, 0.0);
        assert!(scroll.bar_width <= 8.0);
    }

    /// An input is a well cut into the panel, not a button raised off it: the
    /// field sits below the panel, the panel below a raised header, and hover
    /// lifts a control rather than sinking it.
    #[test]
    fn surfaces_climb_in_value_as_they_come_forward() {
        let theme = Theme::default();
        let luma = |[r, g, b]: [u8; 3]| 0.2126 * r as f32 + 0.7152 * g as f32 + 0.0722 * b as f32;
        assert!(luma(theme.extreme) < luma(theme.panel));
        assert!(luma(theme.panel) < luma(theme.window));
        assert!(luma(theme.widget) < luma(theme.widget_hover));
        assert!(luma(theme.bar_rgb()) < luma(theme.panel));
        assert!(luma(theme.bar_rgb()) > luma(theme.extreme));
    }

    #[test]
    fn only_the_disclosure_triangle_animates() {
        let ctx = egui::Context::default();
        apply(&ctx, &Theme::default());

        assert_eq!(ctx.style().animation_time, 0.120);
        assert_eq!(ctx.style().spacing.indent, 14.0);
    }
}
