/// The game "board" that has the pieces in it
mod board;

/// Some color constants
mod colors;

/// The gui that is drawn
mod gui;

use gui::BoardGui;
use iced::{Sandbox, Settings};

fn main() {
    BoardGui::run(Settings::default());
}
