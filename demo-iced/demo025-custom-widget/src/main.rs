use iced::{
    Border, Color, Element, Size, Theme,
    widget::{MouseArea, Space, column, container, mouse_area, text},
    window,
};

pub fn main() -> iced::Result {
    let settings = window::Settings {
        size: Size {
            width: 300.0,
            height: 200.0,
        },
        ..Default::default()
    };
    iced::application("Demo: Key Press", Demo::update, Demo::view)
        .theme(|_| Theme::Light)
        .window(settings)
        .centered()
        .run()
}

#[derive(Debug, Clone)]
pub enum Message {
    ButtonPressed,
}

#[derive(Default)]
struct Demo;

impl Demo {
    fn update(&mut self, message: Message) {
        match message {
            Message::ButtonPressed => println!("Button pressed"),
        }
    }
    fn view(&self) -> Element<Message> {
        column![
            Space::new(1.0, 20.0),
            custom_text("Press", 60.0, None),
            custom_button("Custom Button", 200.0, None).on_press(Message::ButtonPressed)
        ]
        .spacing(10)
        .padding(10)
        .into()
    }
}

pub fn custom_text(label: &str, width: f32, height: Option<f32>) -> Element<Message> {
    let style = container::Style::default().border(Border {
        color: Color::BLACK,
        width: 2.0,
        ..Default::default()
    });
    let height = height.unwrap_or(24.0);
    container(text(label))
        .style(move |_| style)
        .center_x(width)
        .center_y(height)
        .into()
}

pub fn custom_button(label: &str, width: f32, height: Option<f32>) -> MouseArea<Message> {
    let style = container::Style::default().border(Border {
        color: Color::BLACK,
        width: 2.0,
        radius: 5.0.into(),
    });
    let height = height.unwrap_or(24.0);
    mouse_area(
        container(text(label))
            .style(move |_| style)
            .center_x(width)
            .center_y(height),
    )
}
