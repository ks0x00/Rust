use eframe::egui::{CentralPanel, Context, Key, Visuals};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        centered: true,
        ..Default::default()
    }; // 프레임 설정
    eframe::run_native(
        "Demo: Key",
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
        if ctx.input(|input| input.key_pressed(Key::Z)) {
            self.count += 1;
        }
        CentralPanel::default().show(ctx, |ui| {
            ui.label(format!("{}", self.count));
        });
    }
}
