use iced::{Border, Color, Element, Event, Length, Subscription, Theme, event, widget};

pub fn main() -> iced::Result {
    iced::application("Demo: Key Press", Demo::update, Demo::view)
        .subscription(Demo::subscription)
        .theme(|_| Theme::Dark)
        .centered()
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    CursorMoved(iced::Point),
    ButtonPressed,
    MouseEnteredToText,
    MouseExistedFromText,
}
const ENTER_COLOR: Color = Color::from_rgb(0.0, 1.0, 0.0);
const EXIT_COLOR: Color = Color::from_rgb(1.0, 0.0, 1.0);
const TEXT_COLOR: Color = Color::from_rgb(0.0, 0.0, 0.0);
struct Demo {
    cursor_pos: iced::Point,
    msg0: String,
    msg1: String,
    back_color: iced::Color,
}

impl Default for Demo {
    fn default() -> Self {
        Self {
            cursor_pos: Default::default(),
            msg0: Default::default(),
            msg1: "Hello Demo!!!".to_owned(),
            back_color: EXIT_COLOR,
        }
    }
}

impl Demo {
    pub fn subscription(&self) -> Subscription<Message> {
        event::listen_with(|event, _status, _id| match event {
            Event::Mouse(mouse_event) => match mouse_event {
                iced::mouse::Event::CursorEntered => None,
                iced::mouse::Event::CursorLeft => None,
                iced::mouse::Event::CursorMoved { position } => {
                    Some(Message::CursorMoved(position))
                }
                iced::mouse::Event::ButtonPressed(button) => match button {
                    iced::mouse::Button::Left => Some(Message::ButtonPressed),
                    _ => None,
                },
                iced::mouse::Event::ButtonReleased(_button) => None,
                iced::mouse::Event::WheelScrolled { .. } => None,
            },
            _ => None,
        })
    }
    fn update(&mut self, message: Message) {
        match message {
            Message::CursorMoved(pos) => self.cursor_pos = pos,
            Message::ButtonPressed => self.msg0 = format!("Button pressed at {}", self.cursor_pos),
            Message::MouseEnteredToText => self.back_color = ENTER_COLOR,
            Message::MouseExistedFromText => self.back_color = EXIT_COLOR,
        }
    }
    fn view(&self) -> Element<Message> {
        let text_widget = widget::text(&self.msg1)
            .width(Length::Fill) // Make text fill available width for centering within container
            .height(Length::Fill) // Make text fill available height for centering within container
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center)
            .color(TEXT_COLOR);

        let clickable_area = widget::container(text_widget)
            .width(200.0)
            .height(100.0)
            .style(move |_: &Theme| widget::container::Style {
                background: Some(self.back_color.into()),
                border: Border::default().width(1.0), // Add a subtle border
                ..Default::default()
            });

        widget::column![
            widget::text(&self.msg0),
            widget::row![
                widget::mouse_area(clickable_area)
                    .on_enter(Message::MouseEnteredToText)
                    .on_exit(Message::MouseExistedFromText)
            ]
        ]
        .into()
    }
}
