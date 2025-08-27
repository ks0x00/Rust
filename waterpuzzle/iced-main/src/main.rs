#![windows_subsystem = "windows"]

use iced_main::gui::Gui;

pub fn main() -> iced::Result {
    iced::application("Water Puzzle - iced", Gui::update, Gui::view)
        .subscription(Gui::subscription)
        .theme(Gui::theme)
        .centered()
        .run()
}
