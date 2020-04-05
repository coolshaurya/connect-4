#[derive(Clone, Copy, Debug)]
enum Player {
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
}

impl Default for Player {
    fn default() -> Self {
        Self::A
    }
}

#[derive(Clone, Copy, Debug)]
enum GameResult {
    Indefinite,
    Win(Player),
    Draw,
}

#[derive(Clone, Debug, Default)]
struct Board {
    turn: Player,
    board: [[Option<Player>; 6]; 7],
}

impl Board {
    fn new(turn: Player) -> Self {
        Self {
            turn,
            board: [[None; 6]; 7],
        }
    }

    fn put_piece(&mut self, column: usize, player: Player) {
        let column_top = self.board[column].iter().filter(|&x| x.is_some()).count();
        if column >= 7 || column_top >= 6 {
            panic!("invalid column");
        }
        self.board[column][column_top] = Some(player);
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

    fn calculate_result(&self) -> GameResult {
        fn eq_4<T: PartialEq>(vec: Vec<T>) -> bool {
            a == b && b == c && c == d
        }
        let none_count = self
            .iter()
            .flatten()
            .filter(|option| option.is_none())
            .count();
        if none_count == 0 {
            return GameResult::Draw;
        }

        for sequence in self.win_sequences {
            for group_of_4 in sequence.windows(4) {}
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
