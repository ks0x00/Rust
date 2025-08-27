use iced::{
    Border, Color, Element, Length, Size, Theme,
    widget::{column, container, row, text},
    window::Settings,
};

pub fn main() -> iced::Result {
    iced::application("Demo: Widget", Demo::update, Demo::view)
        .theme(|_| Theme::Light)
        .window(Settings {
            size: Size::new(300.0, 200.0),
            ..Default::default()
        })
        .centered()
        .run()
}

const BACK_COLOR: Color = Color::from_rgb(1.0, 0.0, 1.0);
const TEXT_COLOR: Color = Color::from_rgb(0.0, 0.0, 0.0);

#[derive(Debug, Clone)]
enum Message {}

#[derive(Default)]
struct Demo;

impl Demo {
    fn update(&mut self, _: Message) {}
    fn view(&self) -> Element<Message> {
        let text_widget = text("Hello again!!!")
            .width(Length::Fill) // Make text fill available width for centering within container
            .height(Length::Fill) // Make text fill available height for centering within container
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center)
            .color(TEXT_COLOR);

        let color_area =
            container(text_widget)
                .width(200.0)
                .height(100.0)
                .style(move |_: &Theme| container::Style {
                    background: Some(BACK_COLOR.into()),
                    border: Border::default().width(1.0), // Add a subtle border
                    ..Default::default()
                });

        column![text("Hello!!!"), row![color_area]].into()
    }
}
