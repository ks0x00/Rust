use iced::widget::{self, text};
use iced::{Element, Event, Subscription, Theme, event, keyboard};

pub fn main() -> iced::Result {
    iced::application("Demo: Key Press", Demo::update, Demo::view)
        .subscription(Demo::subscription)
        .theme(|_| Theme::Dark)
        .centered()
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    KeyPressed(String),
    KeyReleased(keyboard::Key),
}

#[derive(Default)]
struct Demo {
    msg0: String,
    msg1: String,
}

impl Demo {
    pub fn subscription(&self) -> Subscription<Message> {
        event::listen_with(|event, _status, _id| match event {
            Event::Keyboard(keyboard_event) => match keyboard_event {
                keyboard::Event::KeyPressed { text, .. } => {
                    // println!(
                    //     "physical_key: {physical_key:?}, key: {key:?}, modified_key: {modified_key:?}, location: {location:?}, modifiers: {modifiers:?}, text: {text:?}"
                    // );
                    if let Some(text) = text {
                        Some(Message::KeyPressed(text.to_string()))
                    } else {
                        None
                    }
                }
                keyboard::Event::KeyReleased {
                    key,
                    location: _,
                    modifiers: _,
                } => Some(Message::KeyReleased(key)),
                keyboard::Event::ModifiersChanged(_modifiers) => None,
            },
            _ => None,
        })
    }
    fn update(&mut self, message: Message) {
        match message {
            Message::KeyPressed(key) => {
                self.msg0 = if key == "Z" {
                    "Capital Z pressed"
                } else {
                    "Other key pressed"
                }
                .to_owned();
            }
            Message::KeyReleased(key) => {
                self.msg1 = if key.as_ref() == keyboard::Key::Character("z") {
                    "Z released"
                } else {
                    "Other key released"
                }
                .to_owned();
            }
        }
    }

    fn view(&self) -> Element<Message> {
        widget::column![text(&self.msg0), text(&self.msg1)].into()
    }
}
