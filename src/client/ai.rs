use super::game::Game;

pub fn possible_moves(game: Game) -> [bool; 6] {
    let mut pmoves = [false; 6];
    for i in 0..6 {
        pmoves[i] = !game.holder.is_column_full(i);
    }
    pmoves
}

// todo evaluate