use std::fmt;

#[derive(Copy, Clone, PartialEq)]
pub enum CellState {
    Marked,
    Unmarked, // and also not revealed
    Revealed,
}

#[derive(Copy, Clone)]
pub struct MinesweeperCell {
    pub(crate) is_bomb: bool,
    pub(crate) state: CellState,
    bombs_in_area: Option<i32>,
}

impl MinesweeperCell {
    pub fn new() -> Self {
        MinesweeperCell {
            is_bomb: false,
            state: CellState::Unmarked,
            bombs_in_area: None,
        }
    }

    pub(crate) fn set_state(&mut self, new_state: CellState) {
        // if a cell is revealed it cant be undone
        if self.state == CellState::Revealed {
            return;
        }
        self.state = new_state;
    }

    pub(crate) fn reveal(&mut self, bombs_in_area: i32) {
        self.bombs_in_area = Some(bombs_in_area);
        self.set_state(CellState::Revealed);
    }
}

impl fmt::Display for MinesweeperCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_name: String;

        if self.state == CellState::Revealed {
            if self.bombs_in_area == Some(0) {
                display_name = "   ".to_string();
            } else {
                display_name = " ".to_string() + &self.bombs_in_area.unwrap().to_string() + " ";
            }
        } else if self.state == CellState::Unmarked {
            display_name = " ■ ".to_string();
        } else {
            // cell is marked
            display_name = " 🚩".to_string();
        }
        write!(f, "{}", display_name)
    }
}
