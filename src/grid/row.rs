use crate::{COLUMNS, grid::cell::MinesweeperCell};
use std::fmt;

#[derive(Copy, Clone)]
pub(super) struct MinesweeperRow {
    pub(super) cells: [MinesweeperCell; COLUMNS],
}

impl MinesweeperRow {
    pub(super) fn new() -> Self {
        let cells = std::array::from_fn(|_| MinesweeperCell::new());

        MinesweeperRow { cells }
    }
}

impl fmt::Display for MinesweeperRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for cell in self.cells.iter() {
            write!(f, "{} ", cell)?;
        }
        Ok(())
    }
}
