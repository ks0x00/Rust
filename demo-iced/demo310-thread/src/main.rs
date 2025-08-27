use std::{sync::mpsc, thread};

use iced::{
    Element, Size, Subscription, Theme,
    widget::{button, row, text},
    window::Settings,
};

pub fn main() -> iced::Result {
    iced::application("Demo: Thread", Demo::update, Demo::view)
        .subscription(Demo::subscription)
        .theme(|_| Theme::Light)
        .window(Settings {
            size: Size::new(300.0, 200.0),
            ..Default::default()
        })
        .centered()
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    OnStart,
    OnStop,
    // OnData(i32),
    Tick,
}

#[derive(Default)]
struct Demo {
    data: i32,
    data_rx: Option<mpsc::Receiver<i32>>,
    stop_tx: Option<mpsc::Sender<()>>,
}

impl Demo {
    fn start(&mut self) {
        let (data_tx, data_rx) = mpsc::channel();
        self.data_rx = Some(data_rx);

        let (stop_tx, stop_rx) = mpsc::channel();
        self.stop_tx = Some(stop_tx);

        thread::spawn(move || {
            let mut n = 0;
            loop {
                let _ = data_tx.send(n);
                thread::sleep(std::time::Duration::from_millis(100));
                if stop_rx.try_recv().is_ok() {
                    break;
                }
                n += 1;
            }
        });
    }
    fn stop(&self) {
        if let Some(stop_tx) = self.stop_tx.as_ref() {
            let _ = stop_tx.send(());
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        // Subscription::none()
        iced::time::every(std::time::Duration::from_millis(100)).map(|_| Message::Tick)
    }

    fn tick(&mut self) {
        if let Some(data_rx) = self.data_rx.as_ref() {
            if let Ok(data) = data_rx.try_recv() {
                self.data = data;
            }
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::OnStart => self.start(),
            Message::OnStop => self.stop(),
            // Message::OnData(data) => self.data = data,
            Message::Tick => self.tick(),
        }
    }

    fn view(&self) -> Element<Message> {
        row![
            button("Start").on_press(Message::OnStart),
            button("Stop").on_press(Message::OnStop),
            text(self.data)
        ]
        .into()
    }
}
