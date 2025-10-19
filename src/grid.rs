mod cell;
mod row;
use rand::Rng;
use row::*;
use std::fmt;

use crate::{COLUMNS, NUM_BOMBS, ROWS, grid::cell::CellState};

pub struct MinesweeperGrid {
    rows: [MinesweeperRow; ROWS],
    fields_uncoverd: usize,
    pub game_state: GameState,
}
#[derive(PartialEq)]
pub enum GameState {
    Lost,
    Won,
    InGame,
}

impl MinesweeperGrid {
    fn place_random_bombs(&mut self, num_bombs: usize) {
        let mut rng = rand::rng();

        for _ in 0..num_bombs {
            let mut row_index = rng.random_range(0..ROWS);
            let mut column_index = rng.random_range(0..COLUMNS);

            let mut cell = &mut self.rows[row_index].cells[column_index];

            while cell.is_bomb {
                row_index = rng.random_range(0..ROWS);
                column_index = rng.random_range(0..COLUMNS);
                cell = &mut self.rows[row_index].cells[column_index];
            }

            cell.is_bomb = true;
        }
    }

    pub fn new() -> Self {
        let rows = std::array::from_fn(|_| MinesweeperRow::new());

        let mut grid = MinesweeperGrid {
            rows,
            fields_uncoverd: 0,
            game_state: GameState::InGame,
        };

        grid.place_random_bombs(NUM_BOMBS);
        grid
    }

    pub fn mark(&mut self, column: usize, row: usize) -> bool {
        // return value indicades if game has ended
        self.set_cell_state(column, row, cell::CellState::Marked);
        false
    }

    pub fn unmark(&mut self, column: usize, row: usize) -> bool {
        // return value indicades if game has ended
        self.set_cell_state(column, row, cell::CellState::Unmarked);
        false
    }

    pub fn reveal(&mut self, column: usize, row: usize) -> bool {
        // return value indicades if game has ended
        // check bomb and state immutably first to avoid holding a mutable borrow while iterating neighbors
        if self.rows[row].cells[column].is_bomb {
            // the game is lost if a bomb is uncoverd
            self.game_state = GameState::Lost;
            return true;
        }
        if self.rows[row].cells[column].state == CellState::Revealed {
            return false;
        }

        let mut num_bombs = 0;

        let column_range = if column < 1 { 0 } else { column - 1 }..if column >= COLUMNS - 1 {
            COLUMNS
        } else {
            column + 2
        };
        let row_range =
            if row < 1 { 0 } else { row - 1 }..if row >= ROWS - 1 { ROWS } else { row + 2 };

        for r in self.rows[row_range].iter() {
            for c in r.cells[column_range.clone()].iter() {
                if c.is_bomb {
                    num_bombs += 1;
                }
            }
        }

        self.fields_uncoverd += 1;

        self.rows[row].cells[column].reveal_cell(num_bombs);
        self.check_if_game_is_won();

        if self.game_state != GameState::InGame {
            return true;
        }
        return false;
    }

    fn check_if_game_is_won(&mut self) {
        if self.fields_uncoverd == ROWS * COLUMNS - NUM_BOMBS {
            self.game_state = GameState::Won;
        }
    }

    fn set_cell_state(&mut self, column: usize, row: usize, state: cell::CellState) {
        self.rows[row].cells[column].set_state(state);
    }
}

impl fmt::Display for MinesweeperGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.rows.iter() {
            write!(f, "{}\n", row)?;
        }
        Ok(())
    }
}
