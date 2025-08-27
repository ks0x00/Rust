use std::time::Instant;

use base::{ColorType, FractalType, F, ColorVar};
use iced::Point;

pub const IMAGE_EXPAND_FACTORS: [F; 6] = [2.0, 0.5, 5.0, 0.2, 10.0, 0.1];
pub const IMAGE_EXPAND_CAPTIONS: [&str; 6] = ["x 2", "/ 2", "x 5", "/ 5", "x 10", "/ 10"];

pub const ITER_EXPAND_FACTORS: [usize; 6] = [2, 2, 5, 5, 10, 10];
pub const ITER_EXPAND_CAPTIONS: [&str; 6] = ["x 2", "/ 2", "x 5", "/ 5", "x 10", "/ 10"];

pub const UNIT_EXPAND_FACTORS: [F; 4] = [2.0, 0.5, 3.0, 1.0 / 3.0];
pub const UNIT_EXPAND_CAPTIONS: [&str; 4] = ["x 2", "/ 2", "x 3", "/ 3"];

#[derive(Debug, Clone)]
pub enum Message {
    F(String),
    G(String),
    H(String),
    Thresh(String),
    Fractal(FractalType),
    MinX(String),
    MaxX(String),
    MinY(String),
    MaxY(String),
    UnitX(String),
    UnitY(String),
    ExpandUnit(f64),
    MaxIter(String),
    ExpandIter(usize, bool),
    ExpandAtCenter(f64),
    ExpandAtPoint(Point, f64),
    ColorType(ColorType),
    ColorVar(ColorVar),
    NWorkers(String),
    Undo,
    Redo,
    Stop,
    Draw,
    Png,
    Print,
    Load(bool),
    FromData(usize),
    RemoveData(usize),
    Save,
    FromText(bool),
    InputFromText(String),
    SubmitFromText,
    ToText,
    Reset,
    WindowClose,
    // WindowEvent(window::Event),
    Tick(Instant),
    CenterTo(Point),
    CursorMoved(Point, bool),
    MouseButtonPressed(Point),
    MouseButtonReleased(Point),
}

pub mod canvas;
pub mod data;
pub mod modal;
pub mod history;
