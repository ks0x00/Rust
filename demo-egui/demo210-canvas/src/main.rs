use std::{collections::BTreeMap, sync::Arc};

use eframe::{
    egui::{
        Align2, CentralPanel, Color32, Context, CornerRadius, FontData, FontDefinitions,
        FontFamily, FontId, PointerButton, Rect, Sense, Stroke, StrokeKind, Visuals, pos2,
    },
    epaint::text::{FontInsert, FontPriority, InsertFontFamily},
};

const NOTO_SANS_KR_REGULAR_TTF: &[u8] =
    include_bytes!("../../assets/fonts/Noto_Sans_KR/static/NotoSansKR-Regular.ttf");

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        centered: true,
        ..Default::default()
    }; // 프레임 설정
    eframe::run_native(
        "Demo: Canvas",
        options,
        Box::new(|cc| Ok(Box::new(Demo::new(cc)))),
    )
}

const FILL_COLOR: Color32 = Color32::from_rgb(128, 172, 0);

struct Demo {
    rect: Rect,
}

impl Demo {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::add_fonts(&cc.egui_ctx);
        Self::set_custom_fonts(&cc.egui_ctx);
        Self {
            rect: Rect {
                min: pos2(100.0, 150.0),
                max: pos2(500.0, 250.0),
            },
        }
    }

    fn add_fonts(ctx: &Context) {
        ctx.add_font(FontInsert::new(
            "Noto Sans KR-Regular",
            FontData::from_static(NOTO_SANS_KR_REGULAR_TTF),
            vec![
                InsertFontFamily {
                    family: FontFamily::Proportional,
                    priority: FontPriority::Highest,
                },
                InsertFontFamily {
                    family: FontFamily::Monospace,
                    priority: FontPriority::Lowest,
                },
            ],
        ));
    }

    fn set_custom_fonts(ctx: &Context) {
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "Noto Sans KR-Regular".to_owned(),
            Arc::new(FontData::from_static(NOTO_SANS_KR_REGULAR_TTF)),
        );

        let mut new_family = BTreeMap::new();
        new_family.insert(
            FontFamily::Name("Noto Sans KR-Regular".into()),
            vec!["Noto Sans KR-Regular".to_owned()],
        );
        fonts.families.append(&mut new_family);
        ctx.set_fonts(fonts);
    }

    #[allow(dead_code)]
    fn set_default_font(ctx: &Context) {
        // 기본 egui 폰트 설정으로 돌아갑니다.
        ctx.set_fonts(FontDefinitions::default());
    }
}

impl eframe::App for Demo {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(Visuals::light());
        ctx.input(|input| {
            if input.pointer.button_pressed(PointerButton::Primary) {
                if let Some(pos) = input.pointer.interact_pos() {
                    self.rect.min = pos;
                }
            } else if input.pointer.button_down(PointerButton::Primary) {
                // dragged
                if let Some(pos) = input.pointer.interact_pos() {
                    self.rect.max = pos;
                }
            } else if input.pointer.button_released(PointerButton::Primary) {
                if let Some(pos) = input.pointer.interact_pos() {
                    self.rect.max = pos;
                }
            }
        });
        CentralPanel::default().show(ctx, |ui| {
            let size = ui.available_size();
            let (_response, painter) = ui.allocate_painter(size, Sense::click());
            painter.line_segment(
                [pos2(50.0, 100.0), pos2(250.0, 100.0)],
                Stroke::new(2.0, Color32::BLACK),
            );
            // see also, painter.rect_filled() and painter.rect_stroke()
            painter.rect(
                self.rect,
                CornerRadius::ZERO,
                FILL_COLOR,
                Stroke::new(1.0, Color32::ORANGE),
                StrokeKind::Inside,
            );
            painter.text(
                self.rect.center(),
                Align2::CENTER_CENTER,
                "Drag your mouse\n마우스로 끌어서 사각형을 그리시오".to_string(),
                FontId {
                    size: 20.0,
                    family: FontFamily::Name("Noto Sans KR-Regular".into()),
                },
                Color32::BLACK,
            );
        });
    }
}
