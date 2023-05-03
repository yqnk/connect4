#![allow(dead_code, unused_imports, unused_mut)]

use super::{holder::Holder, game::Game, disk::Disk};

macro_rules! max {
    ($x:expr, $y:expr) => {
        if $x > $y { $x } else { $y }
    };
}

macro_rules! min {
    ($x:expr, $y:expr) => {
        if $x < $y { $x } else { $y }
    };
}

pub struct Idiot;
pub struct NNAi;

pub trait AI {
    fn make_move(holder: &mut Holder, column: usize, disk: Disk) -> Holder;
    fn evaluate(holder: &Holder) -> i32;
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

impl Idiot {
    pub fn alphabeta(depth: i32, holder: &mut Holder, mut alpha: i32, mut beta: i32, maximizing_player: bool) -> (i32, usize) {
        if depth == 0 {
            return (Idiot::evaluate(holder), 0);
        }
    
        if maximizing_player {
            let mut best_value = i32::min_value();
            let mut best_column = 0;
            for column in 1..8 {
                if !holder.is_column_full(column) {
                    let mut new_board = Idiot::make_move(&mut holder.clone(), column, Disk::Yellow);
                    let (new_value, _) = Idiot::alphabeta(depth - 1, &mut new_board, alpha, beta, false);
                    if new_value > best_value {
                        best_value = new_value;
                        best_column = column;
                    }
                    alpha = max!(alpha, best_value);
                    if beta <= alpha {
                        break;
                    }
                }
            }
            return (best_value, best_column);
        } else {
            let mut best_value = i32::max_value();
            let mut best_column = 0;
            for column in 1..8 {
                if !holder.is_column_full(column) {
                    let mut new_board = Idiot::make_move(&mut holder.clone(), column, Disk::Red);
                    let (new_value, _) = Idiot::alphabeta(depth - 1, &mut new_board, alpha, beta, true);
                    if new_value < best_value {
                        best_value = new_value;
                        best_column = column;
                    }
                    beta = min!(beta, best_value);
                    if beta <= alpha {
                        break;
                    }
                }
            }
            return (best_value, best_column);
        }
    }
}

impl AI for Idiot {
    /* Scores :
     * Yellow -> positive
     * Red -> negative
     */
    fn evaluate(holder: &Holder) -> i32 {
        let mut eval: i32 = 0;
        for c in 0..7 {
            for r in 0..6 {
                if holder.is_yellow(c, r) {
                    eval += P_TABLE[c][r] as i32;
                } else if holder.is_red(c, r) {
                    eval -= P_TABLE[c][r] as i32;
                }
            }
        }
        eval
    }

    fn make_move(holder: &mut Holder, column: usize, disk: Disk) -> Holder {
        holder.push(column, disk);
        *holder
    }
}
// todo evaluate