use std::process::exit;

use crate::grid::MinesweeperGrid;

macro_rules! return_if_error {
    ( $( $val:expr ),* $(,)? ) => {
        {
            let mut __counter = 1;
            $(
                if $val.is_err() {
                    println!("Expected arg {} to be int", __counter);
                    return false;
                }
                __counter += 1;
            )*
        }
    };
}

pub struct Command {
    pub func: fn(&mut MinesweeperGrid, &[String]) -> bool,
    pub command: &'static str,
    pub description: &'static str,
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
            
            return_if_error!(x, y);

            return board.mark(x.unwrap() - 1, y.unwrap() - 1);
        },
        command: "m",
        description: "m <x> <y>  - Mark a cell as a mine",
    },
    Command {
        func: |board, args| {
            if args.len() != 2 {
                println!("Expected two arguemnts");
                return false;
            }

            let x = args[0].parse::<usize>();
            let y = args[1].parse::<usize>();
            
            return_if_error!(x, y);

            return board.unmark(x.unwrap() - 1, y.unwrap() - 1);
        },
        command: "u",
        description: "u <x> <y>  - Unmark a cell (remove flag)",
    },
    Command {
        func: |board, args| {
            if args.len() != 2 {
                println!("Expected two arguemnts");
                return false;
            }

            let x = args[0].parse::<usize>();
            let y = args[1].parse::<usize>();

            return_if_error!(x, y);

            return board.reveal(x.unwrap() - 1, y.unwrap() - 1);
        },
        command: "r",
        description: "r <x> <y>  - Reveal a cell"
    },
    Command {
        func: |_, _| {
            exit(0);
        },
        command: "q",
        description: "q          - Quit the game",
    },
    Command {
        func: |_, _| {
            print_help();
            false
        },
        command: "h",
        description: "h          - Show this help message",
    },
];

pub fn print_help() {
    println!("===============================");
    println!("      Minesweeper Commands      ");
    println!("===============================");

    for command in COMMAND_TABEL  {
        println!("{}", command.description);
    }
}
