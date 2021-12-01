// mod othello;
// use crate::othello::*;
use ::othello::*;

fn main() {
    let mut board = Othello::new();
    let mut moves = String::new();
    // println!("{:?}", board.get_moves());
    // board.turn.flip();
    // let mut input = String::new();
    // AI::search_iteratively(board.copy(), 6);
    // println!("{:#?}", AI::get_best_move(board.copy(), 5));
    loop {
    let info = play(&mut board, 6, &mut moves, true);
    println!("{:?}", info);
    println!("{}\n", board);
    if let GameInfo::IsOver(_) = info {
        break;
    }
    // println!("{}\n", board);
    // std::io::stdin().read_line(&mut input).unwrap();
    // input.pop();
    // board.play(Square::from(&input).unwrap()).unwrap();
    // input.clear();
    }
}

fn play(board: &mut Othello, depth: u8, moves: &mut String, iter: bool) -> GameInfo {
    // println!("{:?}", board.get_moves());
    let ai_move = AI::play_openings(board.copy(), moves, iter, depth);
    println!("{} : {} : {}", ai_move.0.unwrap(), ai_move.1, ai_move.2);
    moves.push_str(&format!("{}", ai_move.0.unwrap()));
    println!("{}", moves);
    board.play(ai_move.0.unwrap()).unwrap()
}
