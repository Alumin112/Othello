mod othello;
use othello::Othello;
pub use othello::{Color, GameInfo, Square, Validity};
mod ai;
use ai::AI;

pub struct Game(Othello);

impl Game {
    pub fn new() -> Self {
        Self(Othello::new())
    }

    pub fn new_custom(turn: Color, bitboard: [u64; 2]) -> Self {
        let mut new = Self(Othello::new_empty());
        new.0.turn = turn;
        new.0.squares = bitboard;
        new
    }

    pub fn play_computer(&mut self, depth: u8, iter: bool) -> GameInfo {
        let ai_move = AI::play_openings(self.0.copy(), iter, depth);

        println!(
            "Move: {} | Eval: {:<5} | Nodes: {:<8} | Depth: {}",
            ai_move.0, ai_move.1, ai_move.2, depth
        );

        self.0.play(ai_move.0).unwrap()
    }

    /// # Panics
    /// If the user input is invalid, the function panics
    pub fn play_stdin(&mut self) -> GameInfo {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.pop();
        let sq = Square::from(input.trim()).unwrap();
        self.0.play(sq).unwrap()
    }

    pub fn get_moves(&self) -> Vec<Square> {
        self.0.get_moves()
    }

    pub fn play(&mut self, square: Square) -> Validity<GameInfo> {
        self.0.play(square)
    }

    /// # Panics
    /// If the user input is invalid, the function panics
    pub fn start_p_vs_c(&mut self, depth: u8, player: Player) {
        println!("{}", self);
        let winner = loop {
            // User input
            println!("{:?}", self.get_moves());
            if let GameInfo::IsOver(winner) = self.play_stdin() {
                break winner;
            }
            println!("{}", self);

            // Computer
            println!("{:?}", self.get_moves());
            if let GameInfo::IsOver(winner) = self.play_computer(depth, false) {
                break winner;
            }
            println!("{}", self);
        };
        println!("{}", self);
        println!(
            "Winner: {:?}",
            if player.color == winner {
                player.name
            } else {
                "Computer"
            }
        );
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Player<'a> {
    name: &'a str,
    color: Color,
}

impl<'a> Player<'a> {
    pub fn new(name: &'a str, color: Color) -> Self {
        Self { name, color }
    }
}
