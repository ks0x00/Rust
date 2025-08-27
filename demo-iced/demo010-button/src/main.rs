use iced::{Element, Theme};
use iced::widget::{button, column, text};

pub fn main() -> iced::Result {
    iced::application("A counter", update, view)
        .theme(|_| Theme::Dark)
        .centered()
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
}

#[derive(Default)]
struct State {
    pub count: u32,
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::Increment => state.count += 1,
    }
}

fn view(state: &State) -> Element<Message> {
    column![
        text(state.count), 
        button("+").on_press(Message::Increment),
    ].into()
}

// 다음 코드는 view()의 대안이다.
// fn view(state: &State) -> Column<Message> {
//     column![
//         text(state.count),
//         button("+").on_press(Message::Increment),
//     ]
// }
