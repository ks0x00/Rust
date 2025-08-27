use std::{
    error::Error,
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
};

use eframe::egui::{
    self, Button, CentralPanel, Color32, Grid, Key, PointerButton, RichText, Sense, TopBottomPanel,
};
use puzzle::puzzle::Puzzle;

use crate::viewport::{SPACE_X, Viewport};

pub const WIDTH: f32 = 1024.0;
pub const HEIGHT: f32 = 720.0;
const FILE_NAME: &str = "save.wp";

pub struct PuzzleGui {
    puzzle: Puzzle,
    viewport: Viewport,
    top_panel_height: f32,
    bottom_panel_height: f32,
    button_width: f32,
    button_height: f32,
    info: String,
}

impl PuzzleGui {
    pub fn new(n_rows: usize, n_cols: usize) -> Self {
        Self::from_puzzle_and_viewport(
            Puzzle::random_new(n_rows * n_cols),
            Viewport::new(n_rows, n_cols),
        )
    }

    pub fn from_puzzle_and_viewport(puzzle: Puzzle, viewport: Viewport) -> Self {
        let top_panel_height = 30.0;
        let bottom_panel_height = 60.0;
        Self {
            puzzle,
            viewport,
            top_panel_height,
            bottom_panel_height,
            button_width: 60.0,
            button_height: 30.0,
            info: "".to_string(),
        }
    }
    pub fn load_or_new() -> Self {
        match Self::load() {
            Ok((puzzle, view)) => Self::from_puzzle_and_viewport(puzzle, view),
            Err(_) => Self::new(3, 5),
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let file = File::create(FILE_NAME)?;
        let mut writer = BufWriter::new(file);

        writer.write_all(&(self.viewport.n_rows() as u32).to_le_bytes())?;
        writer.write_all(&(self.viewport.n_cols() as u32).to_le_bytes())?;
        self.puzzle.save(&mut writer)?;
        writer.flush()?;
        Ok(())
    }

    pub fn load() -> Result<(Puzzle, Viewport), Box<dyn Error>> {
        let file = File::open(FILE_NAME)?;
        let mut reader = BufReader::new(file);

        let mut buf = [0; 4];
        reader.read_exact(&mut buf)?;
        let n_rows = u32::from_le_bytes(buf) as usize;

        reader.read_exact(&mut buf)?;
        let n_cols = u32::from_le_bytes(buf) as usize;

        let mut puzzle = Puzzle::empty_new(n_rows * n_cols);
        puzzle.load(&mut reader)?;

        let view = Viewport::new(n_rows, n_cols);
        Ok((puzzle, view))
    }
}

impl eframe::App for PuzzleGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.puzzle.check_solver();
        if ctx.input(|input| input.key_pressed(Key::Z)) {
            self.puzzle.undo();
        } else if ctx.input(|input| input.key_pressed(Key::X)) {
            self.puzzle.redo();
        }
        if self.puzzle.is_completed() {
            self.info = "Completed".to_string();
        } else if !self.puzzle.pouring_results_different_state() {
            self.info = "Cannot move".to_string()
        } else {
            self.info = "".to_string();
        }

        let mut visuals = ctx.style().visuals.clone();
        visuals.panel_fill = Color32::from_rgb(255, 255, 255);
        ctx.set_visuals(visuals);
        TopBottomPanel::top("top_panel")
            .exact_height(self.top_panel_height)
            .show(ctx, |ui| {
                // let width = ui.available_width();
                ui.horizontal_centered(|ui| {
                    ui.add_space(SPACE_X);
                    ui.horizontal(|ui| {
                        ui.label(self.puzzle.step().to_string());
                        let emoticon = RichText::new(self.puzzle.solver_emoticon())
                            .color(Color32::from_rgb(255, 0, 0));
                        ui.label(emoticon);
                        ui.label(self.puzzle.solver_remaining_step());
                        ui.label(&self.info);
                    });
                });
            });
        CentralPanel::default().show(ctx, |ui| {
            let size = ui.available_size();
            let (_response, painter) = ui.allocate_painter(size, Sense::click());
            ui.input(|input| {
                if input.pointer.button_pressed(PointerButton::Primary) {
                    let click_pos = input.pointer.interact_pos();
                    if let Some(pos) = click_pos {
                        self.viewport.on_left_click(&pos, &mut self.puzzle);
                    }
                } else if input.pointer.button_pressed(PointerButton::Secondary)
                    || input.pointer.button_pressed(PointerButton::Middle)
                {
                    let click_pos = input.pointer.interact_pos();
                    if let Some(pos) = click_pos {
                        self.viewport.on_right_click(&pos, &mut self.puzzle);
                    }
                }
            });
            self.viewport.resize(size[0], size[1]);
            self.viewport.draw_puzzle(&painter, &self.puzzle);
        });
        TopBottomPanel::bottom("bottom_panel")
            .exact_height(self.bottom_panel_height)
            .show(ctx, |ui| {
                let width = ui.available_width();
                ui.add_space(5.0);
                Grid::new("bottom buttons")
                    .min_col_width(width / 2.0)
                    .num_columns(2)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.add_space(SPACE_X);
                            if ui
                                .add_sized(
                                    [self.button_width, self.button_height],
                                    Button::new("New game"),
                                )
                                .clicked()
                            {
                                self.puzzle.reset();
                            }
                            if ui
                                .add_sized(
                                    [self.button_width, self.button_height],
                                    Button::new("Undo all"),
                                )
                                .clicked()
                            {
                                self.puzzle.undo_all();
                            }
                            if ui
                                .add_sized(
                                    [self.button_width, self.button_height],
                                    Button::new("Undo(Z)"),
                                )
                                .clicked()
                            {
                                self.puzzle.undo();
                            }
                            if ui
                                .add_sized(
                                    [self.button_width, self.button_height],
                                    Button::new("Redo(X)"),
                                )
                                .clicked()
                            {
                                self.puzzle.redo();
                            }
                        });
                        ui.horizontal(|ui| {
                            if ui
                                .add_sized(
                                    [self.button_width, self.button_height],
                                    Button::new("Hint"),
                                )
                                .clicked()
                            {
                                self.puzzle.reset_solver_hint();
                            }
                        });
                        ui.end_row();
                    });
            });
        // 0.1초 (100ms) 후에 UI 갱신을 요청합니다.
        // 이것이 UI를 "주기적으로 업데이트"하는 방법입니다.
        ctx.request_repaint_after(std::time::Duration::from_millis(100));
    }
}

impl Drop for PuzzleGui {
    fn drop(&mut self) {
        if let Err(e) = self.save() {
            eprintln!("저장 오류: {}", e);
        }
    }
}
