use crate::theme::Theme;
use crate::widgets::{solve_time_short_string, solve_time_string};
use egui::Color32;
use instant::Instant;
use tpscube_core::{Analysis, AnalysisSummary, PartialAnalysis, TimedMove};

#[derive(Clone)]
pub enum TimerState {
    Inactive(u32, Option<Analysis>),
    Preparing(Instant, u32, Option<Analysis>),
    BluetoothPreparing(Instant, u32, Option<Analysis>),
    Ready,
    BluetoothReady,
    Solving(Instant),
    BluetoothSolving(Instant, Vec<TimedMove>, PartialAnalysis),
    SolveComplete(u32, Option<Analysis>),
}

impl TimerState {
    pub fn is_solving(&self) -> bool {
        match self {
            TimerState::Inactive(_, _) | TimerState::SolveComplete(_, _) => false,
            TimerState::Preparing(start, _, _) => (Instant::now() - *start).as_millis() > 10,
            _ => true,
        }
    }

    pub fn current_time_string(&self) -> String {
        match self {
            TimerState::Inactive(time, _) | TimerState::SolveComplete(time, _) => {
                solve_time_string(*time)
            }
            TimerState::Preparing(_, time, _) => {
                if self.is_solving() {
                    solve_time_short_string(0)
                } else {
                    solve_time_string(*time)
                }
            }
            TimerState::Ready
            | TimerState::BluetoothReady
            | TimerState::BluetoothPreparing(_, _, _) => solve_time_short_string(0),
            TimerState::Solving(start) | TimerState::BluetoothSolving(start, _, _) => {
                solve_time_short_string((Instant::now() - *start).as_millis() as u32)
            }
        }
    }

    pub fn current_time_color(&self) -> Color32 {
        match self {
            TimerState::Inactive(_, _)
            | TimerState::BluetoothPreparing(_, _, _)
            | TimerState::Solving(_)
            | TimerState::BluetoothSolving(_, _, _)
            | TimerState::SolveComplete(_, _) => Theme::Content.into(),
            TimerState::Preparing(_, _, _) => {
                if self.is_solving() {
                    Theme::BackgroundHighlight.into()
                } else {
                    Theme::Content.into()
                }
            }
            TimerState::Ready | TimerState::BluetoothReady => Theme::Green.into(),
        }
    }

    pub fn analysis(&self) -> Option<&dyn AnalysisSummary> {
        match self {
            TimerState::Inactive(_, Some(analysis)) => Some(analysis),
            TimerState::Preparing(_, _, Some(analysis)) => Some(analysis),
            TimerState::BluetoothPreparing(_, _, Some(analysis)) => Some(analysis),
            TimerState::BluetoothSolving(_, _, analysis) => Some(analysis),
            TimerState::SolveComplete(_, Some(analysis)) => Some(analysis),
            _ => None,
        }
    }
}
