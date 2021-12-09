use ::othello::*;

fn main() {
    let mut board = Game::new();
    board.start_p_vs_c(5, Player::new("Bunch-of-cellulose", Color::White));
}
