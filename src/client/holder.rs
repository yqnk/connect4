#![allow(dead_code)]

use super::disk::Disk;

const NUM_ROWS: usize = 6; // 6 rows = amt of elements in a column
const NUM_COLS: usize = 7; // 7 columns = amt of elements in a row

#[derive(Debug, Clone, Copy)]
pub struct Holder {
    pub holders: [[Disk; NUM_ROWS]; NUM_COLS],
    pub h_ptr: [usize; 7],
}

impl Holder {
    pub fn new() -> Self {
        Self {
            holders: [[Disk::None; 6]; 7],
            h_ptr: [0; 7],
        }
    }

    pub fn push(&mut self, column: usize, disk: Disk) {
        self.holders[column - 1][self.h_ptr[column - 1]] = disk;
        self.h_ptr[column - 1] += 1;
    }

    pub fn is_column_full(&self, column: usize) -> bool {
        self.h_ptr[column - 1] > 5
    }

    pub fn check_columns(&self) -> bool {
        self.holders
            .iter()
            .flat_map(|c| c.windows(4))
            .any(|w| (0..4).all(|i| w[i] == w[0] && w[0] != Disk::None))
    }

    pub fn check_lines(&self) -> bool {
        (0..NUM_ROWS).any(|row| {
            self.holders
                .iter()
                .map(|col| col[row])
                .collect::<Vec<_>>()
                .windows(4)
                .any(|w| (0..4).all(|i| w[i] == w[0] && w[i] != Disk::None))
        })
    }

    pub fn check_left_diagonal(&self) -> bool {
        (0..4).any(|d| {
            (0..3).any(|r| {
                (0..4)
                    .map(|i| self.holders[d + i][r + i])
                    .all(|c| c == self.holders[d][r])
                    && self.holders[d][r] != Disk::None
            })
        })
    }

    pub fn check_right_diagonal(&self) -> bool {
        (0..4).any(|d| {
            (3..6).rev().any(|r| {
                (0..4)
                    .map(|i| self.holders[d + i][r - i])
                    .all(|c| c == self.holders[d][r])
                    && self.holders[d][r] != Disk::None
            })
        })
    }

    pub fn is_yellow(&self, c: usize, r: usize) -> bool {
        self.holders[c][r] == Disk::Yellow
    }

    pub fn is_red(&self, c: usize, r: usize) -> bool {
        self.holders[c][r] == Disk::Red
    }
}

impl std::fmt::Display for Holder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut handler_display = String::new();

        for i in 0..6 {
            handler_display.push('|');
            for j in 0..7 {
                handler_display.push_str(self.holders[j][NUM_ROWS - 1 - i].as_str());
                handler_display.push('|');
            }
            handler_display.push('\n');
        }
        let columns = ["1", "2", "3", "4", "5", "6", "7"];
        write!(f, "  {}\n{}", columns.join("   "), handler_display)
    }
}
