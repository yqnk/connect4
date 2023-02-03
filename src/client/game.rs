#![allow(dead_code)]

use std::io::{self, Write};

use crate::client::ai::{AI, possible_moves};

use super::holder::Holder;
use super::disk::Disk;
use super::ai::Idiot;

pub struct Game {
    pub holder: Holder,
    pub turn: usize
}

impl Game {
    pub fn new() -> Self {
        Self { holder: Holder::new(), turn: 1 }
    }
    
    pub fn begin(&mut self) {
        while !self.is_finished() && self.turn < 43 {
            self.update();
        }
        self.clear();
        println!("{}", self.holder);
    }

    fn update(&mut self) {
        loop {
            self.clear();
            let alpha = i32::min_value();
            let beta = i32::max_value();
            let maximizing_player = false;
            let (best_value, best_column) = Idiot::alphabeta(1, &mut self.holder, alpha, beta, maximizing_player);
            println!("{}", self.holder);
            print!("[{} up to {}] best move : {} > ", Idiot::evaluate(&mut self.holder), best_value, best_column);
            io::stdout().flush().expect("Failed printing stdout...");
    
            let c: String = self.get_column();
            if c.contains("exit") {
                self.turn = 43;
                break;
            }
    
            if let Ok(col) = c.trim().parse::<usize>() {
                if (1..=7).contains(&col) && !self.holder.is_column_full(col) {
                    self.holder.push(col, self.current_disk());
                    self.turn += 1;
                    break;
                }
            }
        }
    }
    
    pub fn current_disk(&self) -> Disk {
        match self.turn % 2 {
            0 => Disk::Red,
            _ => Disk::Yellow
        }
    }

    fn clear(&self) {
        io::stdout().write_all("\x1b[2J\x1b[1;1H".as_bytes()).unwrap();
    }

    fn get_column(&mut self) -> String {
        let mut col = String::new();
        io::stdin().read_line(&mut col).expect("Failed to read column...");
        col
    }

    fn is_finished(&self) -> bool {
        self.holder.check_lines() || self.holder.check_columns() || self.holder.check_left_diagonal() || self.holder.check_right_diagonal()
    }
}