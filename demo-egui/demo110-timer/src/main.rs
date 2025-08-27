use std::time::Duration;

use chrono::{Local, Timelike};
use eframe::egui::{CentralPanel, Context, Visuals};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        centered: true,
        ..Default::default()
    }; // 프레임 설정
    eframe::run_native(
        "Demo: Timer",
        options,
        Box::new(|_cc| Ok(Box::new(Demo::default()))),
    )
}

#[derive(Default)]
struct Demo {
    count: i32,
}

impl eframe::App for Demo {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(Visuals::light());
        self.count += 1;
        CentralPanel::default().show(ctx, |ui| {
            let current_time = Local::now();
            ui.label(format!(
                "{:02}:{:02}:{:02} - {}",
                current_time.hour(),
                current_time.minute(),
                current_time.second(),
                self.count
            ));
            ctx.request_repaint_after(Duration::from_millis(10));
        });
    }
}
