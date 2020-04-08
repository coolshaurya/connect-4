use crate::board::{Board, GameResult, PieceSpot, Player};
use crate::colors;
use iced::{
    button, container, Align, Background, Button, Column, Container, Element, HorizontalAlignment,
    Length, Row, Sandbox, Space, Text,
};

const HEADING: &str = "Connect – 4"; // en dash, not hyphen

const INSTRUCTIONS: &str = "\
In this game there are two players, A and B, A having the red pieces and B having the yellow pieces. \
The way you play this game is by dropping a piece in any of the columns. \
The goal of the game is to make any sequence of four pieces horizontally, vertically, or diagonally. \
";

const CIRCLE_SIZE: u16 = 50;
const INTERCOLUMN_SPACING: u16 = 5;

impl container::StyleSheet for PieceSpot {
    fn style(&self) -> container::Style {
        let background_color = match self {
            PieceSpot::Player(Player::A) => colors::PLAYER_A,
            PieceSpot::Player(Player::B) => colors::PLAYER_B,
            PieceSpot::Empty => colors::EMPTY,
        };

        container::Style {
            background: Some(Background::from(background_color)),
            border_radius: CIRCLE_SIZE / 2,
            ..container::Style::default()
        }
    }
}

impl Board {
    pub fn view(&self) -> Element<Message> {
        let circle = || {
            Container::new(Space::new(
                Length::Units(CIRCLE_SIZE),
                Length::Units(CIRCLE_SIZE),
            ))
        };
        let mut row = Row::new().spacing(INTERCOLUMN_SPACING);
        for i in 0..7 {
            let mut column = Column::new().spacing(INTERCOLUMN_SPACING);
            for j in (0..6).rev() {
                column = column.push(circle().style(self.board[i][j]));
            }
            row = row.push(column);
        }
        row.into()
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct ResetButton {
    state: button::State,
}

#[derive(Clone, Copy, Debug)]
struct DropButton {
    column: usize,
    state: button::State,
}

impl DropButton {
    fn new(column: usize) -> Self {
        Self {
            state: button::State::default(),
            column,
        }
    }

    fn new_array() -> [Self; 7] {
        let mut array: [Self; 7] = [Self::new(0); 7];

        for (index, btn) in (0..array.len()).map(|num| (num, Self::new(num))) {
            array[index] = btn;
        }

        array
    }

    fn view(&mut self) -> Element<Message> {
        Button::new(&mut self.state, Text::new("Drop"))
            .width(Length::Units(CIRCLE_SIZE))
            .on_press(Message::PieceDrop(self.column))
            .into()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    PieceDrop(usize),
    Reset,
}

#[derive(Debug, Clone)]
pub struct BoardGui {
    board: Board,
    reset_button: ResetButton,
    drop_buttons: [DropButton; 7],
    game_over: bool,
}

impl Sandbox for BoardGui {
    type Message = Message;

    fn new() -> Self {
        Self {
            board: Board::default(),
            reset_button: ResetButton::default(),
            drop_buttons: DropButton::new_array(),
            game_over: false,
        }
    }

    fn title(&self) -> String {
        String::from(HEADING)
    }

    fn view(&mut self) -> Element<Self::Message> {
        let (result_text, result_size) = match self.board.calculate_result() {
            GameResult::Indefinite => (format!("Turn: {}", self.board.turn), 25),
            GameResult::Draw => (String::from("It's a Draw!"), 80),
            GameResult::Win(player) => (format!("{} Won!", player), 80),
        };

        let reset_button = Button::new(
            &mut self.reset_button.state,
            Text::new("Reset").horizontal_alignment(HorizontalAlignment::Center),
        )
        .width(Length::Units(CIRCLE_SIZE * 7))
        .on_press(Message::Reset);

        Column::<Message>::new()
            .spacing(25)
            .width(Length::Fill)
            .align_items(Align::Center)
            .push(Text::new(HEADING).size(80).color(colors::HEADING))
            .push(Text::new(INSTRUCTIONS).width(Length::Units(600)))
            .push(Text::new(result_text).size(result_size))
            .push(
                Row::with_children(self.drop_buttons.iter_mut().map(|btn| btn.view()).collect())
                    .spacing(INTERCOLUMN_SPACING),
            )
            .push(self.board.view())
            .push(reset_button)
            .into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::PieceDrop(column) => {
                if !self.game_over {
                    let valid_drop = self.board.drop_piece(column);
                    if let GameResult::Indefinite = self.board.calculate_result() {
                        if valid_drop.is_ok() {
                            self.board.switch_turn();
                        }
                    } else {
                        self.game_over = true;
                    }
                }
            }
            Message::Reset => {
                self.board.reset();
                self.game_over = false;
            }
        }
    }
}
