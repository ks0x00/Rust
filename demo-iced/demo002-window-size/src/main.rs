use iced::widget::{column, text};
use iced::{Element, Size, Theme, window};

pub fn main() -> iced::Result {
    let settings = window::Settings {
        size: Size {
            width: 300.0,
            height: 200.0,
        },
        ..Default::default()
    };
    iced::application("Hello Iced", update, view)
        .theme(|_| Theme::Light)
        .centered()
        .window(settings)
        .run()
}

#[derive(Debug, Clone)]
enum Message {}

fn update(_s: &mut i32, _message: Message) {}

fn view(_s: &i32) -> Element<Message> {
    column![text("Hello Iced!"),].into()
}
