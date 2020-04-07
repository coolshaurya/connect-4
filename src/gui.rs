use crate::board::{Board, GameResult, PieceSpot, Player};
use iced::{
    button, container, Align, Background, Button, Column, Container, Element, Length, Row, Sandbox,
    Space, Text,
};

const HEADING: &str = "Connect – 4"; // en dash, not hyphen

const INSTRUCTIONS: &str = "\
In this game there are two players, A and B, A having the red pieces and B having the yellow pieces. \
The way you play this game is by dropping a piece in any of the columns. \
The goal of the game is to make any sequence of four pieces horizontally, vertically, or diagonally. \
";

const CIRCLE_SIZE: u16 = 60;

impl container::StyleSheet for PieceSpot {
    fn style(&self) -> container::Style {
        use crate::colors;

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
    pub fn view(&self) -> Element<()> {
        let circle = || {
            Container::new(Space::new(
                Length::Units(CIRCLE_SIZE),
                Length::Units(CIRCLE_SIZE),
            ))
        };
        let mut row: Row<()> = Row::new().spacing(2);
        for i in 0..7 {
            let mut column = Column::new().spacing(2);
            for j in 0..6 {
                column = column.push(circle().style(self.board[i][j]));
            }
            row = row.push(column);
        }
        row.into()
    }
}

#[derive(Debug, Clone, Default)]
pub struct BoardGui {
    board: Board,
}

#[derive(Clone, Debug)]
struct ResetButton {
    state: button::State,
}

#[derive(Clone, Debug)]
struct DropButton {
    column: usize,
    state: button::State,
}

impl Sandbox for BoardGui {
    type Message = ();

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from(HEADING)
    }

    fn view(&mut self) -> Element<Self::Message> {
        let result = match self.board.calculate_result() {
            GameResult::Indefinite => format!("Turn: {}", self.board.turn),
            GameResult::Draw => String::from("It's a Draw!"),
            GameResult::Win(player) => format!("{} Won!", player),
        };

        Column::new()
            .spacing(25)
            .width(Length::Fill)
            .align_items(Align::Center)
            .push(Text::new(HEADING).size(80).color([0.22, 0.88, 0.45]))
            .push(Text::new(INSTRUCTIONS).width(Length::Units(600)))
            .push(Text::new(result))
            .push(self.board.view())
            .into()
    }

    fn update(&mut self, message: Self::Message) {}
}
