#![windows_subsystem = "windows"]

use eframe::egui::{CentralPanel, Color32, Context, ViewportBuilder, Visuals};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([WIDTH, HEIGHT]),
        centered: true,
        ..Default::default()
    }; // 프레임 설정
    eframe::run_native(
        "Demo: Without console",
        options,
        Box::new(|_cc| Ok(Box::new(Demo::default()))),
    )
}

const WIDTH: f32 = 400.0;
const HEIGHT: f32 = 300.0;

#[derive(Default)]
struct Demo;

impl eframe::App for Demo {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(Visuals::light());
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Egui without console");
            ui.visuals_mut().override_text_color = Some(Color32::from_rgb(0, 255, 0));
            ui.label("execute \"cargo build --release\" and run exe file");
        });
    }
}
