use std::{
    error::Error,
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
};

use iced::{
    Element, Event, Length, Point, Rectangle, Renderer, Size, Subscription, Theme, alignment,
    event, keyboard, mouse,
    widget::{Space, button, canvas, column, row, text},
    window,
};
use puzzle::puzzle::Puzzle;

use crate::{
    RED,
    viewport::{OFFSET_X, SPACE_X, Viewport},
};

const HINT_ENABLED: bool = false;

// const WIDTH: f32 = 800.0;
// const HEIGHT: f32 = 640.0;
const TOP_PANEL_HEIGHT: f32 = 40.0;
const BOTTOM_PANEL_HEIGHT: f32 = 80.0;
#[allow(dead_code)]
const BUTTON_WIDTH: f32 = 60.0;
#[allow(dead_code)]
const BUTTON_HEIGHT: f32 = 30.0;
#[allow(dead_code)]
const SMALL_ROW_WIDTH: f32 = 20.0;

const FILE_NAME: &str = "save.wp";

#[derive(Debug, Clone)]
pub enum Message {
    // WindowCloseRequested,
    WindowResized(Size),
    NewGame,
    UndoAll,
    Undo,
    Redo,
    Hint,
    LButtonReleased(Point),
    RButtonReleased(Point),
    KeyPressed(keyboard::key::Physical),
    Tick,
}

pub struct Gui {
    width: f32,
    puzzle: Puzzle,
    viewport: Viewport,
    info: String,
    cache: canvas::Cache,
}

impl Default for Gui {
    fn default() -> Self {
        Self::load_or_new()
    }
}

impl Gui {
    pub fn new(n_rows: usize, n_cols: usize) -> Self {
        Self::from_puzzle_and_viewport(
            Puzzle::random_new(n_rows * n_cols),
            Viewport::new(n_rows, n_cols),
        )
    }

    pub fn from_puzzle_and_viewport(puzzle: Puzzle, viewport: Viewport) -> Self {
        Self {
            width: 0.0, // anything
            puzzle,
            viewport,
            info: "".to_string(),
            cache: canvas::Cache::new(),
        }
    }
    pub fn load_or_new() -> Self {
        match Self::load() {
            Ok((puzzle, view)) => Self::from_puzzle_and_viewport(puzzle, view),
            Err(_) => Self::new(3, 5),
        }
    }

