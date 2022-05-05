use iced::{button, Button, Column, Element};
use iced::{Application, Settings};
use iced::{Sandbox, Text};

#[derive(Default)]
struct Counter {
    value: i32,
    inc_button: button::State,
    dec_button: button::State,
}

impl Sandbox for Counter {
    type Message = Message;
    fn view(&mut self) -> Element<Message> {
        Column::new()
            .push(
                Button::new(&mut self.inc_button, Text::new("+"))
                    .on_press(Message::IncrementPressed),
            )
            .push(Text::new(self.value.to_string()).size(50))
            .push(
                Button::new(&mut self.dec_button, Text::new("-"))
                    .on_press(Message::DecrementPressed),
            )
            .padding(50)
            .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => self.value += 1,
            Message::DecrementPressed => self.value -= 1,
        }
    }

    fn new() -> Self {
        Counter::default()
    }

    fn title(&self) -> String {
        "The counter".into()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}

fn main() {
    Counter::run(Settings::default()).expect("Could not run the application");
}
