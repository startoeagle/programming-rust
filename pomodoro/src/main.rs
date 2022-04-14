#![doc = include_str!("../README.md")]

use notify_rust as notify;
use tokio::time;

mod gui;

const SECONDS_PER_MINUTE: u64 = 60;

#[derive(Clone, Copy, Debug, PartialEq)]
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

struct PomodoroSchedule {
    period: u8,
}

impl PomodoroSchedule {
    fn new() -> Self {
        Self { period: 0 }
    }

    fn next_period(&mut self) -> Period {
        self.period += 1;
        if self.period % 2 == 1 {
            return Period::Work;
        } else if self.period == 8 {
            self.period = 0;
            return Period::LongBreak;
        }
        Period::Break
    }
}

async fn notify(msg: &str) -> Result<(), notify_rust::error::Error> {
    notify::Notification::new()
        .summary("Pomodoro")
        .body(msg)
        .show()?;
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // let mut pomodoro_schedule = PomodoroSchedule::new();
    // let mut period = pomodoro_schedule.next_period();
    // let duration = time::Duration::from_secs;
    // let mut interval = time::interval(duration(SECONDS_PER_MINUTE * period as u64));
    // loop {
        // interval.tick().await;
        // notify(period.message())
            // .await
            // .expect("Could notify the system");
        // period = pomodoro_schedule.next_period();
        // interval = time::interval(duration(SECONDS_PER_MINUTE * period as u64));
    // }
    gui::start_main();
}

#[test]
fn test_period_schedule() {
    use crate::Period::*;

    let mut ps = PomodoroSchedule::new();
    for _ in 0..2 {
        assert_eq!(ps.next_period(), Work);
        assert_eq!(ps.next_period(), Break);

        assert_eq!(ps.next_period(), Work);
        assert_eq!(ps.next_period(), Break);

        assert_eq!(ps.next_period(), Work);
        assert_eq!(ps.next_period(), Break);

        assert_eq!(ps.next_period(), Work);
        assert_eq!(ps.next_period(), LongBreak);

        assert_eq!(ps.period, 0);
    }
}