    pub fn canvas_height(&self, total_height: f32) -> f32 {
        total_height - TOP_PANEL_HEIGHT - BOTTOM_PANEL_HEIGHT
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

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            // Listen for window events (resize, keyboard)
            event::listen_with(|event, _status, _id| match event {
                Event::Window(window::Event::Resized(size)) => Some(Message::WindowResized(size)),
                Event::Keyboard(keyboard_event) => match keyboard_event {
                    keyboard::Event::KeyPressed { physical_key, .. } => {
                        Some(Message::KeyPressed(physical_key))
                    }
                    _ => None,
                },
                _ => None,
            }),
            // Periodic tick for updates (e.g., solver animation, info text update)
            // --features tokio
            iced::time::every(std::time::Duration::from_millis(100)).map(|_| Message::Tick),
        ])
    }

    pub fn theme(&self) -> Theme {
        Theme::Light
    }

    // fn close_requested(&self) -> Command<Message> {
    //     // Save game state when close is requested
    //     let _ = self.save(); // Perform save synchronously or asynchronously
    //     Command::perform(async {}, |_| Message::NewGame) // Placeholder for actual save command
    //                                                     // In a real app, you might want to return Command::perform(async { self.save() }, |res| Message::Saved(res))
    //                                                     // and then handle Message::Saved to exit.
    //                                                     // For now, we just save and then exit.
    // }

    pub fn update(&mut self, message: Message) {
        // Invalidate canvas cache on any state change that affects drawing
        self.cache.clear();

        match message {
            // Message::WindowCloseRequested => {let _= self.save();},
            Message::WindowResized(size) => {
                self.width = size.width;
                self.viewport
                    .resize(size.width, self.canvas_height(size.height))
            }
            Message::NewGame => self.puzzle.reset(),
            Message::UndoAll => self.puzzle.undo_all(),
            Message::Undo => self.puzzle.undo(),
            Message::Redo => self.puzzle.redo(),
            Message::Hint => self.puzzle.reset_solver_hint(),
            // Message::Hint => self.puzzle.apply_solver_hint(),
            Message::LButtonReleased(point) => {
                self.viewport.on_left_click(&point, &mut self.puzzle)
            }
            Message::RButtonReleased(point) => {
                self.viewport.on_right_click(&point, &mut self.puzzle)
            }
            Message::KeyPressed(key) => {
                if key == keyboard::key::Physical::Code(keyboard::key::Code::KeyZ) {
                    self.puzzle.undo();
                } else if key == keyboard::key::Physical::Code(keyboard::key::Code::KeyX) {
                    self.puzzle.redo();
                }
            }
            Message::Tick => {
                self.puzzle.check_solver();
                if self.puzzle.is_completed() {
                    self.info = "Completed".to_string();
                } else if !self.puzzle.pouring_results_different_state() {
                    self.info = "Cannot move".to_string();
                } else {
                    self.info = "".to_string();
                }
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let row_width_half = (self.width - OFFSET_X) / 2.0;
        column![
            row![
                Space::with_width(OFFSET_X),
                row![
                    text(self.puzzle.step()),
                    text(format!("  {}", self.puzzle.solver_emoticon())).color(RED),
                    text(format!(
                        "  {}  {}",
                        self.puzzle.solver_remaining_step(),
                        self.info
                    ))
                ] // .width(row_width_half),
            ]
            .height(TOP_PANEL_HEIGHT)
            .align_y(alignment::Vertical::Center),
            canvas(self).width(Length::Fill).height(Length::Fill),
            row![
                Space::with_width(OFFSET_X),
                row![
                    button("New game").on_press(Message::NewGame),
                    Space::with_width(SPACE_X),
                    button("Undo all").on_press(Message::UndoAll),
                    Space::with_width(SPACE_X),
                    button("Undo(Z)").on_press(Message::Undo),
                    Space::with_width(SPACE_X),
                    button("Redo(X)").on_press(Message::Redo),
                ]
                .width(row_width_half),
                if HINT_ENABLED {
                    button("Hint").on_press(Message::Hint)
                } else {
                    button("Hint")
                }
            ]
            .height(BOTTOM_PANEL_HEIGHT)
            .align_y(alignment::Vertical::Top)
        ]
        .into()
    }
}

impl canvas::Program<Message> for Gui {
    // No internal state
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        _event: canvas::Event,
        _bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (canvas::event::Status, Option<Message>) {
        type Status = canvas::event::Status;
        match _event {
            canvas::Event::Mouse(event) => match event {
                mouse::Event::ButtonReleased(button) => match button {
                    mouse::Button::Left => match cursor.position() {
                        Some(mut pos) => {
                            pos.y -= TOP_PANEL_HEIGHT;
                            (Status::Captured, Some(Message::LButtonReleased(pos)))
                        }
                        None => (Status::Ignored, None),
                    },
                    mouse::Button::Right | mouse::Button::Middle => match cursor.position() {
                        Some(mut pos) => {
                            pos.y -= TOP_PANEL_HEIGHT;
                            (Status::Captured, Some(Message::RButtonReleased(pos)))
                        }
                        None => (Status::Ignored, None),
                    },
                    _ => (Status::Ignored, None),
                },
                _ => (Status::Ignored, None),
            },
            _ => (Status::Ignored, None),
        }
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            self.viewport.draw_puzzle(frame, &self.puzzle);
        });

        vec![geometry]
    }
}

impl Drop for Gui {
    fn drop(&mut self) {
        if let Err(e) = self.save() {
            eprintln!("저장 오류: {}", e);
        }
    }
}
