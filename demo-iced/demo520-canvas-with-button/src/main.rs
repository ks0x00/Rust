use iced::widget::canvas::Stroke;
use iced::widget::{button, canvas, column};
use iced::{Color, Length, Point, Rectangle, Renderer, Theme};
use iced::{Element, mouse};

pub fn main() -> iced::Result {
    iced::application("Canvas with Button Demo", Circle::update, Circle::view)
        .theme(|_| Theme::Light)
        .centered()
        .run()
}

// First, we define the data we need for drawing
#[derive(Debug)]
struct Circle {
    radius: f32,
    cache: canvas::Cache,
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            radius: 50.0,
            cache: canvas::Cache::default(),
        }
    }
}
#[derive(Debug, Clone)]
enum Message {
    Increment,
}

impl Circle {
    fn view(&self) -> Element<Message> {
        column![
            button("+").on_press(Message::Increment),
            canvas(self).width(500.0).height(Length::Fill),
        ]
        .into()
    }

    fn update(&mut self, message: Message) {
        self.cache.clear();
        match message {
            Message::Increment => self.radius += 1.0,
        }
    }
}

// Then, we implement the `Program` trait
impl<Message> canvas::Program<Message> for Circle {
    // No internal state
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        // We prepare a new `Frame`
        // let size = Size::new(2.0 * self.radius, 2.0 * self.radius);
        let size = bounds.size();
        // let mut frame = canvas::Frame::new(renderer, size);

        let geometry = self.cache.draw(renderer, size, |frame| {
            let rect = canvas::Path::rectangle(Point::new(0.0, 0.0), size);
            frame.stroke(&rect, Stroke::default());

            // We create a `Path` representing a simple circle
            let circle = canvas::Path::circle(frame.center(), self.radius);

            // And fill it with some color
            frame.fill(&circle, Color::BLACK);
        });
        // Then, we produce the geometry
        // vec![frame.into_geometry()]
        vec![geometry]
    }
}
