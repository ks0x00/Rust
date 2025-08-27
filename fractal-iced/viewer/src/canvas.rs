use base::Pair;
use iced::{
    advanced::graphics::Primitive,
    mouse,
    widget::{
        canvas::{self, Cache, Geometry, Path, Stroke},
        image::Handle,
    },
    Point, Rectangle, Renderer, Size, Theme, Vector,
};

use crate::Message;

pub struct FractalCanvas {
    shift: Vector,
    rect: Rectangle,
    handle: Option<Handle>,
    cache: Cache,
    press_pos: Option<Point>,
}

impl FractalCanvas {
    // the offset of this canvas in screen
    const OFFSET: Point = Point { x: 310.0, y: 25.0 };

    pub fn new(image_size: &Pair<usize>) -> Self {
        Self {
            cache: Cache::default(),
            shift: Default::default(),
            rect: Rectangle::new(
                Point::ORIGIN,
                Size::new(image_size.x as f32, image_size.y as f32),
            ),
            handle: None,
            press_pos: None,
        }
    }

    pub fn resize(&mut self, image_size_x: usize, image_size_y: usize) {
        self.rect.width = image_size_x as f32;
        self.rect.height = image_size_y as f32;
    }

    pub fn update_handle(&mut self, handle: Handle) {
        self.cache.clear();
        self.handle = Some(handle);
    }

    pub fn cursor_moved(&mut self, pos: &Point) -> Point {
        if let Some(press_pos) = self.press_pos {
            self.shift = *pos - press_pos;
        }
        Point::new(pos.x - self.rect.x, pos.y - self.rect.y)
    }

    pub fn mouse_button_pressed(&mut self, pos: Point) {
        self.press_pos = Some(pos);
    }

    pub fn mouse_button_released(&mut self, pos: &Point) -> Option<Vector> {
        match self.press_pos {
            Some(press_pos) => {
                self.shift = Vector::ZERO;
                self.press_pos = None;
                Some(*pos - press_pos)
            }
            None => None,
        }
    }
}

impl canvas::Program<Message> for FractalCanvas {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        event: canvas::Event,
        _bounds: Rectangle,
        cursor: iced::advanced::mouse::Cursor,
    ) -> (canvas::event::Status, Option<Message>) {
        let pos = match cursor.position_from(Self::OFFSET) {
            Some(p) => p,
            _ => {
                return (canvas::event::Status::Ignored, None);
            }
        };
        let in_image = self.rect.x <= pos.x
            && pos.x < self.rect.x + self.rect.width
            && self.rect.y <= pos.y
            && pos.y < self.rect.y + self.rect.height;
        let message = match event {
            canvas::event::Event::Mouse(mouse_event) => match mouse_event {
                mouse::Event::ButtonPressed(mouse::Button::Right) if in_image => {
                    Message::CenterTo(pos)
                }
                mouse::Event::ButtonPressed(mouse::Button::Left) if in_image => {
                    Message::MouseButtonPressed(pos)
                }
                mouse::Event::ButtonReleased(mouse::Button::Left) => {
                    Message::MouseButtonReleased(pos)
                }
                mouse::Event::WheelScrolled { delta } if in_image => {
                    if match delta {
                        mouse::ScrollDelta::Lines { x: _, y } => y,
                        mouse::ScrollDelta::Pixels { x: _, y } => y,
                    } > 0.0
                    {
                        Message::ExpandAtPoint(pos, 1.1)
                    } else {
                        Message::ExpandAtPoint(pos, 1.0 / 1.1)
                    }
                }
                _ => Message::CursorMoved(pos, in_image),
            },
            _ => Message::CursorMoved(pos, in_image),
        };
        (canvas::event::Status::Captured, Some(message))
    }

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let geom0 = self.cache.draw(renderer, bounds.size(), |frame| {
            let path = Path::rectangle(Point::ORIGIN, self.rect.size());
            frame.stroke(&path, Stroke::default())
        });
        match self.handle.as_ref() {
            Some(handle) => {
                let rect = Rectangle::new(self.rect.position() + self.shift, self.rect.size());
                let prim = Primitive::Image {
                    handle: handle.clone(),
                    bounds: rect,
                };
                vec![Geometry::Wgpu(prim), geom0]
            }
            _ => vec![geom0],
        }
    }
}
