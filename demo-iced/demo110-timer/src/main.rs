use iced::{widget, Element, Subscription, Theme};

pub fn main() -> iced::Result {
    iced::application("Demo: Timer", Timer::update, Timer::view)
        .subscription(Timer::subscription)
        .theme(|_| Theme::Dark)
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
}

impl Timer {
    pub fn subscription(&self) -> Subscription<Message> {
        // --features tokio
        iced::time::every(std::time::Duration::from_millis(100)).map(|_| Message::Tick)
    }
    fn update(&mut self, message: Message) {
        match message {
            Message::Tick => self.count += 1,
        }
    }

    fn view(&self) -> Element<Message> {
        widget::column![widget::text(self.count.to_string())].into()
    }
}
