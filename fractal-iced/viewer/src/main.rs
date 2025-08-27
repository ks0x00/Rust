#![windows_subsystem = "windows"]

use std::sync::Arc;
use std::time::Duration;

use base::{ColorType, ColorVar, FractalType, COLOR_TYPE, COLOR_VAR};
use frac::calc::Calculator;
use iced::widget::{
    button, canvas, column, container, horizontal_space, radio, row, text, text_input,
    vertical_space,
};
use iced::{
    executor, subscription, theme, window, Alignment, Event, Length, Padding, Subscription,
};
use iced::{Application, Command, Element, Settings, Theme};
use param::{DParameter, SParameter};
use viewer::canvas::FractalCanvas;
use viewer::data::DataManager;
use viewer::history::History;
use viewer::modal::Modal;
use viewer::Message;
use viewer::{
    IMAGE_EXPAND_CAPTIONS, IMAGE_EXPAND_FACTORS, ITER_EXPAND_CAPTIONS, ITER_EXPAND_FACTORS,
    UNIT_EXPAND_CAPTIONS, UNIT_EXPAND_FACTORS,
};

pub fn main() -> iced::Result {
    // env_logger::builder().format_timestamp(None).init();

    let window = window::Settings {
        size: (1440, 900),
        ..window::Settings::default()
    };
    FractalApp::run(Settings {
        window,
        default_text_size: 14.0,
        exit_on_close_request: false,
        ..Settings::default()
    })
}

struct FractalApp {
    data_mgr: DataManager,
    sparam: SParameter,
    dparam: DParameter,
    calc: Calculator,
    hist: History,
    canvas: FractalCanvas,
    show_load_modal: bool,
    show_from_text_modal: bool,
    from_text_text: String,
}

impl FractalApp {
    fn redraw(&mut self) {
        self.hist.push(self.sparam.to_text());
        self.canvas
            .resize(self.sparam.image_size_x(), self.sparam.image_size_y());
        self.calc.reset(&self.sparam);
    }
}

