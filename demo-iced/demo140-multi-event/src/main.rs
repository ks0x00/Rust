use iced::{
    Element, Event, Size, Subscription, Theme, event,
    keyboard::{self, key::Physical},
    widget::{Column, Row, Space, text, vertical_space},
    window::{self, Settings},
};

pub fn main() -> iced::Result {
    iced::application("Demo: Timer", Timer::update, Timer::view)
        .subscription(Timer::subscription)
        .window(Settings {
            size: Size {
                width: 300.0,
                height: 200.0,
            },
            ..Default::default()
        })
        .theme(|_| Theme::Light)
        .centered()
        .run()
}

#[derive(Debug, Default)]
struct Timer {
    count: u32,
}

#[derive(Debug)]
enum Message {
    Tick,
    WindowResized(Size),
    KeyPressed(Physical),
}

impl Timer {
    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            // Listen for window events (resize, keyboard)
            event::listen_with(|event, _status, _id| match event {
                Event::Window(window::Event::Resized(size)) => Some(Message::WindowResized(size)),
                Event::Keyboard(keyboard_event) => match keyboard_event {
                    keyboard::Event::KeyPressed { physical_key, .. } => {
                        Some(Message::KeyPressed(physical_key))
                    }
                    _ => None,
                },
                _ => None,
            }),
            // Periodic tick for updates (e.g., solver animation, info text update)
            // --features tokio
            iced::time::every(std::time::Duration::from_millis(100)).map(|_| Message::Tick),
        ])
    }
    fn update(&mut self, message: Message) {
        match message {
            Message::Tick => self.count += 1,
            Message::WindowResized(size) => println!("Window resized: {:?}", size),
            Message::KeyPressed(key) => {
                if key == keyboard::key::Physical::Code(keyboard::key::Code::KeyZ) {
                    self.count = 0;
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        Column::new()
            .push(vertical_space().height(30.0))
            .push(
                Row::new()
                    .push(Space::with_width(50.0))
                    .push(text(self.count.to_string())),
            )
            .push(vertical_space().height(30.0))
            .push(text("Press z to reset"))
            .into()
    }
}
