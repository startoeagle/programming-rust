use iced::{Element, Sandbox, Settings, Text};

pub fn start_main() -> iced::Result {
    Hello::run(Settings::default())
}

struct Hello;

impl Sandbox for Hello {
    type Message = ();

    fn new() -> Hello {
        Hello
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn update(&mut self, _message: Self::Message) {
        // This application has no interactions
    }

    fn view(&mut self) -> Element<Self::Message> {
        Text::new("Hello, world!").into()
    }

    fn background_color(&self) -> iced::Color {
        iced::Color::from_rgb(0.2, 0.2, 0.6)
    }
}