impl Application for FractalApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let sparam = SParameter::new();
        let dparam = DParameter::default();
        let hist = History::init(sparam.to_text());
        let calc = Calculator::new(&sparam, Arc::clone(&dparam.elapsed));
        let canvas = FractalCanvas::new(calc.image_size());
        (
            FractalApp {
                data_mgr: DataManager::new(),
                sparam,
                dparam,
                calc,
                hist,
                canvas,
                show_load_modal: false,
                show_from_text_modal: false,
                from_text_text: "".to_string(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Fractal Viewer")
    }

    // every - feature "smol"
    fn subscription(&self) -> Subscription<Message> {
        let tick = iced::time::every(Duration::from_millis(100)).map(Message::Tick);
        let event = subscription::events_with(|event, _| match event {
            Event::Window(event) => match event {
                window::Event::CloseRequested => Some(Message::WindowClose),
                _ => None,
            },
            _ => None,
        });
        Subscription::batch(vec![tick, event])
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::F(s) => self.sparam.set_f(s),
            Message::G(s) => self.sparam.g = s,
            Message::H(s) => self.sparam.h = s,
            Message::Thresh(s) => self.sparam.thresh = s,
            Message::Fractal(fractal) => {
                self.sparam.fractal_type = fractal;
                self.sparam.reset();
                self.redraw();
            }
            Message::MinX(s) => self.sparam.set_min_x(s),
            Message::MaxX(s) => self.sparam.set_max_x(s),
            Message::MinY(s) => self.sparam.set_min_y(s),
            Message::MaxY(s) => self.sparam.set_max_y(s),
            Message::UnitX(s) => self.sparam.set_unit_x(s),
            Message::UnitY(s) => self.sparam.set_unit_y(s),
            Message::ExpandUnit(factor) => {
                self.sparam.mul_unit(factor);
                self.redraw();
            }
            Message::MaxIter(s) => self.sparam.max_iter = s,
            Message::ExpandIter(factor, divide) => {
                self.sparam.mul_max_iter(factor, divide);
                self.redraw();
            }
            Message::ExpandAtCenter(factor) => {
                self.calc.expand_at_center(factor, &mut self.sparam);
                self.redraw();
            }
            Message::ExpandAtPoint(pos, factor) => {
                self.calc.expand_at_point(&pos, factor, &mut self.sparam);
                self.redraw();
            }
            Message::ColorType(color_type) => {
                self.sparam.color_type = color_type;
                self.redraw();
            }
            Message::ColorVar(color_var) => {
                self.sparam.color_var = color_var;
                self.redraw();
            }
            Message::NWorkers(s) => {
                self.sparam.n_workers = s;
                self.redraw();
            }
            Message::Undo => {
                if let Some(text) = self.hist.undo() {
                    self.sparam.from_text(text);
                    self.redraw();
                }
            }
            Message::Redo => {
                if let Some(text) = self.hist.redo() {
                    self.sparam.from_text(text);
                    self.redraw();
                }
            }
            Message::Stop => self.calc.stop_building(),
            Message::Draw => self.redraw(),
            Message::Png => self.calc.png(),
            Message::Print => {
                println!("Print ...");
            }
            Message::Load(tf) => self.show_load_modal = tf,
            Message::FromData(index) => {
                self.sparam.from_text(self.data_mgr.get(index));
                self.show_load_modal = false;
                self.redraw();
            }
            Message::RemoveData(index) => self.data_mgr.remove(index),
            Message::Save => {
                self.data_mgr.push(&self.sparam.to_text());
            }
            Message::FromText(tf) => self.show_from_text_modal = tf,
            Message::InputFromText(s) => self.from_text_text = s,
            Message::SubmitFromText => {
                self.sparam.from_text(&self.from_text_text);
                self.redraw();
                self.from_text_text = "".to_string();
                self.show_from_text_modal = false;
            }
            Message::ToText => {
                let text = self.sparam.to_text();
                cli_clipboard::set_contents(text).unwrap();
            }
            Message::Reset => {
                self.sparam.reset();
                self.redraw();
            }
            Message::Tick(_) => {
                self.canvas.update_handle(self.calc.image_handle());
            }
            Message::CursorMoved(pos, in_image) => {
                let unshift_pos = self.canvas.cursor_moved(&pos);
                self.dparam.set_xy(&self.calc.pixel_coord(&unshift_pos));
                if in_image {
                    self.dparam.set_color(self.calc.pixel_color(&unshift_pos));
                    self.dparam.set_iter(self.calc.pixel_iter(&unshift_pos));
                }
            }
            Message::CenterTo(pos) => {
                self.calc.center_to(&pos, &mut self.sparam);
                self.redraw();
            }
            Message::MouseButtonPressed(pos) => self.canvas.mouse_button_pressed(pos),
            Message::MouseButtonReleased(pos) => {
                if let Some(p) = self.canvas.mouse_button_released(&pos) {
                    self.calc.translate(&p, &mut self.sparam);
                    self.redraw();
                }
            }
            Message::WindowClose => {
                self.calc.stop_building();
                self.data_mgr.write_to_file();
                return window::close();
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let top = row![
            text("  F "),
            text_input("F", &self.sparam.get_f())
                .width(200)
                .padding(Padding::from([1, 3]))
                .on_input(|s| Message::F(s))
                .on_submit(Message::Draw),
            text("  G "),
            text_input("G", &self.sparam.g)
                .width(120)
                .padding(Padding::from([1, 3]))
                .on_input(|s| Message::G(s))
                .on_submit(Message::Draw),
            text("  H "),
            if self.sparam.fractal_type == FractalType::Mandelbrot {
                text_input("H", &self.sparam.h)
                    .width(120)
                    .padding(Padding::from([1, 3]))
                    .on_input(|s| Message::H(s))
                    .on_submit(Message::Draw)
            } else {
                text_input("H", &self.sparam.h)
                    .width(120)
                    .padding(Padding::from([1, 3]))
            },
            text("  Thresh "),
            text_input("Thresh", &self.sparam.thresh)
                .width(120)
                .padding(Padding::from([1, 3]))
                .on_input(|s| Message::Thresh(s))
                .on_submit(Message::Draw),
            text("  X: "),
            text(&self.dparam.x).width(100),
            text("  Y: "),
            text(&self.dparam.y).width(100),
            text("  Color: "),
            text(&self.dparam.color).width(70),
            text("  Iter: "),
            text(&self.dparam.iter).width(70),
            text("  Elapsed: "),
            text(&self.dparam.elapsed.lock().unwrap()).width(70),
        ]
        .align_items(Alignment::Center);

        let left = column![
            row![
                column![
                    radio(
                        "Mandelbrot Set",
                        FractalType::Mandelbrot,
                        Some(self.sparam.fractal_type),
                        |fractal| Message::Fractal(fractal),
                    )
                    .spacing(5)
                    .size(15),
                    radio(
                        "Julia Set",
                        FractalType::Julia,
                        Some(self.sparam.fractal_type),
                        |fractal| Message::Fractal(fractal),
                    )
                    .spacing(5)
                    .size(15),
                    radio(
                        "Newton Method",
                        FractalType::Newton,
                        Some(self.sparam.fractal_type),
                        |fractal| Message::Fractal(fractal),
                    )
                    .spacing(5)
                    .size(15),
                ]
                .spacing(5)
                .align_items(Alignment::Start),
                horizontal_space(75)
            ],
            row![
                text("Range X "),
                column![
                    text_input("Range X Min", self.sparam.get_min_x())
                        .line_height(0.8)
                        .on_input(|s| Message::MinX(s))
                        .on_submit(Message::Draw),
                    text_input("Range X Max", self.sparam.get_max_x())
                        .line_height(0.8)
                        .on_input(|s| Message::MaxX(s))
                        .on_submit(Message::Draw)
                ]
                .width(200)
            ]
            .align_items(Alignment::Center),
            row![
                text("Range Y "),
                column![
                    text_input("Range Y Min", self.sparam.get_min_y())
                        .line_height(0.8)
                        .on_input(|s| Message::MinY(s))
                        .on_submit(Message::Draw),
                    text_input("Range Y Max", self.sparam.get_max_y())
                        .line_height(0.8)
                        .on_input(|s| Message::MaxY(s))
                        .on_submit(Message::Draw),
                ]
                .width(200),
            ]
            .align_items(Alignment::Center),
            row![
                text("Image Size "),
                column![
                    text(format!(" {}", self.sparam.image_size_x)),
                    text(format!(" {}", self.sparam.image_size_y)),
                ]
                .width(200),
            ]
            .align_items(Alignment::Center),
            row![
                text("Unit "),
                column![
                    text_input("Unit X", self.sparam.get_unit_x())
                        .padding(Padding::from([1, 3]))
                        .on_input(|s| Message::UnitX(s))
                        .on_submit(Message::Draw),
                    text_input("Unit Y", self.sparam.get_unit_y())
                        .padding(Padding::from([1, 3]))
                        .on_input(|s| Message::UnitY(s))
                        .on_submit(Message::Draw),
                ]
                .width(200),
            ]
            .align_items(Alignment::Center),
            {
                let mut row = row![];
                for (&c, &f) in UNIT_EXPAND_CAPTIONS.iter().zip(UNIT_EXPAND_FACTORS.iter()) {
                    row = row.push(
                        button(c)
                            .padding(Padding::from([1, 10]))
                            .on_press(Message::ExpandUnit(f)),
                    );
                }
                row.spacing(3)
            },
            row![
                text("Max Iter "),
                text_input("Max Iter", &self.sparam.max_iter)
                    .width(200)
                    .padding(Padding::from([1, 3]))
                    .on_input(|s| Message::MaxIter(s))
                    .on_submit(Message::Draw),
            ],
            {
                let mut row = row![];
                for (i, &c) in ITER_EXPAND_CAPTIONS.iter().enumerate() {
                    row = row.push(
                        button(c)
                            .padding(Padding::from([1, 10]))
                            .on_press(Message::ExpandIter(ITER_EXPAND_FACTORS[i], i % 2 == 1)),
                    );
                }
                row.spacing(3)
            },
            row![text("Expand at th Center"), horizontal_space(Length::Fill)],
            {
                let mut row = row![];
                for (&c, &f) in IMAGE_EXPAND_CAPTIONS
                    .iter()
                    .zip(IMAGE_EXPAND_FACTORS.iter())
                {
                    row = row.push(
                        button(c)
                            .padding(Padding::from([1, 10]))
                            .on_press(Message::ExpandAtCenter(f)),
                    );
                }
                row.spacing(3)
            },
            row![text("Color Type"), horizontal_space(Length::Fill)],
            {
                let mut col = column![];
                let mut row = row![];
                for (i, c) in COLOR_TYPE.iter().enumerate() {
                    row = row.push(
                        radio(
                            c.to_string(),
                            ColorType::new(c),
                            Some(self.sparam.color_type),
                            |color_type| Message::ColorType(color_type),
                        )
                        .spacing(2)
                        .size(15)
                        .width(45),
                    );
                    if i % 6 == 5 {
                        col = col.push(row.spacing(5));
                        row = row![];
                    }
                }
                if COLOR_TYPE.len() % 6 != 0 {
                    col = col.push(row.spacing(5))
                }
                col.spacing(3)
            },
            row![text("Color Variant"), horizontal_space(Length::Fill)],
            {
                let mut col = column![];
                let mut row = row![];
                for (i, c) in COLOR_VAR.iter().enumerate() {
                    row = row.push(
                        radio(
                            c.to_string(),
                            ColorVar::new(c),
                            Some(self.sparam.color_var),
                            |color_var| Message::ColorVar(color_var),
                        )
                        .spacing(3)
                        .size(15)
                        .width(55),
                    );
                    if i % 6 == 5 {
                        col = col.push(row.spacing(5));
                        row = row![];
                    }
                }
                if COLOR_VAR.len() % 6 != 0 {
                    col = col.push(row.spacing(5))
                }
                col.spacing(3)
            },
            row![
                text("N Workers "),
                text_input("N Workers", &self.sparam.n_workers)
                    .width(200)
                    .padding(Padding::from([1, 3]))
                    .on_input(|s| Message::NWorkers(s))
                    .on_submit(Message::Draw),
            ],
            row![
                button("Undo")
                    .width(140)
                    .padding(Padding::from([1, 50]))
                    .on_press(Message::Undo),
                horizontal_space(Length::Fill),
                button("Redo")
                    .width(140)
                    .padding(Padding::from([1, 50]))
                    .on_press(Message::Redo),
            ],
            row![
                button("Stop")
                    .width(140)
                    .padding(Padding::from([1, 50]))
                    .on_press(Message::Stop),
                horizontal_space(Length::Fill),
                button("Draw")
                    .width(140)
                    .padding(Padding::from([1, 50]))
                    .on_press(Message::Draw),
            ],
            row![
                button("PNG")
                    .width(140)
                    .padding(Padding::from([1, 50]))
                    .on_press(Message::Png),
                horizontal_space(Length::Fill),
                button("Print")
                    .width(140)
                    .padding(Padding::from([1, 50]))
                    .on_press(Message::Print),
            ],
            row![
                button("Load")
                    .width(140)
                    .padding(Padding::from([1, 50]))
                    .on_press(Message::Load(true)),
                horizontal_space(Length::Fill),
                button("Save")
                    .width(140)
                    .padding(Padding::from([1, 50]))
                    .on_press(Message::Save),
            ],
            row![
                button("From Text")
                    .width(140)
                    .padding(Padding::from([1, 30]))
                    .on_press(Message::FromText(true)),
                horizontal_space(Length::Fill),
                button("To Text")
                    .width(140)
                    .padding(Padding::from([1, 40]))
                    .on_press(Message::ToText),
            ],
            row![
                button("Reset")
                    .width(140)
                    .padding(Padding::from([1, 50]))
                    .on_press(Message::Reset),
                horizontal_space(Length::Fill),
            ]
        ]
        .padding(Padding::from([3, 3, 0, 5])) // related with OFFSET of canvas
        .width(305) // related with OFFSET of canvas
        .spacing(3)
        .align_items(Alignment::End);

        let canvas = canvas(&self.canvas)
            .width(Length::Fill)
            .height(Length::Fill);
        let content = container(column![
            top,
            vertical_space(5),
            row![left, horizontal_space(5), column![canvas]],
        ]);
        if self.show_load_modal {
            let modal = container({
                let mut col = column![];
                for (i, line) in self.data_mgr.iter().enumerate() {
                    col = col.push(row![
                        button(line.as_str())
                            .padding([1, 1, 1, 1])
                            .style(theme::Button::Secondary)
                            .on_press(Message::FromData(i)),
                        horizontal_space(3),
                        button("X")
                            .padding([1, 1, 1, 1])
                            .on_press(Message::RemoveData(i))
                    ]);
                }
                col.push(
                    button("Cancel")
                        .padding([1, 1, 1, 1])
                        .on_press(Message::Load(false)),
                )
                .spacing(3)
            })
            .padding(10)
            .style(theme::Container::Box);
            Modal::new(content, modal).into()
        } else if self.show_from_text_modal {
            let modal = container(
                column![
                    text_input("text", &self.from_text_text)
                        .padding([3, 1])
                        .on_input(|s| Message::InputFromText(s))
                        .on_submit(Message::SubmitFromText),
                    row![
                        button("Cancel")
                            .padding([1, 1, 1, 1])
                            .on_press(Message::FromText(false)),
                        horizontal_space(5),
                        button("Load")
                            .padding([1, 1, 1, 1])
                            .on_press(Message::SubmitFromText),
                    ]
                ]
                .spacing(3),
            )
            .padding(10)
            .style(theme::Container::Box);
            Modal::new(content, modal).into()
        } else {
            content.into()
        }
    }
}
