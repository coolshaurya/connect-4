use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum Player {
    A,
    B,
}

impl Player {
    fn opposite(&self) -> Self {
        match self {
            Self::A => Self::B,
            Self::B => Self::A,
        }
    }

    /*
        fn to_color(&self) -> Color {
            match self {
                Self::A => Color::from_rgb8(200, 30, 50),
                Self::B => Color::from_rgb8(150, 80, 35),
            }
        }
    */
}

impl Default for Player {
    fn default() -> Self {
        Self::A
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::A => write!(f, "Player A (Red)"),
            Self::B => write!(f, "Player B (Yellow)"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum GameResult {
    Indefinite,
    Win(Player),
    Draw,
}

#[derive(Clone, Copy, Debug)]
pub enum PieceSpot {
    Empty,
    Player(Player),
}

impl Default for PieceSpot {
    fn default() -> Self {
        Self::Empty
    }
}

impl PieceSpot {
    fn is_empty(&self) -> bool {
        match self {
            Self::Empty => true,
            _ => false,
        }
    }
    fn is_player(&self) -> bool {
        match self {
            Self::Player(_) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Board {
    pub turn: Player,
    pub board: [[PieceSpot; 6]; 7],
}

impl Board {
    fn reset(mut self) {
        self = Self::default();
    }

    fn drop_piece(&mut self, column: usize) {
        let column_top = self.board[column].iter().filter(|&x| x.is_player()).count();
        if column >= 7 || column_top >= 6 {
            panic!("invalid column");
        }
        self.board[column][column_top] = PieceSpot::Player(self.turn);
    }

    fn switch_turn(&mut self) {
        self.turn = self.turn.opposite();
    }

    fn win_sequences(&self) -> Vec<Vec<(usize, usize)>> {
        let is_valid = |column: usize, row: usize| -> bool { column < 7 && row < 6 };

        let rows = (0..6).map(|row| {
            (0..7)
                .map(|column| (column, row))
                .collect::<Vec<(usize, usize)>>()
        });

        let columns = (0..7).map(|column| {
            (0..6)
                .map(|row| (column, row))
                .collect::<Vec<(usize, usize)>>()
        });

        let diag1 = [(0, 2), (0, 1), (0, 0), (1, 0), (2, 0), (3, 0)]
            .iter()
            .map(|&start| {
                let mut vec = vec![start];
                let mut row = start.1 + 1;
                let mut column = start.0 + 1;
                while is_valid(column, row) {
                    vec.push((column, row));
                    row += 1;
                    column += 1;
                }
                vec
            });
        let diag2 = [(0, 3), (0, 4), (0, 5), (1, 5), (2, 5), (3, 5)]
            .iter()
            .map(|&start| {
                let mut vec = vec![start];
                let mut row = start.1 - 1;
                let mut column = start.0 + 1;
                while is_valid(column, row) {
                    vec.push((column, row));
                    row = if let Some(row) = row.checked_sub(1) {
                        row
                    } else {
                        break;
                    };

                    column += 1;
                }
                vec
            });

        rows.chain(columns).chain(diag1).chain(diag2).collect()
    }

    fn is_draw(&self) -> bool {
        let empty_count = self
            .board
            .iter()
            .flatten()
            .filter(|option| option.is_empty())
            .count();

        empty_count == 0
    }

    fn is_win(&self) -> bool {
        for sequence in self.win_sequences() {
            for group_of_4 in sequence.windows(4) {
                if group_of_4[0] == group_of_4[1]
                    && group_of_4[1] == group_of_4[2]
                    && group_of_4[2] == group_of_4[3]
                {
                    return true;
                }
            }
        }

        false
    }

    pub fn calculate_result(&self) -> GameResult {
        if self.is_draw() {
            GameResult::Draw
        } else if self.is_win() {
            GameResult::Win(self.turn)
        } else {
            GameResult::Indefinite
        }
    }
}
