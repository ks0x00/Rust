use eframe::egui::{Button, CentralPanel, Context, Visuals};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        centered: true,
        ..Default::default()
    }; // 프레임 설정
    eframe::run_native(
        "Demo: Button",
        options,
        Box::new(|_cc| Ok(Box::new(Demo::default()))),
    )
}

#[derive(Default)]
struct Demo {
    count: i32,
}

impl Demo {
    pub fn increase(&mut self, amount: i32) {
        self.count += amount
    }
}

impl eframe::App for Demo {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(Visuals::light());
        CentralPanel::default().show(ctx, |ui| {
            ui.label(format!("{}", self.count));
            if ui.button("Increase").clicked() {
                self.increase(1);
            }
            if ui
                .add_sized([80.0, 30.0], Button::new("Increase 2"))
                .clicked()
            {
                self.increase(2);
            }
        });
    }
}
