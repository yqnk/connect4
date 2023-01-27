#![allow(dead_code)]

use std::io::{self, Write};

use super::holder::Holder;
use super::disk::Disk;

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
    }

    fn update(&mut self) {
        loop {
            self.clear();
            println!("{}", self.holder);
            print!("{} > ", self.turn);
            io::stdout().flush().expect("Failed flushing stdout...");
    
            let c: String = self.get_column();
            if c.contains("exit") {
                self.turn = 43;
                break;
            }
    
            if let Ok(col) = c.trim().parse::<usize>() {
                if (1..=7).contains(&col) {
                    let color = if self.turn % 2 == 0 { Disk::Red } else { Disk::Yellow };
                    self.holder.push(col, color);
                    self.turn += 1;
                    break;
                }
            }
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
        self.holder.check_lines() || self.holder.check_columns()
    }
}