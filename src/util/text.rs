use freya::prelude::*;

use crate::state::StateCtx;

/// 100
pub static FIRA_SANS_THIN: (&str, &[u8]) = (
    "FiraSans-Thin",
    include_bytes!("../../assets/Fira_Sans/FiraSans-Thin.ttf"),
);
/// 100
pub static FIRA_SANS_THIN_ITALIC: (&str, &[u8]) = (
    "FiraSans-ThinItalic",
    include_bytes!("../../assets/Fira_Sans/FiraSans-ThinItalic.ttf"),
);
/// 200
pub static FIRA_SANS_EXTRA_LIGHT: (&str, &[u8]) = (
    "FiraSans-ExtraLight",
    include_bytes!("../../assets/Fira_Sans/FiraSans-ExtraLight.ttf"),
);
/// 200
pub static FIRA_SANS_EXTRA_LIGHT_ITALIC: (&str, &[u8]) = (
    "FiraSans-ExtraLightItalic",
    include_bytes!("../../assets/Fira_Sans/FiraSans-ExtraLightItalic.ttf"),
);
/// 300
pub static FIRA_SANS_LIGHT: (&str, &[u8]) = (
    "FiraSans-Light",
    include_bytes!("../../assets/Fira_Sans/FiraSans-Light.ttf"),
);
/// 300
pub static FIRA_SANS_LIGHT_ITALIC: (&str, &[u8]) = (
    "FiraSans-LightItalic",
    include_bytes!("../../assets/Fira_Sans/FiraSans-LightItalic.ttf"),
);
/// 400
pub static FIRA_SANS_REGULAR: (&str, &[u8]) = (
    "FiraSans-Regular",
    include_bytes!("../../assets/Fira_Sans/FiraSans-Regular.ttf"),
);
/// 400
pub static FIRA_SANS_ITALIC: (&str, &[u8]) = (
    "FiraSans-Italic",
    include_bytes!("../../assets/Fira_Sans/FiraSans-Italic.ttf"),
);
/// 500
pub static FIRA_SANS_MEDIUM: (&str, &[u8]) = (
    "FiraSans-Medium",
    include_bytes!("../../assets/Fira_Sans/FiraSans-Medium.ttf"),
);
/// 500
pub static FIRA_SANS_MEDIUM_ITALIC: (&str, &[u8]) = (
    "FiraSans-MediumItalic",
    include_bytes!("../../assets/Fira_Sans/FiraSans-MediumItalic.ttf"),
);
/// 600
pub static FIRA_SANS_SEMI_BOLD: (&str, &[u8]) = (
    "FiraSans-SemiBold",
    include_bytes!("../../assets/Fira_Sans/FiraSans-SemiBold.ttf"),
);
/// 600
pub static FIRA_SANS_SEMI_BOLD_ITALIC: (&str, &[u8]) = (
    "FiraSans-SemiBoldItalic",
    include_bytes!("../../assets/Fira_Sans/FiraSans-SemiBoldItalic.ttf"),
);
/// 700
pub static FIRA_SANS_BOLD: (&str, &[u8]) = (
    "FiraSans-Bold",
    include_bytes!("../../assets/Fira_Sans/FiraSans-Bold.ttf"),
);
/// 700
pub static FIRA_SANS_BOLD_ITALIC: (&str, &[u8]) = (
    "FiraSans-BoldItalic",
    include_bytes!("../../assets/Fira_Sans/FiraSans-BoldItalic.ttf"),
);
/// 800
pub static FIRA_SANS_EXTRA_BOLD: (&str, &[u8]) = (
    "FiraSans-ExtraBold",
    include_bytes!("../../assets/Fira_Sans/FiraSans-ExtraBold.ttf"),
);
/// 800
pub static FIRA_SANS_EXTRA_BOLD_ITALIC: (&str, &[u8]) = (
    "FiraSans-ExtraBoldItalic",
    include_bytes!("../../assets/Fira_Sans/FiraSans-ExtraBoldItalic.ttf"),
);
/// 900
pub static FIRA_SANS_BLACK: (&str, &[u8]) = (
    "FiraSans-Black",
    include_bytes!("../../assets/Fira_Sans/FiraSans-Black.ttf"),
);
/// 900
pub static FIRA_SANS_BLACK_ITALIC: (&str, &[u8]) = (
    "FiraSans-BlackItalic",
    include_bytes!("../../assets/Fira_Sans/FiraSans-BlackItalic.ttf"),
);

