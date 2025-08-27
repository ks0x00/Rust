#![windows_subsystem = "windows"]

use eframe::epaint::vec2;
use viewer::app::FractalApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(vec2(1320.0, 760.0)),
        centered: true,
        ..Default::default()
    }; // 프레임 크기 설정
    eframe::run_native(
        "Fractal Viewer",
        options,
        Box::new(|_cc| Box::new(FractalApp::new())),
    )
}