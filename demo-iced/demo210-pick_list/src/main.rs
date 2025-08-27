use iced::{Element, Size, Theme, widget::pick_list, window::Settings};

pub fn main() -> iced::Result {
    iced::application("Demo: PickList", Demo::update, Demo::view)
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
    Selected(String),
}

struct Demo {
    list: Vec<String>,
    selected: Option<String>,
}

impl Default for Demo {
    fn default() -> Self {
        Self {
            list: vec!["a".to_owned(), "b".to_owned()],
            selected: Some("a".to_owned()),
        }
    }
}

impl Demo {
    fn update(&mut self, message: Message) {
        match message {
            Message::Selected(value) => {
                self.selected = Some(value);
            }
        }
    }
    fn view(&self) -> Element<Message> {
        pick_list(
            &self.list[..],
            self.selected.clone(),
            Message::Selected // 여기서 값(String)을 자동으로 Message::Selected에 넣어줌
        ).into()
    }
}
