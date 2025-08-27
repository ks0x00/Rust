use std::f32::consts::PI;

use iced::widget::canvas;
use iced::{Color, Element, Point, Rectangle, Renderer, Theme, mouse};
use iced::{Length, Vector};

pub fn main() -> iced::Result {
    iced::application("Canvas Demo", update, view)
        .theme(|_| Theme::Light)
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
        ////////////////////////////////////////////////////////////////////////////////
        // first pie
        let center = Point::new(100.0, 100.0);
        let radius = 50.0;
        let start_angle: f32 = 180.0 / 180.0 * PI;
        let end_angle: f32 = 210.0 / 180.0 * PI;
        let color = Color::from_rgb(0.0, 1.0, 1.0);
        draw_pie(&mut frame, center, radius, start_angle, end_angle, color);

        /////////////////////////////////////////////////////////////////////////////////////
        // second pie
        let start_angle: f32 = 375.0 / 180.0 * PI;
        let end_angle: f32 = 270.0 / 180.0 * PI;
        let color = Color::from_rgb(1.0, 0.0, 1.0);
        draw_pie(&mut frame, center, radius, start_angle, end_angle, color);

        //////////////////////////////////////////////////////////////////////////////
        // third
        let center = Point::new(150.0, 60.0);
        let inner_radius = 150.0;
        let outer_radius = 250.0;
        let start_angle: f32 = 30.0 / 180.0 * PI;
        let end_angle: f32 = 120.0 / 180.0 * PI;
        let color = Color::from_rgb(1.0, 0.0, 0.0);
        draw_circular_band(
            &mut frame,
            center,
            inner_radius,
            outer_radius,
            start_angle,
            end_angle,
            color,
        );

        //////////////////////////////////////////////////////////////////////////////
        // four
        let n = 30;
        let center = Point::new(200.0, 200.0);
        let inner_radius = 150.0;
        let outer_radius = 250.0;
        let mut end_angle: f32 = 0.0 / 180.0 * PI;
        let step = 2.0 * PI / n as f32;
        for i in 0..n {
            let start_angle = end_angle;
            end_angle += step;
            let color = Color::from_rgb(1.0 - i as f32 / n as f32, i as f32 / n as f32, 0.0);
            draw_circular_band(
                &mut frame,
                center,
                inner_radius,
                outer_radius,
                start_angle,
                end_angle,
                color,
            );
        }

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

fn draw_circular_band(
    frame: &mut canvas::Frame,
    center: Point,
    inner_radius: f32,
    outer_radius: f32,
    start_angle: f32,
    end_angle: f32,
    fill: impl Into<canvas::Fill>,
) {
    let (p0, p1, p2) = control_points_for_arc(&center, inner_radius, start_angle, end_angle);
    let (q0, q1, q2) = control_points_for_arc(&center, outer_radius, start_angle, end_angle);
    let path = canvas::Path::new(|builder: &mut canvas::path::Builder| {
        builder.move_to(p0);
        builder.arc_to(p1, p2, inner_radius);
        builder.line_to(q2);
        builder.arc_to(q1, q0, outer_radius);
        builder.close();
    });
    frame.fill(&path, fill);
}

fn draw_pie(
    frame: &mut canvas::Frame,
    center: Point,
    radius: f32,
    start_angle: f32,
    end_angle: f32,
    fill: impl Into<canvas::Fill>,
) {
    let (p0, p1, p2) = control_points_for_arc(&center, radius, start_angle, end_angle);
    let mut path_builder = canvas::path::Builder::new();
    path_builder.move_to(center);
    path_builder.line_to(p0);
    path_builder.arc_to(p1, p2, radius);
    path_builder.close();
    frame.fill(&path_builder.build(), fill);
}

fn control_points_for_arc(
    center: &Point,
    radius: f32,
    start_angle: f32,
    end_angle: f32,
) -> (Point, Point, Point) {
    // https://html.spec.whatwg.org/multipage/canvas.html#building-paths:dom-context-2d-arcto
    // 중심, 반지름, start angle, end angle이 주어진 원의 원호는 다음과 같이 얻는다.
    // let (p0, p1, p2) = three_points_for_arc(center, radius, start_angle, end_angle);
    // builder.move_to(p0)
    // builder.arc_to(p1, p2, radius)
    // let path = builder.build();

    // c = center, r = radius, s = start_angle, e = end_angle
    // p_0 = c + (r\cos s, r\sin s)
    // v = \vec{cp_0} / || \vec{cp_0} || = \vec{cp_0} / r = (\cos s, \sin s)
    // v^\perp = (-\sin s, \cos s)
    // p_2 = c + (r\cos e, r\sin e)
    // w = \vec{cp_2} / || \vec{cp_2} || = \vec{cp_2} / r = (\cos e, \sin e)
    // w^\perp = (-\sin e, \cos e)
    // 대칭성을 이용하면 다음을 얻는다.
    // \vec{cp_1} = \vec{cp_0} + t v^\perp = \vec{cp_2} - t w^\perp for some t
    // \vec{cp_2} - \vec{cp_0} = t (v^\perp + w^\perp)
    // r (\cos e - \cos s, \sin e - \sin s)  = t (-\sin s - \sin e, \cos s + \cos e)
    // t = r (\cos e - \cos s) / (-\sin s - \sin e)
    //   = r (\sin e - \sin s) / (\cos s + \cos e)
    // 추가로 다음을 얻을 수 있다.
    // p_1 = p_0 + t v^\perp = p_2 - t w^\perp
    //     = c + \vec{cp_0} + t v^\perp
    //     = c + (r\cos s - t\sin s, r\sin s + t\cos s)
    //     = c + (r\cos s(\cos s + \cos e) - r\sin s(\sin e - \sin s),
    //            r\sin s(\cos s + \cos e) + r\cos s(\sin e - \sin s)) / (\cos s + \cos e)
    //     = c + r / (\cos s + \cos e) (1 + \cos(s + e), \sin(s + e))

    let cos_start = start_angle.cos();
    let sin_start = start_angle.sin();
    let cos_end = end_angle.cos();
    let sin_end = end_angle.sin();
    let p0 = *center + Vector::new(cos_start, sin_start) * radius;
    let p2 = *center + Vector::new(cos_end, sin_end) * radius;

    let t = radius * (sin_end - sin_start) / (cos_end + cos_start);
    let p1 = p0 + Vector::new(-sin_start, cos_start) * t;
    // 다음과 같이 계산해도 된다.
    // let d = radius / (cos_start + cos_end);
    // let p1 = *center + Vector::new(1.0 + (start_angle + end_angle).cos(), (start_angle + end_angle).sin()) * d;
    (p0, p1, p2)
}
