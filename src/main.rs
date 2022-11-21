use std::io;

mod game_board;

use game_board::*;

fn main() {
    let black_holes = 10usize;
    let dimension = 10usize;
    println!("Board size: {dimension}x{dimension}, black holes: {black_holes}");
    let mut board = GameBoard::new(dimension);
    board.populate_black_holes(black_holes);
    println!("{}", board.formatted(true));
    println!("Current board displayed as:");
    loop {
        println!("{}", board.formatted(false));
        println!("Your move:");
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read string");
        let pos_str: Vec<&str> = buffer.trim().split(" ").collect();
        let mut is_correct = false;
        if pos_str.len() == 2 {
            let row: isize = pos_str[0].parse().unwrap_or(-1);
            let col: isize = pos_str[1].parse().unwrap_or(-1);
            if row >= 0 && col >= 0 {
                is_correct = true;
                board.open(row as usize, col as usize);
                if board.is_game_over() {
                    if board.is_won() {
                        println!("The mine is here. Game over");
                    } else {
                        println!("You won. Game over");
                    }
                    break;
                }
            }
        }
        if !is_correct {
            println!("Incorrect input, try again");
        }
    }
}
