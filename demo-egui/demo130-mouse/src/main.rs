use eframe::egui::{CentralPanel, Context, Pos2, TopBottomPanel, Visuals};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        centered: true,
        ..Default::default()
    }; // 프레임 설정
    eframe::run_native(
        "Demo: Mouse",
        options,
        Box::new(|_cc| Ok(Box::new(Demo::default()))),
    )
}

#[derive(Default)]
struct Demo {
    click_pos: Option<Pos2>,
}

impl eframe::App for Demo {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(Visuals::light());
        self.click_pos = ctx.input(|input| {
            if input
                .pointer
                .button_down(eframe::egui::PointerButton::Primary)
            {
                input.pointer.interact_pos()
            } else {
                None
            }
        });
        TopBottomPanel::top(" top panel").show(ctx, |ui| {
            ui.set_height(30.0);
            ui.label(format!("over all click pos: {:?}", self.click_pos));
        });
        CentralPanel::default().show(ctx, |ui| {
            let pos = ctx.input(|input| {
                if input
                    .pointer
                    .button_down(eframe::egui::PointerButton::Primary)
                {
                    input.pointer.interact_pos()
                } else {
                    None
                }
            });
            ui.label(format!(
                "central panel click pos: {:?} == over all click pos",
                pos
            ));
        });
    }
}
