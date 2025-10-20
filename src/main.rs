mod command_parser;
mod commands;
mod grid;

use grid::MinesweeperGrid;

use crate::{commands::COMMAND_TABEL, grid::GameState};
const NUM_BOMBS: usize = 10;
const COLUMNS: usize = 8;
const ROWS: usize = 8;

fn main() {
    commands::print_help();
    game();
}

fn game() {
    let mut board = MinesweeperGrid::new();
    loop {
        let input = get_console_input();

        let (maybe_func, maybe_args) = command_parser::parse_input(&COMMAND_TABEL, input);

        if let (Some(func), Some(args)) = (maybe_func, maybe_args) {
            if func(&mut board, &args) {
                break;
            }
            println!("{}", board);
        }
    }
    println!("{}", board);

    if board.game_state == GameState::Lost {
        println!("You lost!");
    } else {
        println!("You won!");
    }

    println!("Play again? (y/n)");
    let input = get_console_input();

    if input == "y".to_string() {
        game();
    }
}

fn get_console_input() -> String {
    use std::io::{Write, stdin, stdout};

    let mut input: String = String::new();

    print!(">");

    let _ = stdout().flush();
    stdin()
        .read_line(&mut input)
        .expect("Did not enter a correct string");

    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }
    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }
    input
}
