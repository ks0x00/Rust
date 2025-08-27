use std::sync::Arc;

use base::{ColorType, Fractal, Pair, COLOR_CLASS};
use eframe::egui::{
    vec2, Align, Button, CentralPanel, Color32, Context, Image, Key, Label, Layout, Pos2,
    RadioButton, Rect, SidePanel, TextEdit, TopBottomPanel, Ui, Window,
};
use frac::calc::Calculator;
use func::{rational::Rational, Function};
use param::{DParameter, SParameter};
use crate::{
    data::DataManager, SetWithOffset, IMAGE_EXPAND_CAPTIONS, IMAGE_EXPAND_FACTORS,
    ITER_EXPAND_CAPTIONS, ITER_EXPAND_FACTORS, UNIT_EXPAND_CAPTIONS, UNIT_EXPAND_FACTORS,
};

pub struct FractalApp {
    data_mgr: DataManager,
    sparam: SParameter,
    view: Calculator,
    pixel: Pair<usize>,
    dparam: DParameter,
    load_saved: bool,
    from_text: bool,
    text_for_from_text: String,
}

impl FractalApp {
    const OFFSET: Pos2 = Pos2::new(300.0, 30.0);

    pub fn new() -> Self {
        let sparam = SParameter::new();
        let dparam = DParameter::default();
        let view = Calculator::new(&sparam, Arc::clone(&dparam.elapsed));
        Self {
            data_mgr: DataManager::new(),
            sparam,
            view,
            pixel: Default::default(),
            dparam,
            load_saved: false,
            from_text: false,
            text_for_from_text: "".to_string(),
        }
    }

    fn add_color_type_radio(
        &mut self,
        ui: &mut Ui,
        size: &[f32; 2],
        color_type: ColorType,
        text: &str,
    ) {
        let res = ui.add_sized(
            size,
            RadioButton::new(self.sparam.color_type == color_type, text),
        );
        if res.clicked() {
            self.sparam.color_type = color_type;
            self.view.redraw(&self.sparam);
            res.request_focus();
        }
    }
}

impl eframe::App for FractalApp {
    fn on_close_event(&mut self) -> bool {
        self.data_mgr.write_to_file();
        true
    }

    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.top_panel(ctx);
        self.left_panel(ctx);
        self.central_panel(ctx);
        self.load_dialog(ctx);
        self.from_text_dialog(ctx);
    }
}

