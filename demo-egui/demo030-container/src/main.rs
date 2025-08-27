use eframe::egui::{CentralPanel, Context, Grid, TopBottomPanel, Visuals};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        centered: true,
        ..Default::default()
    }; // 프레임 설정
    eframe::run_native(
        "Demo: Container",
        options,
        Box::new(|_cc| Ok(Box::new(Demo::default()))),
    )
}

#[derive(Default)]
struct Demo;

impl eframe::App for Demo {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(Visuals::light());
        TopBottomPanel::top("top panel").show(ctx, |ui| {
            ui.set_height(30.0);
            ui.label("this is top panel");
        });
        CentralPanel::default().show(ctx, |ui| {
            let width = ui.available_width();
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("label1");
                    ui.label("label2");
                });

                Grid::new("grid1")
                    .min_col_width(width / 2.0)
                    .num_columns(2)
                    .show(ui, |ui| {
                        ui.label("label3");
                        ui.label("label4");
                        ui.end_row();
                        ui.label("label5");
                        ui.label("label6");
                    });
            });
        });
        TopBottomPanel::bottom("bottom panel").show(ctx, |ui| {
            ui.set_height(30.0);
            ui.label("this is bottom panel");
        });
    }
}
