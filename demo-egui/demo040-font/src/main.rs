use std::{collections::BTreeMap, sync::Arc};

use eframe::{
    egui::{
        Button, CentralPanel, Color32, Context, FontData, FontDefinitions, FontFamily, FontId,
        RichText, Vec2, Visuals,
    },
    epaint::text::{FontInsert, FontPriority, InsertFontFamily},
};

const NOTO_SANS_KR_REGULAR_TTF: &[u8] =
    include_bytes!("../../assets/fonts/Noto_Sans_KR/static/NotoSansKR-Regular.ttf");
const NANUM_MYEONGJO_REGULAR_TTF: &[u8] =
    include_bytes!("../../assets/fonts/Nanum_Myeongjo/NanumMyeongjo-Regular.ttf");

const NOTO_SANS_KR: &str = "Noto Sans KR";
const NANUM_MYEONGJO: &str = "Nanum Myeongjo";

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        centered: true,
        ..Default::default()
    }; // 프레임 설정
    eframe::run_native(
        "Demo: Button",
        options,
        Box::new(|cc| Ok(Box::new(Demo::new(cc)))),
    )
}

#[derive(Default)]
struct Demo {
    clicked: bool,
}

impl Demo {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::add_fonts(&cc.egui_ctx);
        Self::set_custom_fonts(&cc.egui_ctx);
        Self { clicked: false }
    }

    fn add_fonts(ctx: &Context) {
        /*
         * Not necessary
        ctx.add_font(FontInsert::new(
            NANUM_MYEONGJO,
            FontData::from_static(NANUM_MYEONGJO_REGULAR_TTF),
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
        */
        // default font
        ctx.add_font(FontInsert::new(
            NOTO_SANS_KR,
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
            NOTO_SANS_KR.to_owned(),
            Arc::new(FontData::from_static(NOTO_SANS_KR_REGULAR_TTF)),
        );
        let mut new_family = BTreeMap::new();
        new_family.insert(
            FontFamily::Name(NOTO_SANS_KR.into()),
            vec![NOTO_SANS_KR.to_owned()],
        );
        fonts.families.append(&mut new_family);

        fonts.font_data.insert(
            NANUM_MYEONGJO.to_owned(),
            Arc::new(FontData::from_static(NANUM_MYEONGJO_REGULAR_TTF)),
        );
        let mut new_family = BTreeMap::new();
        new_family.insert(
            FontFamily::Name(NANUM_MYEONGJO.into()),
            vec![NANUM_MYEONGJO.to_owned()],
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
        CentralPanel::default().show(ctx, |ui| {
            ui.label("산은 높고 물은 깊어");
            ui.label("Narrow gate");
            let color_text = RichText::new("Colored Text").color(Color32::from_rgb(0, 255, 255));
            ui.label(color_text);
            let mut text = RichText::new("산은 높고 물은 깊어")
                .strong()
                .size(20.)
                .color(Color32::BLACK);
            text = if self.clicked {
                text.font(FontId {
                    size: 20.0,
                    family: FontFamily::Name(NANUM_MYEONGJO.into()),
                })
            } else {
                text.font(FontId {
                    size: 20.0,
                    family: FontFamily::Name(NOTO_SANS_KR.into()),
                })
            };
            if ui
                .add(
                    Button::new(text)
                        .min_size(Vec2 { x: 100., y: 50. })
                        .fill(Color32::LIGHT_RED),
                )
                .clicked()
            {
                self.clicked = !self.clicked;
            }
        });
    }
}
