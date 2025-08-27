
use iced::widget::canvas;
use iced::{Color, Length, Rectangle, Renderer, Theme};
use iced::{Element, mouse};

pub fn main() -> iced::Result {
    iced::application("Demo: Canvas with Cache", Circle::update, Circle::view)
        .theme(|_| Theme::Light)
        .centered()
        .run()
}

#[derive(Debug, Clone)]
enum Message {}

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

impl Circle {
    // Finally, we simply use our `Circle` to create the `Canvas`!
    fn view(self: &Self) -> Element<Message> {
        canvas(self).width(Length::Fill).height(Length::Fill).into()
    }

    fn update(self: &mut Self, _message: Message) {
        self.cache.clear();
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
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            // We prepare a new `Frame`
            // let mut frame = canvas::Frame::new(renderer, bounds.size());

            // We create a `Path` representing a simple circle
            let circle = canvas::Path::circle(frame.center(), self.radius);

            // And fill it with some color
            frame.fill(&circle, Color::BLACK);
        });

        vec![geometry]
        // Then, we produce the geometry
        // vec![frame.into_geometry()]
    }
}
