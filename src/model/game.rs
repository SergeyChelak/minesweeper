use rand::Rng;
use std::time::Instant;

#[derive(PartialEq, Eq, Debug)]
enum State {
    InProgress,
    Win,
    Lose,
}

#[derive(Clone, Copy)]
struct Cell {
    is_visible: bool,
    is_safe: bool,
    is_flagged: bool,
    mines_count: usize,
}

impl Cell {
    fn new() -> Self {
        Self {
            is_visible: false,
            is_safe: true,
            is_flagged: false,
            mines_count: 0,
        }
    }
}

enum Op {
    Inc,
    Dec,
    None,
}

impl Op {
    fn is_applicable(&self, value: usize, max_value: usize) -> bool {
        match self {
            Op::Inc => value + 1 < max_value,
            Op::Dec => value > 0,
            _ => true,
        }
    }

    fn apply(&self, value: usize) -> usize {
        match self {
            Op::Inc => value + 1,
            Op::Dec => value - 1,
            _ => value,
        }
    }
}

struct Game {
    board: Vec<Vec<Cell>>,
    state: State,
    mines: usize,
    row_count: usize,
    col_count: usize,
    start_time: Instant,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: vec![],
            state: State::InProgress,
            mines: 0,
            row_count: 0,
            col_count: 0,
            start_time: Instant::now(),
        }
    }

    fn start(&mut self, rows: usize, cols: usize, mines: usize) {
        self.row_count = rows;
        self.col_count = cols;
        self.mines = mines;
        self.start_time = Instant::now();
        self.state = State::InProgress;
        self.board = vec![vec![Cell::new(); self.col_count]; self.row_count];
        self.fill_mines();
        self.fill_safe_numbers();
    }

    fn fill_mines(&mut self) {
        let mut rng = rand::thread_rng();
        let mut counter = self.mines;
        while counter > 0 {
            let row = rng.gen_range(0..self.row_count);
            let col = rng.gen_range(0..self.col_count);
            let mut cell = &mut self.board[row][col];
            if cell.is_safe {
                cell.is_safe = false;
                counter -= 1;
            }
        }
    }

    fn fill_safe_numbers(&mut self) {
        for row in 0..self.row_count {
            for col in 0..self.col_count {
                self.board[row][col].mines_count = self.calc_mines_count(row, col);
            }
        }
    }

    fn calc_mines_count(&mut self, row: usize, col: usize) -> usize {
        self.adjacent_cells(row, col)
            .iter()
            .filter(|(r, c)| !self.board[*r][*c].is_safe)
            .count()
    }

    fn adjacent_cells(&mut self, row: usize, col: usize) -> Vec<(usize, usize)> {
        vec![
            (Op::Inc, Op::None),
            (Op::Dec, Op::None),
            (Op::None, Op::Inc),
            (Op::None, Op::Dec),
            (Op::Inc, Op::Inc),
            (Op::Dec, Op::Dec),
            (Op::Inc, Op::Dec),
            (Op::Dec, Op::Inc),
        ]
        .iter()
        .filter(|(r, c)| {
            r.is_applicable(row, self.row_count) && c.is_applicable(col, self.col_count)
        })
        .map(|(r, c)| (r.apply(row), c.apply(col)))
        .collect()
    }

    pub fn open_cell(&mut self, row: usize, col: usize) {
        if !self.can_touch_cell(row, col) {
            return;
        }
        let current = &mut self.board[row][col];
        if current.is_safe {
            if current.mines_count > 0 {
                current.is_visible = true;
            } else {
                let mut adjacent = vec![(row, col)];
                while let Some((row, col)) = adjacent.pop() {
                    {
                        let mut cell = &mut self.board[row][col];
                        if cell.is_visible {
                            continue;
                        }
                        cell.is_visible = true;
                    }
                    for (r, c) in self.adjacent_cells(row, col) {
                        let cell = &mut self.board[r][c];
                        if !cell.is_safe {
                            continue;
                        }
                        if cell.mines_count == 0 {
                            adjacent.push((r, c));
                        } else {
                            cell.is_visible = true;
                        }
                    }
                }
            }
            self.track_win_state();
        } else {
            self.state = State::Lose;
        }
    }

    pub fn flag_cell(&mut self, row: usize, col: usize) {
        if !self.can_touch_cell(row, col) {
            return;
        }
        self.board[row][col].is_flagged = !self.board[row][col].is_flagged;
        self.track_win_state();
    }

    fn can_touch_cell(&self, row: usize, col: usize) -> bool {
        if self.state != State::InProgress {
            return false;
        }
        if row >= self.row_count && col >= self.col_count {
            return false;
        }
        !self.board[row][col].is_visible
    }

    fn track_win_state(&mut self) {
        let mut is_flag_win = true;
        let mut is_open_win = true;
        for r in 0..self.row_count {
            for c in 0..self.col_count {
                let cell = &self.board[r][c];
                if cell.is_safe {
                    is_open_win &= cell.is_visible && !cell.is_flagged;
                } else {
                    is_flag_win &= cell.is_flagged;
                }
                if !is_flag_win && !is_open_win {
                    return;
                }
            }
        }
        if is_flag_win || is_open_win {
            self.state = State::Win;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_game() -> Game {
        let mut game = Game::new();
        let mines = 10;
        game.start(9, 9, mines);
        game
    }

    #[test]
    fn model_fill_mines() {
        let game = create_game();
        let found = game.board.iter()
            .map(|row| {
                row.iter().filter(|cell| !cell.is_safe).count()
            })
            .sum::<usize>();
        assert_eq!(found, game.mines);
    }

    #[test]
    fn model_lose() {
        let mut game = create_game();
        'outer: for r in 0..game.row_count {
            for c in 0..game.col_count {
                if !game.board[r][c].is_safe {
                    game.open_cell(r, c);
                    break 'outer
                }
            }
        }
        assert_eq!(game.state, State::Lose);
    }
}