use crate::font::{FontSize, LabelFontSize};
use crate::theme::Theme;
use chrono::{DateTime, Local};
use egui::{widgets::Label, Pos2, Response, Sense, Stroke, Ui, Vec2};

pub trait CustomWidgets {
    fn header_label(&mut self, icon: &str, text: &str, landscape: bool, active: bool) -> Response;
    fn section(&mut self, text: &str);
    fn section_separator(&mut self);
}

pub fn solve_time_string(time: u32) -> String {
    let time = (time + 5) / 10;
    if time > 6000 {
        format!(
            "{}:{:02}.{:02}",
            time / 6000,
            (time % 6000) / 100,
            time % 100
        )
    } else {
        format!("{}.{:02}", time / 100, time % 100)
    }
}

pub fn solve_time_short_string(time: u32) -> String {
    let time = time / 100;
    if time > 600 {
        format!("{}:{:02}.{}", time / 600, (time % 600) / 10, time % 10)
    } else {
        format!("{}.{}", time / 10, time % 10)
    }
}

pub fn date_string(time: &DateTime<Local>) -> String {
    let now = Local::now();
    let current_day = now.date();
    let target_day = time.date();
    let days = (current_day - target_day).num_days();
    match days {
        0 => format!(
            "Today at {}",
            time.time().format("%l:%M %P").to_string().trim()
        ),
        1 => format!(
            "Yesterday at {}",
            time.time().format("%l:%M %P").to_string().trim()
        ),
        2..=6 => format!(
            "{} at {}",
            target_day.format("%A"),
            time.time().format("%l:%M %P").to_string().trim()
        ),
        7..=364 => format!(
            "{} {} at {}",
            target_day.format("%B"),
            target_day.format("%e").to_string().trim(),
            time.time().format("%l:%M %P").to_string().trim()
        ),
        _ => format!(
            "{} {} at {}",
            target_day.format("%B"),
            target_day.format("%e, %Y").to_string().trim(),
            time.time().format("%l:%M %P").to_string().trim()
        ),
    }
}

impl CustomWidgets for Ui {
    fn header_label(&mut self, icon: &str, text: &str, landscape: bool, active: bool) -> Response {
        if landscape {
            self.add(
                if active {
                    Label::new(format!("{}  {}", icon, text)).text_color(Theme::Green)
                } else {
                    Label::new(format!("{}  {}", icon, text))
                }
                .sense(Sense::click()),
            )
        } else {
            // In portrait mode, display icon with small text below
            let icon_galley = self
                .fonts()
                .layout_single_line(FontSize::Normal.into(), icon.into());
            let text_galley = self
                .fonts()
                .layout_single_line(FontSize::Small.into(), text.into());

            let (response, painter) = self.allocate_painter(
                Vec2::new(text_galley.size.x, icon_galley.size.y + text_galley.size.y),
                Sense::click(),
            );

            let icon_height = icon_galley.size.y;
            let color = if active {
                Theme::Green.into()
            } else if response.hovered() {
                Theme::Content.into()
            } else {
                Theme::Disabled.into()
            };
            painter.galley(
                Pos2::new(
                    response.rect.center().x - icon_galley.size.x / 2.0,
                    response.rect.top(),
                ),
                icon_galley,
                color,
            );

            painter.galley(
                Pos2::new(response.rect.left(), response.rect.top() + icon_height),
                text_galley,
                color,
            );

            response
        }
    }

    fn section(&mut self, text: &str) {
        self.add(
            Label::new(text)
                .font_size(FontSize::Section)
                .text_color(Theme::Blue),
        );
        self.section_separator();
    }

    fn section_separator(&mut self) {
        self.scope(|ui| {
            ui.style_mut().visuals.widgets.noninteractive.bg_stroke = Stroke {
                width: 1.0,
                color: Theme::DarkBlue.into(),
            };
            ui.separator();
        });
    }
}
