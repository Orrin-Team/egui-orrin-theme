//! Phosphor (regular weight, 2.1.1, MIT), as a font.
//!
//! A font rather than a folder of SVGs, for sharpness. An SVG rasterised to a
//! 14px texture is only crisp if it lands exactly on the pixel grid at exactly
//! the size it was rasterised for, and a 1.5px stroke at that size is an
//! antialiased smear either way. A glyph goes through egui's font rasteriser,
//! which renders at the display's real pixel density and positions on whole
//! pixels — the same path that keeps the text beside it sharp. It also costs
//! one font atlas instead of a texture per icon per size.
//!
//! The font is embedded whole: 1 530 glyphs, 0.5 MB, of which a few dozen are
//! named below. Any other is [`Icon::new`] with its codepoint from that
//! release's `style.css`. The cost is binary size only, since egui parses a
//! font's tables lazily.

use egui::{Color32, FontFamily, FontId, RichText};

/// In bars and rows.
pub const ROW: f32 = 16.0;

const FAMILY: &str = "icons";
const FONT: &str = "Phosphor";

/// The font family the glyphs are set in.
pub fn family() -> FontFamily {
    FontFamily::Name(FAMILY.into())
}

/// Adds the icon font as its own family, leaving egui's families alone.
pub fn add_font(fonts: &mut egui::FontDefinitions) {
    fonts.font_data.insert(
        FONT.to_owned(),
        std::sync::Arc::new(
            egui::FontData::from_static(include_bytes!("../assets/fonts/Phosphor.ttf")).tweak(
                egui::FontTweak {
                    // The glyphs sit on the em square's baseline; nudged down so
                    // an icon centres on a line of body text rather than riding
                    // above it.
                    y_offset_factor: 0.08,
                    ..Default::default()
                },
            ),
        ),
    );
    fonts.families.insert(family(), vec![FONT.to_owned()]);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Icon(pub char);

impl Icon {
    pub const fn new(code: char) -> Self {
        Self(code)
    }
}

macro_rules! icons {
    ($($name:ident => $code:literal,)*) => {
        $(pub const $name: Icon = Icon($code);)*

        /// Every named icon.
        pub const ALL: &[(&str, Icon)] = &[$((stringify!($name), $name),)*];
    };
}

// Phosphor's codepoints, from its `style.css` at 2.1.1.
icons! {
    ACTIVITY => '\u{e000}',
    BOX => '\u{e1da}',
    CHECK => '\u{e182}',
    CHEVRON_DOWN => '\u{e136}',
    CHEVRON_RIGHT => '\u{e13a}',
    CIRCLE => '\u{e18a}',
    CIRCLE_DOT => '\u{ece0}',
    CIRCLE_X => '\u{e4f8}',
    ELLIPSIS => '\u{e1fe}',
    FILE_CODE => '\u{e914}',
    GLOBE => '\u{e288}',
    GRID => '\u{e296}',
    INFO => '\u{e2ce}',
    LAYERS => '\u{e466}',
    LIGHTBULB => '\u{e2dc}',
    LIST_TREE => '\u{ee48}',
    MOUNTAIN_SNOW => '\u{e7ae}',
    MOVE => '\u{e0a4}',
    PALETTE => '\u{e6c8}',
    PAUSE => '\u{e39e}',
    PLAY => '\u{e3d0}',
    PLUS => '\u{e3d4}',
    POINTER => '\u{e1dc}',
    REFRESH => '\u{e094}',
    ROTATE => '\u{e036}',
    ROTATE_CCW => '\u{e038}',
    SCALING => '\u{e0a2}',
    SEARCH => '\u{e30c}',
    SLIDERS => '\u{e434}',
    STEP_FORWARD => '\u{e5a6}',
    SUN => '\u{e472}',
    TERMINAL => '\u{eae8}',
    TRASH => '\u{e4a6}',
    WARNING => '\u{e4e0}',
    X => '\u{e4f6}',
}

/// An icon as text, for a label or a button.
pub fn text(icon: Icon, size: f32, color: Color32) -> RichText {
    RichText::new(icon.0.to_string())
        .family(family())
        .size(size)
        .color(color)
}

/// An icon and a word in one piece of text — a menu entry, a labelled button.
/// `None` takes the text colour, so the icon follows hover and press with it.
pub fn with_label(icon: Icon, color: Option<Color32>, label: &str) -> egui::WidgetText {
    let mut job = egui::text::LayoutJob::default();
    job.append(
        &icon.0.to_string(),
        0.0,
        egui::TextFormat::simple(
            FontId::new(ROW - 2.0, family()),
            color.unwrap_or(Color32::PLACEHOLDER),
        ),
    );
    job.append(
        label,
        7.0,
        egui::TextFormat::simple(FontId::proportional(12.5), Color32::PLACEHOLDER),
    );
    job.into()
}

/// Paint an icon centred in `rect`, as tall as the rect.
pub fn paint(ui: &egui::Ui, icon: Icon, rect: egui::Rect, color: Color32) {
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        icon.0,
        FontId::new(rect.height(), family()),
        color,
    );
}

/// An icon-only button.
pub fn button(
    ui: &mut egui::Ui,
    icon: Icon,
    size: f32,
    color: Color32,
    frame: bool,
) -> egui::Response {
    ui.add(egui::Button::new(text(icon, size, color)).frame(frame))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn context() -> egui::Context {
        let ctx = egui::Context::default();
        let mut fonts = egui::FontDefinitions::default();
        add_font(&mut fonts);
        ctx.set_fonts(fonts);
        let _ = ctx.run(Default::default(), |_| {});
        ctx
    }

    /// Every named icon has to be a glyph the font actually has. A codepoint it
    /// lacks falls back to egui's replacement box, which looks like a rendering
    /// bug rather than a typo in a table.
    #[test]
    fn every_icon_is_in_the_font() {
        let ctx = context();
        let font = FontId::new(ROW, family());
        for (name, icon) in ALL {
            assert!(
                ctx.fonts(|fonts| fonts.has_glyph(&font, icon.0)),
                "{name} (U+{:04X}) is not in the icon font",
                icon.0 as u32
            );
        }
    }

    /// No two names for one glyph: a table edited by hand drifts that way, and
    /// two commands drawn alike read as one.
    #[test]
    fn no_two_icons_share_a_glyph() {
        for (index, (name, icon)) in ALL.iter().enumerate() {
            for (other, other_icon) in &ALL[index + 1..] {
                assert_ne!(icon, other_icon, "{name} and {other}");
            }
        }
    }

    /// Icons are exactly the kind of addition that quietly costs a fifth of a
    /// second at startup. Laying out every glyph, which is what rasterises them
    /// into the atlas, stays far inside a frame.
    #[test]
    fn the_whole_set_lays_out_in_well_under_a_frame_budget() {
        let ctx = context();
        let start = std::time::Instant::now();
        let text: String = ALL.iter().map(|(_, icon)| icon.0).collect();
        ctx.fonts(|fonts| fonts.layout_no_wrap(text, FontId::new(ROW, family()), Color32::WHITE));
        let elapsed = start.elapsed();
        assert!(
            elapsed < std::time::Duration::from_millis(250),
            "{} icons took {elapsed:?}",
            ALL.len()
        );
    }
}
