use crate::font::{FontSize, LabelFontSize};
use crate::theme::Theme;
use chrono::{DateTime, Local};
use egui::{widgets::Label, Response, Sense, Stroke, Ui};

pub trait CustomWidgets {
    fn header_label(&mut self, text: &str, active: bool) -> Response;
    fn section(&mut self, text: &str);
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
    fn header_label(&mut self, text: &str, active: bool) -> Response {
        self.add(
            if active {
                Label::new(text).text_color(Theme::Green)
            } else {
                Label::new(text)
            }
            .sense(Sense::click()),
        )
    }

    fn section(&mut self, text: &str) {
        self.add(
            Label::new(text)
                .font_size(FontSize::Section)
                .text_color(Theme::Blue),
        );
        self.scope(|ui| {
            ui.style_mut().visuals.widgets.noninteractive.bg_stroke = Stroke {
                width: 1.0,
                color: Theme::DarkBlue.into(),
            };
            ui.separator();
        });
    }
}
