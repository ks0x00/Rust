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

#[derive(Debug)]
struct Demo;

impl<Message> canvas::Program<Message> for Demo {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        let n = 30;
        let center = Point::new(300.0, 300.0);
        let inner_radius = 150.0;
        let outer_radius = 250.0;
        let mut end_angle: f32 = 0.0 / 180.0 * PI;
        // Try the following
        // let mut end_angle: f32 = 0.00001 / 180.0 * PI;
        let step = 2.0 * PI / n as f32;
        for i in 0..n {
            let start_angle = end_angle;
            end_angle += step;
            // determine color
            let g = i as f32 / n as f32;
            let color = Color::from_rgb(1.0 - g, g, 0.0);
            // it draws the wrong shape if i = 7 and T = f32
            // if i != 7 {continue;}
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

fn control_points_for_arc(
    center: &Point,
    radius: f32,
    start_angle: f32,
    end_angle: f32,
) -> (Point, Point, Point) {
    // https://html.spec.whatwg.org/multipage/canvas.html#building-paths:dom-context-2d-arcto
    // control points for the arc with center, etc.
    // let (p0, p1, p2) = control_points_for_arc(center, radius, start_angle, end_angle);
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
    // p_1 = c + t (-\sin s, \cos s)

    // By Pythagorian thream
    // \|\vec{cp_1}\|^2 = \|\vec{cp_0}\|^2 + \|\vec{p_0p_1}\|^2
    //                  = (1 + ((\sin e - \sin s) / (\cos s + \cos e)))^2 r^2
    //                  = r^2 / \cos^2((s - e) / 2)
    // let diff2 = (s - e) / 2, mid = (s + e) / 2
    // p_1 = c + r / |\cos diff2 | (\cos mid, \sin mid)

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

    // One may compute as the following.
    // let cos_diff2 = ((start_angle - end_angle) / 2.0).cos().abs();
    // let cos_mid = ((start_angle + end_angle) / 2.0).cos();
    // let sin_mid = ((start_angle + end_angle) / 2.0).sin();
    // let p1 = *center + Vector::new(cos_mid, sin_mid) * (radius / cos_diff2);
    
    // this is another method.
    // let d = radius / (cos_start + cos_end);
    // let p1 = *center + Vector::new(1.0 + (start_angle + end_angle).cos(), (start_angle + end_angle).sin()) * d;
    
    (p0, p1, p2)
}
