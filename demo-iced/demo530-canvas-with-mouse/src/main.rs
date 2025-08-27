use iced::widget::canvas;
use iced::{Color, Point, Rectangle, Renderer, Size, Theme};
use iced::{Element, mouse};

pub fn main() -> iced::Result {
    iced::application("Demo: Canvas with Mouse", Circle::update, Circle::view)
        .theme(|_| Theme::Light)
        .centered()
        .run()
}

// First, we define the data we need for drawing
#[derive(Debug)]
struct Circle {
    center: Point,
    radius: f32,
    cache: canvas::Cache,
    button_pressed: bool,
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            center: Point::new(0.0, 0.0),
            radius: 0.0,
            cache: canvas::Cache::default(),
            button_pressed: false,
        }
    }
}
#[derive(Debug, Clone)]
enum Message {
    ButtonPressed(Point),
    ButtonReleased(Point),
    CursorMoved(Point),
}

impl Circle {
    fn view(&self) -> Element<Message> {
        canvas(self).into()
    }

    fn update(&mut self, message: Message) {
        self.cache.clear();
        match message {
            Message::ButtonPressed(point) => {
                self.center = point;
                self.radius = 0.0;
                self.button_pressed = true;
            }
            Message::ButtonReleased(point) => {
                self.radius = self.center.distance(point);
                self.button_pressed = false;
            }
            Message::CursorMoved(point) => {
                if self.button_pressed {
                    self.radius = self.center.distance(point);
                }
            }
        }
    }
}

// Then, we implement the `Program` trait
impl canvas::Program<Message> for Circle {
    // No internal state
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        _bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        // let mut frame = canvas::Frame::new(renderer, Size::new(400.0, 400.0));
        let geometry = self.cache.draw(renderer, Size::new(400.0, 400.0), |frame| {
            // We create a `Path` representing a simple circle
            let circle = canvas::Path::circle(self.center, self.radius);

            // And fill it with some color
            frame.fill(&circle, Color::BLACK);
        });
        // Then, we produce the geometry
        // vec![frame.into_geometry()]
        vec![geometry]
    }

    fn update(
        &self,
        _state: &mut Self::State,
        event: canvas::Event,
        _bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (canvas::event::Status, Option<Message>) {
        if let canvas::Event::Mouse(e) = event {
            match e {
                mouse::Event::ButtonPressed(mouse::Button::Left) => {
                    if let Some(point) = cursor.position() {
                        (
                            canvas::event::Status::Captured,
                            Some(Message::ButtonPressed(point)),
                        )
                    } else {
                        (canvas::event::Status::Ignored, None)
                    }
                }
                mouse::Event::ButtonReleased(mouse::Button::Left) => {
                    if let Some(point) = cursor.position() {
                        (
                            canvas::event::Status::Captured,
                            Some(Message::ButtonReleased(point)),
                        )
                    } else {
                        (canvas::event::Status::Ignored, None)
                    }
                }
                mouse::Event::CursorMoved { position } => (
                    canvas::event::Status::Captured,
                    Some(Message::CursorMoved(position)),
                ),
                _ => (canvas::event::Status::Ignored, None),
            }
        } else {
            (canvas::event::Status::Ignored, None)
        }
    }
}
