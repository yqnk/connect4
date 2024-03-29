#![allow(dead_code)]

use std::io::{self, Write};
use std::{thread, time};

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
    
    pub fn begin(&mut self, ai: u8) {
        while !self.is_finished() && self.turn < 43 {
            match ai {
                0 => self.update_idiot(),
                1 => self.update_good(),
                _ => self.alone()
            }
        }
        self.clear();
        println!("{}", self.holder);
    }

    pub fn flush_holder(&self) {
        self.clear();
        println!("{}", self.holder);
        print!("[Turn {}] > ", self.turn);
        io::stdout().flush().expect("Failed printing stdout...");
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

    fn update_idiot(&mut self) {
        loop {
            let alpha = i32::min_value();
            let beta = i32::max_value();
            let (_best_value, best_column) = Idiot::alphabeta(4, &mut self.holder, alpha, beta, true);

            self.flush_holder();
    
            let c: String = self.get_column();
            if c.contains("exit") {
                self.turn = 43;
                break;
            }
    
            if let Ok(col) = c.trim().parse::<usize>() {
                if (1..=7).contains(&col) && !self.holder.is_column_full(col) {
                    self.holder.push(col, self.current_disk());
                    self.turn += 1;
                    
                    self.flush_holder();
                    // AI's turn
                    let laps = time::Duration::from_secs(1);
                    thread::sleep(laps);
                    self.holder.push(best_column, self.current_disk());
                    self.turn += 1;
                    break;
                }
            }
        }
    }
    
    pub fn update_good(&mut self) {
        
    }

    pub fn alone(&mut self) {
        loop {
            self.flush_holder();
    
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
}