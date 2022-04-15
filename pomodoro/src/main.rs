#![doc = include_str!("../README.md")]

use iced::Application;
use notify_rust as notify;

mod gui;

const SECONDS_PER_MINUTE: u64 = 60;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Period {
    Work = 25,
    Break = 5,
    LongBreak = 15,
}

impl Period {
    fn message(&self) -> &'static str {
        match self {
            Self::Work => "Time is up! Focus on your task",
            Self::Break => "Take a break",
            Self::LongBreak => "Take a long break; you deserve it!",
        }
    }
}

#[derive(Clone)]
struct PomodoroSchedule {
    period: u8,
}

impl PomodoroSchedule {
    fn new() -> Self {
        Self { period: 0 }
    }
}
impl Iterator for PomodoroSchedule {
    type Item = Period;

    fn next(&mut self) -> Option<Period> {
        self.period += 1;
        if self.period > 8 {
            None
        } else if self.period == 8 {
            Some(Period::LongBreak)
        } else if self.period % 2 == 1 {
            Some(Period::Work)
        } else {
            Some(Period::Break)
        }
    }
}

fn notify(msg: &str) -> Result<(), notify_rust::error::Error> {
    notify::Notification::new()
        .summary("Pomodoro")
        .body(msg)
        .show()?;
    Ok(())
}

fn main() {
    gui::AppData::run(iced::Settings::default()).expect("could start application");
}

#[test]
fn test_period_schedule() {
    use crate::Period::*;

    let mut ps = PomodoroSchedule::new();
    let mut period = ps.into_iter();
    assert_eq!(period.next(), Some(Work));
    assert_eq!(period.next(), Some(Break));

    assert_eq!(period.next(), Some(Work));
    assert_eq!(period.next(), Some(Break));

    assert_eq!(period.next(), Some(Work));
    assert_eq!(period.next(), Some(Break));

    assert_eq!(period.next(), Some(Work));
    assert_eq!(period.next(), Some(LongBreak));

    assert_eq!(period.next(), None);
}
#[test]
fn test_cyclic_schedule() {
    use crate::Period::*;

    let mut ps = PomodoroSchedule::new();
    let mut period = ps.into_iter().cycle();
    assert_eq!(period.next(), Some(Work));
    assert_eq!(period.next(), Some(Break));

    assert_eq!(period.next(), Some(Work));
    assert_eq!(period.next(), Some(Break));

    assert_eq!(period.next(), Some(Work));
    assert_eq!(period.next(), Some(Break));

    assert_eq!(period.next(), Some(Work));
    assert_eq!(period.next(), Some(LongBreak));

    assert_eq!(period.next(), Some(Work));
}