impl FractalApp {
    fn top_panel(&mut self, ctx: &Context) {
        let top_panel = TopBottomPanel::top("top panel");
        top_panel.show(ctx, |ui| {
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                let mut label = ui.label("F");
                let res_f = ui
                    .add(TextEdit::singleline(&mut self.sparam.f).desired_width(200.0))
                    .labelled_by(label.id);
                label = ui.label("G");
                let res_g = ui
                    .add(TextEdit::singleline(&mut self.sparam.g).desired_width(100.0))
                    .labelled_by(label.id);
                let bgcolor = ui.style_mut().visuals.extreme_bg_color;
                if self.sparam.fractal != Fractal::Mandelbrot {
                    ui.style_mut().visuals.extreme_bg_color = Color32::LIGHT_GRAY;
                }
                label = ui.label("H");
                let res_h = ui
                    .add(
                        TextEdit::singleline(&mut self.sparam.h)
                            .interactive(self.sparam.fractal == Fractal::Mandelbrot)
                            .desired_width(100.0),
                    )
                    .labelled_by(label.id);
                label = ui.label("Tresh");
                ui.style_mut().visuals.extreme_bg_color = bgcolor;
                let res_th = ui
                    .add(TextEdit::singleline(&mut self.sparam.thresh).desired_width(120.0))
                    .labelled_by(label.id);
                if res_f.changed() && self.sparam.fractal == Fractal::Newton {
                    match Rational::parse(self.sparam.f.as_str(), 'z') {
                        Ok(rp) => {
                            self.sparam.h = rp.diff().to_string();
                        }
                        Err(_) => {}
                    }
                }
                if res_f.clicked() || res_g.clicked() || res_h.clicked() || res_th.clicked() {
                    if ctx.input(|i| i.key_pressed(Key::Enter)) {
                        self.view.redraw(&self.sparam);
                    }
                }

                ui.style_mut().visuals.extreme_bg_color = Color32::LIGHT_GRAY;
                label = ui.label("X");
                ui.add(
                    TextEdit::singleline(&mut self.dparam.x)
                        .interactive(false)
                        .desired_width(100.0),
                )
                .labelled_by(label.id);
                label = ui.label("Y");
                ui.add(
                    TextEdit::singleline(&mut self.dparam.y)
                        .interactive(false)
                        .desired_width(70.0),
                )
                .labelled_by(label.id);
                label = ui.label("Color");
                ui.add(
                    TextEdit::singleline(&mut self.dparam.color)
                        .interactive(false)
                        .desired_width(70.0),
                )
                .labelled_by(label.id);
                label = ui.label("Iter");
                ui.add(
                    TextEdit::singleline(&mut self.dparam.iter)
                        .interactive(false)
                        .desired_width(70.0),
                )
                .labelled_by(label.id);
                label = ui.label("Time");
                // let x = &mut *self.view.ellapsed.lock();
                ui.add(
                    TextEdit::singleline(&mut *self.dparam.elapsed.lock())
                        .interactive(false)
                        .desired_width(70.0),
                )
                .labelled_by(label.id);
            });
        });
    }

    fn left_panel(&mut self, ctx: &Context) {
        let left_panel = SidePanel::left("left panel").resizable(true);
        left_panel.show(ctx, |ui| {
            let resb = ui.add(RadioButton::new(
                self.sparam.fractal == Fractal::Mandelbrot,
                "Mandelbrot Set",
            ));
            if resb.clicked() {
                self.sparam.fractal = Fractal::Mandelbrot;
                self.sparam.reset();
                self.view.redraw(&self.sparam);
                resb.request_focus();
            }

            let resj = ui.add(RadioButton::new(
                self.sparam.fractal == Fractal::Julia,
                "Julia Set",
            ));
            if resj.clicked() {
                self.sparam.fractal = Fractal::Julia;
                self.sparam.reset();
                self.view.redraw(&self.sparam);
                resj.request_focus();
            };
            let resn = ui.add(RadioButton::new(
                self.sparam.fractal == Fractal::Newton,
                "Newton Method",
            ));
            if resn.clicked() {
                self.sparam.fractal = Fractal::Newton;
                self.sparam.reset();
                self.view.redraw(&self.sparam);
                resn.request_focus();
            };

            ui.add_space(5.0);
            let textedit_width = 180.0;
            let label_size = [70.0, 10.0];
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                let label = ui.add_sized(label_size, Label::new("Range X"));
                ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                    let res0 = ui
                        .add(
                            TextEdit::singleline(&mut self.sparam.min_x)
                                .desired_width(textedit_width),
                        )
                        .labelled_by(label.id);
                    let res1 = ui.add(
                        TextEdit::singleline(&mut self.sparam.max_x).desired_width(textedit_width),
                    );
                    if res0.changed() || res1.changed() {
                        self.sparam.reset_image_size_x();
                    } else if res0.clicked() || res1.clicked() {
                        if ctx.input(|i| i.key_pressed(Key::Enter)) {
                            self.view.redraw(&self.sparam);
                        }
                    }
                });
            });
            ui.add_space(5.0);
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                let label = ui.add_sized(label_size, Label::new("Range Y"));
                ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                    let res0 = ui
                        .add(
                            TextEdit::singleline(&mut self.sparam.min_y)
                                .desired_width(textedit_width),
                        )
                        .labelled_by(label.id);
                    let res1 = ui.add(
                        TextEdit::singleline(&mut self.sparam.max_y).desired_width(textedit_width),
                    );
                    if res0.changed() || res1.changed() {
                        let _ = self.sparam.reset_image_size_y();
                    } else if res0.clicked() || res1.clicked() {
                        if ctx.input(|i| i.key_pressed(Key::Enter)) {
                            self.view.redraw(&self.sparam);
                        }
                    }
                });
            });
            ui.add_space(5.0);
            let width = 180.0;
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                let label = ui.add_sized(label_size, Label::new("Image Size"));
                ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                    ui.style_mut().visuals.extreme_bg_color = Color32::LIGHT_GRAY;
                    ui.add(
                        TextEdit::singleline(&mut self.sparam.image_size_x)
                            .desired_width(width)
                            .interactive(false),
                    )
                    .labelled_by(label.id);
                    ui.add(
                        TextEdit::singleline(&mut self.sparam.image_size_y)
                            .desired_width(width)
                            .interactive(false),
                    );
                });
            });
            ui.add_space(5.0);
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                let label = ui.add_sized(label_size, Label::new("Unit"));
                ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                    let res0 = ui
                        .add(TextEdit::singleline(&mut self.sparam.unit_x).desired_width(width))
                        .labelled_by(label.id);
                    let res1 =
                        ui.add(TextEdit::singleline(&mut self.sparam.unit_y).desired_width(width));
                    if res0.changed() {
                        self.sparam.reset_image_size_x();
                    } else if res1.changed() {
                        self.sparam.reset_image_size_y();
                    } else if res0.clicked() || res1.clicked() {
                        if ctx.input(|i| i.key_pressed(Key::Enter)) {
                            self.view.redraw(&self.sparam);
                        }
                    }
                });
            });
            ui.horizontal_top(|ui| {
                UNIT_EXPAND_CAPTIONS.iter().enumerate().for_each(|(i, &c)| {
                    if ui.button(c).clicked() {
                        self.sparam.mul_unit(UNIT_EXPAND_FACTORS[i]);
                        self.view.redraw(&self.sparam);
                    }
                });
            });
            ui.add_space(5.0);
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                let label = ui.add_sized(label_size, Label::new("Max Iter"));
                if ui
                    .add(TextEdit::singleline(&mut self.sparam.max_iter).desired_width(width))
                    .labelled_by(label.id)
                    .clicked()
                {
                    if ctx.input(|i| i.key_pressed(Key::Enter)) {
                        self.view.redraw(&self.sparam);
                    }
                }
            });
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                ITER_EXPAND_CAPTIONS.iter().enumerate().for_each(|(i, &c)| {
                    if ui.button(c).clicked() {
                        self.sparam.mul_max_iter(ITER_EXPAND_FACTORS[i], i % 2 == 1);
                        self.view.redraw(&self.sparam);
                    }
                });
            });
            ui.add_space(5.0);
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                ui.add(Label::new("Expand at the Center"));
            });
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                IMAGE_EXPAND_CAPTIONS
                    .iter()
                    .enumerate()
                    .for_each(|(i, &c)| {
                        if ui.button(c).clicked() {
                            self.sparam.expand_at_center(IMAGE_EXPAND_FACTORS[i]);
                            self.view.redraw(&self.sparam);
                        }
                    });
            });
            ui.add_space(5.0);
            ui.add(Label::new("Color Type"));
            let size = [49.0_f32, 17.0];
            for cls in COLOR_CLASS {
                ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                    for var in 0..5 {
                        self.add_color_type_radio(
                            ui,
                            &size,
                            ColorType::new(cls, var),
                            format!("{}{}", cls, var).as_str(),
                        );
                    }
                });
            }
            ui.add_space(5.0);
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                let label = ui.add(Label::new("N Workers"));
                let res = ui
                    .add(TextEdit::singleline(&mut self.sparam.n_workers).desired_width(width))
                    .labelled_by(label.id);
                if res.clicked() {
                    if ctx.input(|i| i.key_pressed(Key::Enter)) {
                        self.view.redraw(&self.sparam);
                    }
                }
            });

            ui.add_space(5.0);
            let button_size = vec2(134.0, 19.0);
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                if ui.add_sized(button_size, Button::new("Stop")).clicked() {
                    self.view.stop_building();
                };
                if ui.add_sized(button_size, Button::new("Draw")).clicked() {
                    self.view.redraw(&self.sparam);
                };
            });
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                if ui.add_sized(button_size, Button::new("PNG")).clicked() {
                    self.view.png();
                };
                ui.add_sized(button_size, Button::new("Print"));
            });
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                if ui.add_sized(button_size, Button::new("Load")).clicked() {
                    self.load_saved = true;
                };
                if ui.add_sized(button_size, Button::new("Save")).clicked() {
                    self.data_mgr.push(&self.sparam.to_text());
                };
            });
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                if ui
                    .add_sized(button_size, Button::new("From Text"))
                    .clicked()
                {
                    self.from_text = true;
                };
                if ui.add_sized(button_size, Button::new("To Text")).clicked() {
                    let text = self.sparam.to_text();
                    cli_clipboard::set_contents(text).unwrap();
                };
            });
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                if ui.add_sized(button_size, Button::new("Reset")).clicked() {
                    self.sparam.reset();
                    self.view.redraw(&self.sparam)
                }
            });
        });
    }

    fn central_panel(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            let handle = self.view.texture_handle(ui);
            let res = ui.put(
                Rect {
                    min: Self::OFFSET,
                    max: Self::OFFSET + handle.size_vec2(),
                },
                Image::new(handle, handle.size_vec2()),
            );
            if let Some(pos) = res.hover_pos() {
                self.pixel.set_with_offset(&pos, &Self::OFFSET);
                self.dparam.set_xy(&self.view.coord(&self.pixel));
                self.dparam.color = match self.view.color(&self.pixel) {
                    Some(c) => format!("{:02X}{:02X}{:02X}{:02X}", c[0], c[1], c[2], c[3]),
                    None => "".to_string(),
                };
                self.dparam.iter = match self.view.iter(&self.pixel) {
                    Some(iter) => format!("{}", iter),
                    None => "".to_string(),
                }
            }
            res.context_menu(|ui| self.context_menu(ui)); // contextmenu
        }); // show
    }

    fn context_menu(&mut self, ui: &mut Ui) {
        ui.set_max_width(300.0);
        ui.horizontal(|ui| {
            ui.add(Label::new("Expand at the Center"));
            IMAGE_EXPAND_CAPTIONS
                .iter()
                .enumerate()
                .for_each(|(i, &c)| {
                    if ui.button(c).clicked() {
                        self.view
                            .expand_at_center(IMAGE_EXPAND_FACTORS[i], &mut self.sparam);
                        self.view.redraw(&self.sparam);
                        ui.close_menu();
                    }
                });
        }); // horizontal
        if ui
            .add(Button::new("Translate This Point to Center"))
            .clicked()
        {
            self.view.center_to(&self.pixel, &mut self.sparam);
            self.view.redraw(&self.sparam);
            ui.close_menu();
        }
        ui.add_space(2.0);
        ui.horizontal(|ui| {
            ui.add(Label::new("Expand at This Point"));
            IMAGE_EXPAND_CAPTIONS
                .iter()
                .enumerate()
                .for_each(|(i, &c)| {
                    if ui.button(c).clicked() {
                        self.view.expand_at_point(
                            &self.pixel,
                            IMAGE_EXPAND_FACTORS[i],
                            &mut self.sparam,
                        );
                        self.view.redraw(&self.sparam);
                        ui.close_menu();
                    }
                });
        }); // horizontal
        ui.add_space(2.0);
        ui.horizontal(|ui| {
            ui.add(Label::new("Translate This Point to Center and Expand"));
            IMAGE_EXPAND_CAPTIONS
                .iter()
                .enumerate()
                .for_each(|(i, &c)| {
                    if ui.button(c).clicked() {
                        self.view.center_and_expand(
                            &self.pixel,
                            IMAGE_EXPAND_FACTORS[i],
                            &mut self.sparam,
                        );
                        self.view.redraw(&self.sparam);
                        ui.close_menu();
                    }
                });
        }); // horizontal
    }

    fn load_dialog(&mut self, ctx: &Context) {
        if self.load_saved {
            let mut remove = self.data_mgr.len();
            let mut b = true;
            Window::new("Load").open(&mut b).show(ctx, |ui| {
                ui.set_max_width(1320.0);
                self.data_mgr.iter().enumerate().for_each(|(i, line)| {
                    ui.horizontal(|ui| {
                        if ui.button(line).clicked() {
                            self.sparam.from_text(line);
                            self.view.redraw(&self.sparam);
                            self.load_saved = false;
                        }
                        if ui.button("X").clicked() {
                            remove = i;
                        }
                    });
                });
            });
            if remove < self.data_mgr.len() {
                self.data_mgr.remove(remove);
            }
            if !b {
                self.load_saved = false;
            }
        }
    }

    fn from_text_dialog(&mut self, ctx: &Context) {
        if self.from_text {
            let mut b = true;
            Window::new("From Text")
                .open(&mut b)
                .resizable(true)
                .show(ctx, |ui| {
                    if ui
                        .text_edit_multiline(&mut self.text_for_from_text)
                        .clicked()
                    {
                        if ctx.input(|i| i.key_pressed(Key::Enter)) {
                            self.sparam.from_text(&self.text_for_from_text);
                            self.view.redraw(&self.sparam);
                            self.from_text = false;
                        }
                    }
                });
            if !b {
                self.from_text = false;
            }
        }
    }
}
