pub struct Othello {
    squares: [u64; 2], // 0 for black, 1 for white
    turn: Color,
}

impl Othello {
    pub fn new() -> Self {
        let mut squares = [0, 0];
        flip_bit(&mut squares[0], 28);
        flip_bit(&mut squares[0], 35);
        flip_bit(&mut squares[1], 27);
        flip_bit(&mut squares[1], 36);
        Self {
            squares,
            turn: Color::Black,
        }
    }

    pub fn play(&mut self, position: Square) -> Result<GameInfo, &str> {
        self.place_if_possible(position)?;
        if let Some(winner) = self.winner() {
            return Ok(GameInfo::IsOver(winner));
        }
        self.turn.flip();
        Ok(GameInfo::Ok(self.turn))
    }

    fn place_if_possible(&mut self, position: Square) -> Result<(), &'static str> {
        if (self.squares[0] | self.squares[1]) & position.0 == 1 {
            return Err("The square is occupied");
        }
        let flips = self.get_flips(position);
        if flips.is_empty() {
            return Err("This is Illegal");
        }
        flip_bit(&mut self.squares[self.turn.to_bin()], position.0);
        for direction in flips {
            for flip in direction {
                flip_bit(&mut self.squares[0], flip.0);
                flip_bit(&mut self.squares[1], flip.0);
            }
        }
        Ok(())
    }

    pub fn get_possible_moves(&self) -> Vec<(Square, Vec<Vec<Square>>)> {
        let mut moves = Vec::new();
        for i in 0..64 {
            let flips = self.get_flips(Square(i));
            if !flips.is_empty() {
                moves.push((Square(i), flips));
            }
        }
        moves
    }

    fn get_flips(&self, position: Square) -> Vec<Vec<Square>> {
        let mut flipped_pieces = Vec::new();
        for (x, y) in [
            (9, 1),
            (9, -1),
            (8, 1),
            (8, -1),
            (7, 1),
            (7, -1),
            (1, 1),
            (1, -1),
        ] {
            let mut square = if y < 0 {
                match position.0.checked_sub(x) {
                    None => continue,
                    Some(res) => res,
                }
            } else {
                match position.0.checked_add(x) {
                    None => continue,
                    Some(res) => res,
                }
            };
            let mut flips = Vec::new();
            let flips = loop {
                if !(0..64).contains(&square) {
                    break None;
                }
                match (
                    self.squares[!self.turn.to_bin() & 1] & 1 << square,
                    self.squares[self.turn.to_bin()] & 1 << square,
                ) {
                    (0, 0) => break None,
                    (0, _) => break Some(flips),
                    (_, 0) => {
                        flips.push(Square(square));
                        square = if y < 0 {
                            match square.checked_sub(x) {
                                None => break None,
                                Some(res) => res,
                            }
                        } else {
                            match square.checked_add(x) {
                                None => break None,
                                Some(res) => res,
                            }
                        };
                    }
                    (_, _) => break None,
                }
            };
            if let Some(flips) = flips {
                if !flips.is_empty() {
                    flipped_pieces.push(flips)
                }
            }
        }
        flipped_pieces
    }

    fn winner(&self) -> Option<Color> {
        let mut white = 0;
        let mut black = 0;
        for i in 0..64 {
            if self.squares[0] & 1 << i != 0 {
                black += 1;
            } else if self.squares[1] & 1 << i != 0 {
                white += 1;
            } else {
                return None;
            }
        }
        Some(if white > black {
            Color::White
        } else {
            Color::Black
        })
    }
}

impl std::fmt::Debug for Othello {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board = String::new();
        let total = self.squares[0] | self.squares[1];
        for i in 0..64 {
            match total & (1 << i) {
                0 => board.push_str(&format!("0:{:<3} ", i)),
                _ => board.push_str(&format!("1:{:<3} ", i)),
            }
            if (i + 1) % 8 == 0 {
                board.push('\n')
            }
        }
        write!(f, "{}", board)
    }
}

impl std::fmt::Display for Othello {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board = String::new();
        let total = self.squares[0] | self.squares[1];
        for i in 0..64 {
            match total & (1 << i) {
                0 => board.push_str("x "),
                _ => board.push_str(if self.squares[0] & (1 << i) != 0 {
                    "B "
                } else {
                    "W "
                }),
            }
            if (i + 1) % 8 == 0 {
                board.push('\n')
            }
        }
        write!(f, "{}", board)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Square(pub u64);

pub enum GameInfo {
    IsOver(Color),
    Ok(Color),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    fn flip(&mut self) {
        *self = match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
        }
    }

    fn to_bin(self) -> usize {
        match self {
            Self::Black => 0,
            Self::White => 1,
        }
    }
}

fn flip_bit(bitboard: &mut u64, bit: u64) {
    *bitboard ^= 1 << (bit)
}
