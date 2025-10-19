use std::process::exit;

use crate::grid::MinesweeperGrid;

pub struct Command {
    pub func: fn(&mut MinesweeperGrid, &[String]) -> bool,
    pub command: &'static str,
}

pub const COMMAND_TABEL: [Command; 5] = [
    Command {
        func: |board: &mut MinesweeperGrid, args: &[String]| {
            if args.len() != 2 {
                println!("Expected two arguemnts");
                return false;
            }

            let x = args[0].parse::<usize>();
            let y = args[1].parse::<usize>();
            if x.is_err() {
                println!("Expected arg 1 to be int");
            }
            if y.is_err() {
                println!("Expected arg 2 to be int");
            }
            return board.mark(x.unwrap() - 1, y.unwrap() - 1);
        },
        command: "m",
    },
    Command {
        func: |board, args| {
            if args.len() != 2 {
                println!("Expected two arguemnts");
                return true;
            }

            let x = args[0].parse::<usize>();
            let y = args[1].parse::<usize>();
            if x.is_err() {
                println!("Expected arg 1 to be int");
            }
            if y.is_err() {
                println!("Expected arg 2 to be int");
            }
            return board.unmark(x.unwrap() - 1, y.unwrap() - 1);
        },
        command: "u",
    },
    Command {
        func: |board, args| {
            if args.len() != 2 {
                println!("Expected two arguemnts");
                return true;
            }

            let x = args[0].parse::<usize>();
            let y = args[1].parse::<usize>();
            if x.is_err() {
                println!("Expected arg 1 to be int");
            }
            if y.is_err() {
                println!("Expected arg 2 to be int");
            }
            return board.reveal(x.unwrap() - 1, y.unwrap() - 1);
        },
        command: "r",
    },
    Command {
        func: |_, _| {
            exit(0);
        },
        command: "q",
    },
    Command {
        func: |_, _| {
            print_help();
            false
        },
        command: "h",
    },
];

pub fn print_help() {
    println!("===============================");
    println!("      Minesweeper Commands      ");
    println!("===============================");
    println!("m <x> <y>  - Mark a cell as a mine");
    println!("u <x> <y>  - Unmark a cell (remove flag)");
    println!("r <x> <y>  - Reveal a cell");
    println!("q          - Quit the game");
    println!("h          - Show this help message");
}
