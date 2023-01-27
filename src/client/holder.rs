#![allow(dead_code)]

use super::disk::Disk;

const COL_SIZE: usize = 6;
const ROW_SIZE: usize = 7;

#[derive(Debug)]
pub struct Holder {
    pub holders: [[Disk; COL_SIZE]; ROW_SIZE],
    pub h_ptr: [usize; 7]
}

impl Holder {
    pub fn new() -> Self {
        Self {
            holders: [[Disk::None; 6]; 7],
            h_ptr: [0; 7],
        }
    }

    pub fn push(&mut self, column: usize, disk: Disk) {
        if self.h_ptr[column - 1] < 6 {
            self.holders[column - 1][self.h_ptr[column - 1]] = disk;
            self.h_ptr[column - 1] += 1;
        }
        
    }

    pub fn check_columns(&self) -> bool {
        (0..7).any(|j| (self.holders[j][2] == self.holders[j][3]) && (self.holders[j][4] == self.holders[j][5]) || (self.holders[j][4] == self.holders[j][1]) || (self.holders[j][1] == self.holders[j][0]))
    }
}

impl std::fmt::Display for Holder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut handler_display = String::new();

        for i in 0..6 {
            handler_display.push('|');
            for j in 0..7 {
                handler_display.push_str(self.holders[j][COL_SIZE - 1 - i].as_str());
                handler_display.push('|');
            }
            handler_display.push('\n');
        }
        let columns = ["1", "2", "3", "4", "5", "6", "7"];
        write!(f, "  {}\n{}", columns.join("   "), handler_display)
    }
}