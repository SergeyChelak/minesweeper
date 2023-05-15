pub struct Configuration;

impl Configuration {
    pub fn window_title(&self) -> String {
        "Minesweeper".to_string()
    }

    pub fn row_count(&self) -> usize {
        10
    }

    pub fn col_count(&self) -> usize {
        16
    }

    pub fn mines_count(&self) -> usize {
        20
    }
}
