use iced::{Element, Font, Theme};
use iced::widget::{button, column, text};


const NOTO_SANS_KR_REGULAR_TTF: &[u8] =
    include_bytes!("../../assets/fonts/Noto_Sans_KR/static/NotoSansKR-Regular.ttf");
// Regular 폰트만 되는 듯
// Noto Sans KR은 NotoSansKR-Regular.ttf에 등록된 폰트 이름이다. 폰트 파일을 더블클릭하면 보인다.
pub const NOTO_SANS_KR: Font = Font::with_name("Noto Sans KR");

pub fn main() -> iced::Result {
    iced::application("A counter", update, view)
        .theme(|_| Theme::Dark)
        .font(NOTO_SANS_KR_REGULAR_TTF)
        .default_font(NOTO_SANS_KR)
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
        text("한글이 되어야 한다"),
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
