use egui::{widgets::Label, FontDefinitions, FontFamily, TextStyle};
use std::borrow::Cow;
use std::collections::BTreeMap;

#[derive(Copy, Clone, PartialEq, Eq)]
// We use the text styles not as they are actually intended, as we need
// some abnormal font sizes and we can only have font sizes associated with
// the text styles. This enumeration acts as an easy way to semantically
// express the font size we want but automatically convert to the text
// style required to get the correct size.
pub enum FontSize {
    Small,
    Normal,
    Section,
    Scramble,
    BestTime,
    Timer,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ScreenSize {
    Small,
    Normal,
    Large,
    VeryLarge,
}

impl Into<TextStyle> for FontSize {
    fn into(self) -> TextStyle {
        match self {
            FontSize::Small => TextStyle::Small,
            FontSize::Normal => TextStyle::Body,
            FontSize::Section => TextStyle::Button,
            FontSize::Scramble => TextStyle::Heading,
            FontSize::BestTime => TextStyle::Heading,
            FontSize::Timer => TextStyle::Monospace,
        }
    }
}

pub trait LabelFontSize {
    fn font_size(self, size: FontSize) -> Self;
}

impl LabelFontSize for Label {
    fn font_size(self, size: FontSize) -> Self {
        self.text_style(size.into())
    }
}

pub fn font_definitions(screen_size: ScreenSize) -> FontDefinitions {
    let mut fonts = FontDefinitions {
        font_data: BTreeMap::new(),
        fonts_for_family: BTreeMap::new(),
        family_and_size: BTreeMap::new(),
    };

    fonts.font_data.insert(
        "OpenSans".into(),
        Cow::Borrowed(include_bytes!("../fonts/OpenSans-Regular.ttf")),
    );
    fonts.font_data.insert(
        "OpenSans Light".into(),
        Cow::Borrowed(include_bytes!("../fonts/OpenSans-Light.ttf")),
    );
    fonts.font_data.insert(
        "emoji-icon-font".into(),
        Cow::Borrowed(include_bytes!("../fonts/emoji-icon-font.ttf")),
    );
    fonts.fonts_for_family.insert(
        FontFamily::Proportional,
        vec!["OpenSans".into(), "emoji-icon-font".into()],
    );
    fonts.fonts_for_family.insert(
        FontFamily::Monospace,
        vec!["OpenSans Light".into(), "emoji-icon-font".into()],
    );

    if crate::is_mobile() == Some(true) {
        fonts
            .family_and_size
            .insert(FontSize::Small.into(), (FontFamily::Proportional, 20.0));
        fonts
            .family_and_size
            .insert(FontSize::Normal.into(), (FontFamily::Proportional, 24.0));
        fonts
            .family_and_size
            .insert(FontSize::Section.into(), (FontFamily::Proportional, 30.0));
    } else {
        fonts
            .family_and_size
            .insert(FontSize::Small.into(), (FontFamily::Proportional, 16.0));
        fonts
            .family_and_size
            .insert(FontSize::Normal.into(), (FontFamily::Proportional, 20.0));
        fonts
            .family_and_size
            .insert(FontSize::Section.into(), (FontFamily::Proportional, 24.0));
    }

    fonts.family_and_size.insert(
        FontSize::Scramble.into(),
        (
            FontFamily::Monospace,
            match screen_size {
                ScreenSize::Small => 32.0,
                ScreenSize::Normal => 40.0,
                ScreenSize::Large => 48.0,
                ScreenSize::VeryLarge => 64.0,
            },
        ),
    );
    fonts.family_and_size.insert(
        FontSize::Timer.into(),
        (
            FontFamily::Monospace,
            match screen_size {
                ScreenSize::Small => 80.0,
                ScreenSize::Normal => 128.0,
                ScreenSize::Large => 144.0,
                ScreenSize::VeryLarge => 192.0,
            },
        ),
    );

    fonts
}
