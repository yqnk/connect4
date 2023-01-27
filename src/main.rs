#![allow(dead_code, unused_imports, unused_mut)]

mod client;

use client::game::Game;

fn main() {
    let mut g = Game::new();
    g.begin();
}