const FALLBACK_FONT_SIZE: f64 = 28.0;
const FALLBACK_SPACING: u8 = 5;
const WEIGHT_STEP: u16 = 100;
const SIZE_STEP: f64 = 7.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TextProps {
    pub font_size: f64,
    pub font_family: &'static str,
    pub letter_spacing: u8,
    pub word_spacing: u8,
    pub line_height: f64,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum FontValue<T>
where
    T: std::fmt::Debug + Clone + Copy + 'static,
{
    MuchGreater,
    Greater,
    #[default]
    Inherit,
    Exact(T),
    Lesser,
    MuchLesser,
}

#[derive(Debug, Clone, Copy, Default, Builder)]
#[builder(setter(into), default)]
pub struct FontOptions {
    pub size: FontValue<f64>,
    pub weight: FontValue<u16>,
    pub italic: Option<bool>,
    pub spacing: FontValue<u8>,
}

pub fn use_font() -> Memo<TextProps> {
    use_font_with_options(
        FontOptionsBuilder::default()
            .build()
            .expect("default font options"),
    )
}

pub fn use_font_with_options(options: FontOptions) -> Memo<TextProps> {
    let StateCtx(state, _) = use_context::<StateCtx>();
    let text_ctx = try_use_context::<Memo<TextProps>>();

    let text_props = use_memo(move || {
        let config_size = state()
            .config
            .map(|c| c.font_size)
            .flatten()
            .unwrap_or(FALLBACK_FONT_SIZE);

        let base_size = if let Some(text_ctx) = text_ctx {
            text_ctx().font_size
        } else {
            config_size
        };

        let (base_weight, base_italic) = if let Some(text_ctx) = text_ctx {
            match text_ctx().font_family {
                v if v == FIRA_SANS_THIN.0 => (100, false),
                v if v == FIRA_SANS_THIN_ITALIC.0 => (100, true),
                v if v == FIRA_SANS_EXTRA_LIGHT.0 => (200, false),
                v if v == FIRA_SANS_EXTRA_LIGHT_ITALIC.0 => (200, true),
                v if v == FIRA_SANS_LIGHT.0 => (300, false),
                v if v == FIRA_SANS_LIGHT_ITALIC.0 => (300, true),
                v if v == FIRA_SANS_REGULAR.0 => (400, false),
                v if v == FIRA_SANS_ITALIC.0 => (400, true),
                v if v == FIRA_SANS_MEDIUM.0 => (500, false),
                v if v == FIRA_SANS_MEDIUM_ITALIC.0 => (500, true),
                v if v == FIRA_SANS_SEMI_BOLD.0 => (600, false),
                v if v == FIRA_SANS_SEMI_BOLD_ITALIC.0 => (600, true),
                v if v == FIRA_SANS_BOLD.0 => (700, false),
                v if v == FIRA_SANS_BOLD_ITALIC.0 => (700, true),
                v if v == FIRA_SANS_EXTRA_BOLD.0 => (800, false),
                v if v == FIRA_SANS_EXTRA_BOLD_ITALIC.0 => (800, true),
                v if v == FIRA_SANS_BLACK.0 => (900, false),
                v if v == FIRA_SANS_BLACK_ITALIC.0 => (900, true),
                v => unreachable!("no such font: {v}"),
            }
        } else {
            (400, false)
        };

        let base_spacing = if let Some(text_ctx) = text_ctx {
            text_ctx().letter_spacing
        } else {
            FALLBACK_SPACING
        };

        let weight = match options.weight {
            FontValue::MuchGreater => base_weight + WEIGHT_STEP * 2,
            FontValue::Greater => base_weight + WEIGHT_STEP,
            FontValue::Inherit => base_weight,
            FontValue::Exact(val) => val,
            FontValue::Lesser => base_weight - WEIGHT_STEP,
            FontValue::MuchLesser => base_weight - WEIGHT_STEP * 2,
        };

        let weight = weight.max(100).min(900);

        let italic = match options.italic {
            Some(val) => val,
            None => base_italic,
        };

        let font_family = match (weight, italic) {
            (100, false) => FIRA_SANS_THIN.0,
            (100, true) => FIRA_SANS_THIN_ITALIC.0,
            (200, false) => FIRA_SANS_EXTRA_LIGHT.0,
            (200, true) => FIRA_SANS_EXTRA_LIGHT_ITALIC.0,
            (300, false) => FIRA_SANS_LIGHT.0,
            (300, true) => FIRA_SANS_LIGHT_ITALIC.0,
            (400, false) => FIRA_SANS_REGULAR.0,
            (400, true) => FIRA_SANS_ITALIC.0,
            (500, false) => FIRA_SANS_MEDIUM.0,
            (500, true) => FIRA_SANS_MEDIUM_ITALIC.0,
            (600, false) => FIRA_SANS_SEMI_BOLD.0,
            (600, true) => FIRA_SANS_SEMI_BOLD_ITALIC.0,
            (700, false) => FIRA_SANS_BOLD.0,
            (700, true) => FIRA_SANS_BOLD_ITALIC.0,
            (800, false) => FIRA_SANS_EXTRA_BOLD.0,
            (800, true) => FIRA_SANS_EXTRA_BOLD_ITALIC.0,
            (900, false) => FIRA_SANS_BLACK.0,
            (900, true) => FIRA_SANS_BLACK_ITALIC.0,
            v => unreachable!("no such font: {v:?}"),
        };

        let font_size = match options.size {
            FontValue::MuchGreater => base_size + SIZE_STEP * 2.0,
            FontValue::Greater => base_size + SIZE_STEP,
            FontValue::Inherit => base_size,
            FontValue::Exact(val) => val,
            FontValue::Lesser => base_size - SIZE_STEP,
            FontValue::MuchLesser => base_size - SIZE_STEP * 2.0,
        };

        let letter_spacing = match options.spacing {
            FontValue::MuchGreater => base_spacing * 2,
            FontValue::Greater => (base_spacing * 3) / 2,
            FontValue::Inherit => base_spacing,
            FontValue::Exact(val) => val,
            FontValue::Lesser => base_spacing / 2,
            FontValue::MuchLesser => (base_spacing / 3) * 2,
        };

        let word_spacing = letter_spacing * 2;

        let line_height = if font_size / config_size > 1.0 {
            font_size / config_size
        } else {
            1.2
        };

        TextProps {
            font_size,
            font_family,
            letter_spacing,
            word_spacing,
            line_height,
        }
    });

    provide_context(text_props.clone());

    text_props
}
