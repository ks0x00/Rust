#![windows_subsystem = "windows"]

use iced::widget::{column, text};
use iced::{Element, Theme};

pub fn main() -> iced::Result {
    iced::application("Hello Iced", update, view)
        .theme(|_| Theme::Light)
        .centered()
        .run()
}

#[derive(Debug, Clone)]
enum Message {}

fn update(_s: &mut i32, _message: Message) {}

fn view(_s: &i32) -> Element<Message> {
    column![text("Hello Iced!"),].into()
}
