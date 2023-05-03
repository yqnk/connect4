#![allow(dead_code, unused_imports, unused_mut)]

use std::io;

mod client;

use client::game::Game;

fn main() {
    let mut g = Game::new();
    println!("Choose a number between :\n - Simple AI (0)\n - Good AI (1)\n - No AI (else)");
    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let choice: u8 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => 2,
    };
    g.begin(choice);
}
