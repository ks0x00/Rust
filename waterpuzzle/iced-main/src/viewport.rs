use iced::{Point, Rectangle, Size, alignment, widget::canvas};
use puzzle::puzzle::Puzzle;
use state::{MAX_WATERS, beaker::NO_WATER};

use crate::*;

pub const OFFSET_X: f32 = 30.0;
const OFFSET_Y: f32 = 30.0;

pub const SPACE_X: f32 = 20.0;
const SPACE_Y: f32 = 30.0;

const NORMAL_ALPHA: f32 = 0.95;
const HINT_DEST_ALPHA: f32 = 0.5;

#[derive(Default)]
pub struct Viewport {
    n_rows: usize,
    n_cols: usize,

    beaker_width: f32,
    beaker_height: f32,
    water_height: f32,
    selected_shift: f32,
}

impl Viewport {
    pub fn new(n_rows: usize, n_cols: usize) -> Self {
        Self {
            n_rows,
            n_cols,
            beaker_width: 0.0,
            beaker_height: 0.0,
            water_height: 0.0,
            selected_shift: 0.0,
        }
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        let net_width = width - 2.0 * OFFSET_X - (self.n_cols - 1) as f32 * SPACE_X;
        self.beaker_width = net_width / self.n_cols as f32;
        let net_height = height - OFFSET_Y - self.n_rows as f32 * SPACE_Y;
        self.beaker_height = (net_height / self.n_rows as f32).floor();
        self.water_height = (self.beaker_height / MAX_WATERS as f32).floor();
        self.selected_shift = (self.water_height / 2.0).floor();
    }

    pub fn n_rows(&self) -> usize {
        self.n_rows
    }
    pub fn n_cols(&self) -> usize {
        self.n_cols
    }

    pub fn row(&self, index: usize) -> usize {
        index / self.n_cols
    }
    pub fn col(&self, index: usize) -> usize {
        index % self.n_cols
    }
    pub fn row_col(&self, index: usize) -> (usize, usize) {
        (index / self.n_cols, index % self.n_cols)
    }
    pub fn index(&self, row: usize, col: usize) -> usize {
        row * self.n_cols + col
    }

    pub fn beaker_x(&self, col: usize) -> f32 {
        OFFSET_X + col as f32 * (self.beaker_width + SPACE_X)
    }
    pub fn beaker_y(&self, row: usize) -> f32 {
        OFFSET_Y + row as f32 * (self.beaker_height + SPACE_Y)
    }
    pub fn beaker_xy(&self, row: usize, col: usize) -> (f32, f32) {
        (self.beaker_x(col), self.beaker_y(row))
    }
    pub fn beaker_pos(&self, row: usize, col: usize) -> Point {
        Point::new(self.beaker_x(col), self.beaker_y(row))
    }
    pub fn beaker_size(&self) -> Size {
        Size::new(self.beaker_width, self.beaker_height)
    }
    pub fn beaker_rect(&self, row: usize, col: usize) -> Rectangle {
        Rectangle::new(
            self.beaker_pos(row, col),
            Size::new(self.beaker_width, self.beaker_height),
        )
    }

    pub fn beaker_contains(&self, row: usize, col: usize, p: &Point) -> bool {
        let x = self.beaker_x(col);
        let y = self.beaker_y(row);
        x < p.x && p.x < x + self.beaker_width && y < p.y && p.y < y + self.beaker_height
    }

    pub fn water_x(&self, col: usize) -> f32 {
        self.beaker_x(col)
    }
    pub fn water_y(&self, row: usize, water_index: usize) -> f32 {
        self.beaker_y(row) + (MAX_WATERS - water_index - 1) as f32 * self.water_height
    }
    pub fn water_xy(&self, row: usize, col: usize, water_index: usize) -> (f32, f32) {
        (self.beaker_x(col), self.water_y(row, water_index))
    }
    pub fn water_pos(&self, row: usize, col: usize, water_index: usize) -> Point {
        Point::new(self.beaker_x(col), self.water_y(row, water_index))
    }
    pub fn water_size(&self) -> Size {
        Size::new(self.beaker_width, self.water_height)
    }
    pub fn water_rect(&self, row: usize, col: usize, water_index: usize) -> Rectangle {
        Rectangle::new(
            self.water_pos(row, col, water_index),
            Size::new(self.beaker_width, self.water_height),
        )
    }

    pub fn index_of_beaker_which_contains(&self, p: &Point) -> Option<usize> {
        for row in 0..self.n_rows {
            for col in 0..self.n_cols {
                if self.beaker_contains(row, col, p) {
                    return Some(row * self.n_cols + col);
                }
            }
        }
        None
    }

    pub fn on_right_click(&self, p: &Point, puzzle: &mut Puzzle) {
        puzzle.on_right_click(self.index_of_beaker_which_contains(p));
    }

    pub fn on_left_click(&self, p: &Point, puzzle: &mut Puzzle) {
        if let Some(clicked_beaker_index) = self.index_of_beaker_which_contains(p) {
            puzzle.on_left_click(clicked_beaker_index);
        }
    }

    pub fn draw_beaker(&self, frame: &mut canvas::Frame, puzzle: &Puzzle, beaker_index: usize) {
        let beaker = puzzle.beaker(beaker_index);
        let selected = puzzle.is_selected_beaker_index(beaker_index);
        let alpha = if puzzle.is_hint_dst_beaker_index(beaker_index) {
            HINT_DEST_ALPHA
        } else {
            NORMAL_ALPHA
        };

        let row = self.row(beaker_index);
        let col = self.col(beaker_index);
        let water_size = self.water_size();
        for i in 0..MAX_WATERS {
            let mut water_pos = self.water_pos(row, col, i);
            if selected {
                water_pos.y -= self.selected_shift;
            }
            let water = beaker.water(i);
            if water != NO_WATER {
                // let color = if hint_dst {COLORS[water as usize].scale_alpha(0.5)} else {COLORS[water as usize]};
                frame.fill_rectangle(
                    water_pos,
                    water_size,
                    canvas::Fill::from(COLORS[water as usize].scale_alpha(alpha)),
                );
                // Draw text for water index
                frame.fill_text(canvas::Text {
                    content: water.to_string(),
                    position: Point::new(
                        water_pos.x + self.beaker_width / 2.0,
                        water_pos.y + self.water_height / 2.0,
                    ),
                    color: BLACK,
                    size: iced::Pixels(20.0), // Font size
                    horizontal_alignment: alignment::Horizontal::Center, // Align2::CENTER_CENTER
                    vertical_alignment: alignment::Vertical::Center,
                    ..Default::default()
                });
            }
            frame.stroke_rectangle(
                water_pos,
                water_size,
                canvas::Stroke::default()
                    .with_color(BLACK.scale_alpha(alpha))
                    .with_width(2.0),
            );
        }
        ///////////////////////////////////////////////////////////////////
        // beaker rect
        let mut beaker_pos = self.beaker_pos(row, col);
        if selected {
            beaker_pos.y -= self.selected_shift;
        }
        frame.stroke_rectangle(
            beaker_pos,
            self.beaker_size(),
            canvas::Stroke::default().with_color(BLACK.scale_alpha(alpha)),
        );
    }

    pub fn draw_puzzle(&self, frame: &mut canvas::Frame, puzzle: &Puzzle) {
        for i in 0..puzzle.n_beakers() {
            self.draw_beaker(frame, puzzle, i);
        }
    }
}
