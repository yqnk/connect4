use super::{holder::Holder, game::Game, disk::Disk};

pub struct Idiot;

pub trait AI {
    fn new() -> Self;
    fn evaluate(holder: &Holder) -> i32;
    fn think(depth: u8, holder: &Holder) -> usize;
}

const P_TABLE: [[u8; 6]; 7] = [[3,  4,  5,  5,  4, 3],
                               [4,  6,  8,  8,  6, 4],
                               [5,  8, 11, 11,  8, 5],
                               [7, 10, 13, 13, 10, 7],
                               [5,  8, 11, 11,  8, 5],
                               [4,  6,  8,  8,  6, 4],
                               [3,  4,  5,  5,  4, 3]];

pub fn possible_moves(holder: &Holder) -> Vec<usize> {
    let mut pmoves = Vec::<usize>::new();
    for i in 1..7 {
        if !holder.is_column_full(i) {
            pmoves.push(i)
        }
    }
    pmoves 
}

impl AI for Idiot {

    fn new() -> Self {
        Self
    }
    /* Scores :
     * Yellow -> positive
     * Red -> negative
     */
    fn evaluate(holder: &Holder) -> i32 {
        let mut eval: i8 = 0;
        for c in 0..7 {
            for r in 0..6 {
                if holder.is_yellow(c, r) {
                    eval += P_TABLE[c][r] as i8;
                } else if holder.is_red(c, r) {
                    eval -= P_TABLE[c][r] as i8;
                }
            }
        }
        eval as i32
    }

    fn think(_depth: u8, _holder: &Holder) -> usize {
        0
    }
}
// todo evaluate