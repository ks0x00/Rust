#![windows_subsystem = "windows"]

use eframe::egui::{self};
use egui_main::gui::{HEIGHT, PuzzleGui, WIDTH};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([WIDTH, HEIGHT]),
        centered: true,
        ..Default::default()
    }; // 프레임 크기 설정
    eframe::run_native(
        "Water Puzzle - egui",
        options,
        Box::new(|_cc| Ok(Box::new(PuzzleGui::load_or_new()))),
    )
}
