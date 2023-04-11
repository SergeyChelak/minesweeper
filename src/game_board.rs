use rand::Rng;

struct Cell {
    is_visible: bool,
    is_safe: bool,
    is_flagged: bool,
}

#[derive(Debug, PartialEq, Eq)]
enum State {
    Lose,
    Won,
    Continues,
}

pub struct GameBoard {
    cells: Vec<Vec<Cell>>,
    state: State,
}

impl GameBoard {
    pub fn new(dimension: usize) -> Self {
        Self {
            cells: Self::default_cells(dimension),
            state: State::Continues,
        }
    }

    fn default_cells(dim: usize) -> Vec<Vec<Cell>> {
        (0..dim)
            .map({
                |_| {
                    (0..dim)
                        .map(|_| Cell {
                            is_visible: false,
                            is_safe: true,
                            is_flagged: false,
                        })
                        .collect()
                }
            })
            .collect()
    }

    pub fn is_game_over(&self) -> bool {
        self.state != State::Continues
    }

    pub fn is_won(&self) -> bool {
        self.state == State::Won
    }

    pub fn populate_black_holes(&mut self, amount: usize) {
        if self.is_game_over() {
            return;
        }
        let mut rng = rand::thread_rng();
        let dim = self.cells.len();
        let mut counter = amount;
        while counter > 0 {
            let row = rng.gen_range(0..dim);
            let col = rng.gen_range(0..dim);
            let mut cell = &mut self.cells[row][col];
            if cell.is_safe {
                cell.is_safe = false;
                counter -= 1;
            }
        }
    }

    pub fn get_black_holes_count(&self, row: usize, col: usize) -> Option<usize> {
        if self.cells[row][col].is_safe {
            let count = self
                .get_surrounding_positions(row, col)
                .iter()
                .fold(0usize, |acc, (i, j)| {
                    acc + if self.cells[*i][*j].is_safe { 0 } else { 1 }
                });
            Some(count)
        } else {
            None
        }
    }

    fn get_surrounding_positions(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let range = 0..self.cells.len() as isize;
        vec![
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
            (1, 1),
            (-1, -1),
            (1, -1),
            (-1, 1),
        ]
        .iter()
        .map(|(di, dj)| {
            let i = row as isize + di;
            let j = col as isize + dj;
            (i, j)
        })
        .filter(|(i, j)| range.contains(i) && range.contains(j))
        .map(|(i, j)| (i as usize, j as usize))
        .collect()
    }

    pub fn is_open(&self, row: usize, col: usize) -> bool {
        self.cells[row][col].is_visible
    }

    pub fn open(&mut self, row: usize, col: usize) {
        if self.is_game_over() {
            return;
        }
        if let Some(number) = self.get_black_holes_count(row, col) {
            if number > 0 {
                self.cells[row][col].is_visible = true;
                return;
            }
            let mut positions = vec![(row, col)];
            while positions.len() > 0 {
                let (row, col) = positions.pop().expect("shouldn't happen");
                let mut cell = &mut self.cells[row][col];
                if cell.is_visible {
                    continue;
                }
                cell.is_visible = true;
                for (i, j) in self.get_surrounding_positions(row, col) {
                    if let Some(number) = self.get_black_holes_count(i, j) {
                        if number == 0 {
                            positions.push((i, j));
                        } else {
                            self.cells[i][j].is_visible = true;
                        }
                    }
                }
            }
        } else {
            self.state = State::Lose;
            return;
        }
    }

    pub fn put_flag(&mut self, row: usize, col: usize) {
        if self.is_game_over() {
            return;
        }
        self.cells[row][col].is_flagged = !self.cells[row][col].is_flagged;
        for i in 0..self.cells.len() {
            for j in 0..self.cells[i].len() {
                let cell = &self.cells[i][j];
                if cell.is_safe {
                    if cell.is_flagged {
                        return;
                    }
                } else if !cell.is_flagged {
                    return;
                }
            }
        }
        self.state = State::Won;
    }

    pub fn formatted(&self, ignore_hidden: bool) -> String {
        let mut strs: Vec<String> = Vec::with_capacity(self.cells.len());
        for i in 0..self.cells.len() {
            let row = &self.cells[i];
            let mut comp: Vec<String> = Vec::with_capacity(row.len());
            for j in 0..row.len() {
                if !self.is_open(i, j) && !ignore_hidden {
                    comp.push("*".to_string());
                } else if let Some(number) = self.get_black_holes_count(i, j) {
                    comp.push(format!("{}", number));
                } else {
                    comp.push("@".to_string());
                }
            }
            let line: Vec<String> = comp.iter().map(|s| format!("{:3}", s)).collect();
            strs.push(line.join(" "))
        }
        strs.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use crate::game_board::State;

    use super::GameBoard;

    #[test]
    fn board_populate_black_holes() {
        let dim = 10usize;
        let mut board = GameBoard::new(dim);
        let holes = 15;
        board.populate_black_holes(holes);
        let mut count = 0usize;
        for i in 0..dim {
            for j in 0..dim {
                count += if board.cells[i][j].is_safe { 0 } else { 1 };
            }
        }
        assert_eq!(count, holes, "Wrong holes count");
    }

    #[test]
    fn board_lose() {
        let dim = 10usize;
        let mut board = GameBoard::new(dim);
        board.populate_black_holes(10);
        assert!(!board.is_game_over(), "Wrong initial state");
        'l: for i in 0..dim {
            for j in 0..dim {
                if !board.cells[i][j].is_safe {
                    board.open(i, j);
                    break 'l;
                }
            }
        }
        assert_eq!(
            board.state,
            State::Lose,
            "Game state wasn't changed properly"
        );
        assert!(board.is_game_over(), "Game over flag did not properly");
    }

    #[test]
    fn board_win() {
        let dim = 10usize;
        let mut board = GameBoard::new(dim);
        let mut flags = 10;
        board.populate_black_holes(flags);
        for i in 0..dim {
            for j in 0..dim {
                if !board.cells[i][j].is_safe {
                    board.put_flag(i, j);
                    flags -= 1;
                    if flags > 0 {
                        assert_eq!(
                            board.state,
                            State::Continues,
                            "Game state changed incorrectly"
                        );
                    }
                }
            }
        }
        assert_eq!(
            board.state,
            State::Won,
            "Game state wasn't changed properly"
        );
        assert!(board.is_game_over(), "Game over flag did not properly");
    }
}
