use crate::theme::Theme;
use egui::{
    epaint::Shadow,
    style::{Selection, Visuals, WidgetVisuals, Widgets},
    Color32, Stroke,
};

pub fn base_visuals() -> Visuals {
    Visuals {
        dark_mode: true,
        widgets: Widgets {
            noninteractive: WidgetVisuals {
                bg_fill: Theme::BackgroundWindow.into(),
                bg_stroke: Stroke {
                    width: 1.0,
                    color: Color32::TRANSPARENT,
                },
                corner_radius: 0.0,
                fg_stroke: Stroke {
                    width: 1.0,
                    color: Theme::Content.into(),
                },
                expansion: 0.0,
            },
            inactive: WidgetVisuals {
                bg_fill: Color32::TRANSPARENT,
                bg_stroke: Stroke {
                    width: 1.0,
                    color: Color32::TRANSPARENT,
                },
                corner_radius: 0.0,
                fg_stroke: Stroke {
                    width: 1.0,
                    color: Theme::Disabled.into(),
                },
                expansion: 0.0,
            },
            hovered: WidgetVisuals {
                bg_fill: Color32::TRANSPARENT,
                bg_stroke: Stroke {
                    width: 1.0,
                    color: Color32::TRANSPARENT,
                },
                corner_radius: 0.0,
                fg_stroke: Stroke {
                    width: 1.0,
                    color: Theme::Content.into(),
                },
                expansion: 0.0,
            },
            active: WidgetVisuals {
                bg_fill: Color32::TRANSPARENT,
                bg_stroke: Stroke {
                    width: 1.0,
                    color: Color32::TRANSPARENT,
                },
                corner_radius: 0.0,
                fg_stroke: Stroke {
                    width: 1.0,
                    color: Theme::Green.into(),
                },
                expansion: 0.0,
            },
            open: WidgetVisuals {
                bg_fill: Color32::TRANSPARENT,
                bg_stroke: Stroke {
                    width: 1.0,
                    color: Color32::TRANSPARENT,
                },
                corner_radius: 0.0,
                fg_stroke: Stroke {
                    width: 1.0,
                    color: Theme::Green.into(),
                },
                expansion: 0.0,
            },
        },
        selection: Selection {
            bg_fill: Theme::Selection.into(),
            stroke: Stroke {
                width: 1.0,
                color: Theme::Content.into(),
            },
        },
        extreme_bg_color: Theme::BackgroundDark.into(),
        hyperlink_color: Theme::Blue.into(),
        code_bg_color: Theme::BackgroundDark.into(),
        window_corner_radius: 0.0,
        window_shadow: Shadow {
            extrusion: 12.0,
            color: Color32::from_rgba_premultiplied(0, 0, 0, 160),
        },
        ..Default::default()
    }
}

pub fn header_visuals() -> Visuals {
    let mut result = base_visuals();
    result.widgets.noninteractive.bg_fill = Theme::BackgroundWindow.into();
    result
}

pub fn content_visuals() -> Visuals {
    let mut result = base_visuals();
    result.widgets.noninteractive.bg_fill = Theme::BackgroundDark.into();
    result
}

pub fn side_visuals() -> Visuals {
    let mut result = base_visuals();
    result.widgets.noninteractive.bg_fill = Theme::Background.into();
    result
}

pub fn dialog_visuals() -> Visuals {
    let mut result = base_visuals();
    result.widgets.noninteractive.bg_fill = Theme::BackgroundHighlight.into();
    result.widgets.noninteractive.bg_stroke = Stroke {
        width: 1.0,
        color: Theme::Disabled.into(),
    };
    result.widgets.inactive.fg_stroke = Stroke {
        width: 1.0,
        color: Theme::Blue.into(),
    };
    result
}

pub fn popup_visuals() -> Visuals {
    let mut result = base_visuals();
    result.widgets.noninteractive.bg_fill = Theme::BackgroundWindow.into();
    result.widgets.noninteractive.bg_stroke = Stroke {
        width: 1.0,
        color: Theme::Disabled.into(),
    };
    result.widgets.inactive.fg_stroke = Stroke {
        width: 1.0,
        color: Theme::Content.into(),
    };
    result.widgets.hovered.bg_fill = Theme::DarkBlue.into();
    result.widgets.active.bg_fill = Theme::DarkBlue.into();
    result.widgets.active.fg_stroke = Stroke {
        width: 1.0,
        color: Theme::Content.into(),
    };
    result
}

pub fn settings_visuals() -> Visuals {
    let mut result = base_visuals();
    result.widgets.noninteractive.bg_fill = Theme::BackgroundDark.into();
    result.widgets.noninteractive.fg_stroke = Stroke {
        width: 1.0,
        color: Theme::Disabled.into(),
    };
    result.widgets.inactive.fg_stroke = Stroke {
        width: 1.0,
        color: Theme::Content.into(),
    };
    result.widgets.hovered.fg_stroke = Stroke {
        width: 1.0,
        color: Theme::Green.into(),
    };
    result.widgets.active.fg_stroke = Stroke {
        width: 1.0,
        color: Theme::Green.into(),
    };
    result
}
