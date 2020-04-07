mod board;
mod colors;
mod gui;

use gui::BoardGui;
use iced::{Sandbox, Settings};

fn main() {
    BoardGui::run(Settings::default());
}
