use base::{Pair, F};
use eframe::egui::Pos2;

pub const IMAGE_EXPAND_FACTORS: [F; 6] = [2.0, 0.5, 5.0, 0.2, 10.0, 0.1];
pub const IMAGE_EXPAND_CAPTIONS: [&str; 6] = ["x 2", "/ 2", "x 5", "/ 5", "x 10", "/ 10"];

pub const ITER_EXPAND_FACTORS: [usize; 6] = [2, 2, 5, 5, 10, 10];
pub const ITER_EXPAND_CAPTIONS: [&str; 6] = ["x 2", "/ 2", "x 5", "/ 5", "x 10", "/ 10"];

pub const UNIT_EXPAND_FACTORS: [F; 4] = [2.0, 0.5, 3.0, 1.0 / 3.0];
pub const UNIT_EXPAND_CAPTIONS: [&str; 4] = ["x 2", "/ 2", "x 3", "/ 3"];

pub trait SetWithOffset {
    fn set_with_offset(&mut self, pos: &Pos2, offset: &Pos2);
}

impl SetWithOffset for Pair<usize> {
    fn set_with_offset(&mut self, pos: &Pos2, offset: &Pos2) {
        self.x = (pos.x - offset.x).round() as usize;
        self.y = (pos.y - offset.y).round() as usize;
    }
}

pub mod app;
pub mod data;
