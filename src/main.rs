mod othello;
use crate::othello::*;

fn main() {
    let mut board = Othello::new();
    println!("{:?}", board.get_possible_moves());
    // println!("{:?}", board);
    // println!("{}", board);
    match board.play(Square(37)).unwrap() {
        GameInfo::IsOver(winner) => println!("WINNER: {:?}", winner),
        GameInfo::Ok(turn) => println!("TURN: {:?}", turn)
    }
    println!("{}", board);
    println!("{:?}", board.get_possible_moves());
    board.play(Square(26)).unwrap();
    println!("{:?}", board.get_possible_moves());
    println!("{}", board);
    // board.play(Square(3, 7)).unwrap();
    // println!("{}", board)
}
