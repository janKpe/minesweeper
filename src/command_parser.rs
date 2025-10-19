use crate::{commands::Command, grid::MinesweeperGrid};

pub fn parse_input(
    command_tabel: &[Command],
    input: String,
) -> (
    Option<fn(&mut MinesweeperGrid, &[String]) -> bool>,
    Option<Vec<String>>,
) {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return (None, None);
    }

    let mut command_parts: Vec<String> = trimmed.split_whitespace().map(String::from).collect();

    // The first part of the input should be the command itself
    let command_uni = match command_tabel.iter().find(|c| c.command == command_parts[0]) {
        Some(c) => c,
        None => {
            println!("Command '{}' not found", command_parts[0]);
            return (None, None);
        }
    };

    command_parts.remove(0);

    (Some(command_uni.func), Some(command_parts))
}
