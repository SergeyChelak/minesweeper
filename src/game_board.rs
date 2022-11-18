use rand::Rng;

struct GameCell {
    is_visible: bool,
    is_safe: bool,
}

pub struct GameBoard {
    cells: Vec<Vec<GameCell>>,
}

impl GameBoard {
    pub fn new(dimension: usize) -> Self {
        Self {
            cells: Self::default_cells(dimension),
        }
    }

    fn default_cells(dim: usize) -> Vec<Vec<GameCell>> {
        (0..dim).map({|_|
            (0..dim)
                .map(|_| GameCell {
                    is_visible: false,
                    is_safe: true,
                })
                .collect()
        }).collect()
    }

    pub fn populate_black_holes(&mut self, amount: usize) {
        let mut rng = rand::thread_rng();
        let dim = self.cells.len();
        let mut counter = amount;
        while counter > 0 {
            let row = rng.gen_range(0..dim);
            let col = rng.gen_range(0..dim);
            let mut cell = &mut self.cells[row][col];
            if cell.is_safe {
                cell.is_safe = false;
                counter -=1;
            }
        }
    }

    pub fn get_black_holes_count(&self, row: usize, col: usize) -> Option<usize> {
        if self.cells[row][col].is_safe {
            let count = self.get_surrounding_positions(row, col)
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
            (-1, 1)
        ].iter().map(|(di, dj)| {
            let i = row as isize + di;
            let j = col as isize + dj;
            (i, j)
        }).filter(|(i, j)| {
            range.contains(i) && range.contains(j)
        }).map(|(i, j)| (i as usize, j as usize))
        .collect()
    }

    pub fn is_open(&self, row: usize, col: usize) -> bool {
        self.cells[row][col].is_visible
    }

    pub fn try_open(&mut self, row: usize, col: usize) -> bool {
        if let Some(number) = self.get_black_holes_count(row, col) {
            if number > 0 {
                self.cells[row][col].is_visible = true;
                return true;
            }
        } else {
            // game over
            return false;
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
        true
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
