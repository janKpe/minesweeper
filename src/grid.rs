mod cell;
mod row;
use rand::Rng;
use row::*;
use std::fmt;

use crate::{COLUMNS, NUM_BOMBS, ROWS, grid::cell::CellState};
use std::ops::Range;

pub struct MinesweeperGrid {
    rows: [MinesweeperRow; ROWS],
    fields_uncoverd: usize,
    pub game_state: GameState,
}

struct CellRange {
    column_range: Range<usize>,
    row_range: Range<usize>,
}

#[derive(PartialEq)]
pub enum GameState {
    Lost,
    Won,
    InGame,
}

impl MinesweeperGrid {
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

        let cell_range = self.get_range_of_sourounding_cells_for_cell_at(row, column);
        let num_bombs = self.count_bombs_in_range(&cell_range);

        if num_bombs == 0 {
            self.reveal_cells_in_range(&cell_range, true);
        } else {
            // if the whole range of the current cell is revealed, the current cell is included
            self.reveal_cell_at(row, column, num_bombs);
        }


        self.check_if_game_is_won();

        if self.game_state != GameState::InGame {
            return true;
        }

        return false;
    }

    fn get_range_of_sourounding_cells_for_cell_at(&self, row: usize, column: usize) -> CellRange {
        let column_range = if column < 1 { 0 } else { column - 1 }..if column >= COLUMNS - 1 {
            COLUMNS
        } else {
            column + 2
        };
        let row_range =
            if row < 1 { 0 } else { row - 1 }..if row >= ROWS - 1 { ROWS } else { row + 2 };

        return CellRange {
            column_range,
            row_range,
        };
    }

    fn reveal_cells_in_range(&mut self, cell_range: &CellRange, reveal_neighbours: bool) {
        for row in cell_range.row_range.clone() {
            for column in cell_range.column_range.clone() {
                if self.rows[row].cells[column].state == CellState::Revealed {
                    continue;
                }
                if self.rows[row].cells[column].is_bomb {
                    continue;
                }

                let current_cell_range =
                    self.get_range_of_sourounding_cells_for_cell_at(row, column);
                let bombs_in_area = self.count_bombs_in_range(&current_cell_range);

                // mark as revealed immediately to prevent re-touching
                self.reveal_cell_at(row, column, bombs_in_area);

                if reveal_neighbours && bombs_in_area == 0 {
                    self.reveal_cells_in_range(&current_cell_range, true);
                }
            }
        }
    }

    fn count_bombs_in_range(&self, cell_range: &CellRange) -> i32 {
        let mut num_bombs = 0;

        for r in self.rows[cell_range.row_range.clone()].iter() {
            for c in r.cells[cell_range.column_range.clone()].iter() {
                if c.is_bomb {
                    num_bombs += 1;
                }
            }
        }
        num_bombs
    }

    fn check_if_game_is_won(&mut self) {
        if self.fields_uncoverd == ROWS * COLUMNS - NUM_BOMBS {
            self.game_state = GameState::Won;
        }
    }

    fn reveal_cell_at(&mut self, row: usize, column: usize, bombs_in_area: i32) {
        self.rows[row].cells[column].reveal(bombs_in_area);
        self.fields_uncoverd += 1;
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
