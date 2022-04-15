use iced::{mouse::Button, Application, Sandbox, Slider};
use std::iter;

use crate::PomodoroSchedule;

pub struct AppData {
    schedule: iter::Cycle<PomodoroSchedule>,
}
impl Sandbox for AppData {

    type Message = ();

    fn new() -> Self {
        AppData {
            schedule: PomodoroSchedule::new().cycle(),
        }
    }

    fn title(&self) -> String {
        "Pomodoro".to_string()
    }

    fn update(
        &mut self,
        _message: Self::Message,
    ) {
    }

    fn view(&mut self) -> iced::Element<Self::Message> {
        let msg = if let Some(period) = self.schedule.next() {
            period.message()
        } else {
            "There are no periods to display"
        };
        match super::notify(msg) {
            Err(e) => println!("{}", e),
            Ok(_) => (),
        }
        let column = iced::Column::new().push(iced::Element::from(iced::Text::new(msg)));
        column.into()
    }
}

/// Displays the time left of the period
struct Timer {
    minute: u8,
    second: u8,
}
