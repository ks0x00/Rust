use iced::alignment::{Horizontal, Vertical};
use iced::widget::canvas;
use iced::{Color, Font, Length, Point, Rectangle, Renderer, Size, Theme};
use iced::{Element, mouse};

const NOTO_SANS_KR_REGULAR_TTF: &[u8] =
    include_bytes!("../../assets/fonts/Noto_Sans_KR/static/NotoSansKR-Regular.ttf");
// Regular 폰트만 되는 듯
// Noto Sans KR은 NotoSansKR-Regular.ttf에 등록된 폰트 이름이다. 폰트 파일을 더블클릭하면 보인다.
pub const NOTO_SANS_KR: Font = Font::with_name("Noto Sans KR");

pub fn main() -> iced::Result {
    iced::application("Canvas Demo", update, view)
        .theme(|_| Theme::Light)
        .font(NOTO_SANS_KR_REGULAR_TTF)
        .centered()
        .run()
}

// First, we define the data we need for drawing
#[derive(Debug)]
struct Demo;

// Then, we implement the `Program` trait
impl<Message> canvas::Program<Message> for Demo {
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
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        frame.fill_text(canvas::Text {
            content: "Hello canvas, 한글은?".to_string(),
            position: Point::new(30.0, 50.0),
            color: Color::BLACK,
            font: NOTO_SANS_KR,
            size: iced::Pixels(20.0),               // Font size
            horizontal_alignment: Horizontal::Left, // Align2::CENTER_CENTER
            vertical_alignment: Vertical::Center,
            ..Default::default()
        });

        let segment = canvas::Path::line(Point::new(30.0, 100.0), Point::new(230.0, 100.0));
        frame.stroke(&segment, canvas::Stroke::default());

        let rect = canvas::Path::rectangle(Point::new(30.0, 120.0), Size::new(200.0, 30.0));
        frame.stroke(&rect, canvas::Stroke::default());

        // We create a `Path` representing a simple circle
        let circle = canvas::Path::circle(frame.center(), 50.0);
        // And fill it with some color
        frame.fill(&circle, Color::from_rgb8(255, 255, 0));
        frame.stroke(&circle, canvas::Stroke::default().with_color(Color::BLACK));
        // Then, we produce the geometry
        vec![frame.into_geometry()]
    }
}

#[derive(Debug, Clone)]
enum Message {}

// Finally, we simply use our `Circle` to create the `Canvas`!
fn view((): &()) -> Element<Message> {
    canvas(Demo {})
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn update((): &mut (), _message: Message) {}
