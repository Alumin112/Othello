pub struct Othello {
    pub squares: [u64; 2], // 0 for black, 1 for white
    pub turn: Color,
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
        if let GameState::Pass = self.place_if_possible(position)? {
            self.turn.flip();
            return Ok(GameInfo::Pass(self.turn));
        }
        self.turn.flip();
        if let Some(winner) = self.winner() {
            return Ok(GameInfo::IsOver(winner));
        }
        Ok(GameInfo::Ok(self.turn))
    }

    fn place_if_possible(&mut self, position: Square) -> Result<GameState, &'static str> {
        if (self.squares[0] | self.squares[1]) & 1 << position.0 == 1 {
            return Err("The square is occupied");
        }
        let flips = self.get_possible_moves();
        if flips.is_empty() {
            return Ok(GameState::Pass);
        }
        let flips = match flips.iter().find(|square| square.0 .0 == position.0) {
            Some(flips) => flips,
            None => return Err("This move is Illegal"),
        };
        flip_bit(&mut self.squares[self.turn.to_bin()], flips.0 .0);
        for flip in &flips.1 {
            flip_bit(&mut self.squares[0], flip.0);
            flip_bit(&mut self.squares[1], flip.0);
        }
        Ok(GameState::Ok)
    }

    pub fn get_possible_moves(&self) -> Vec<(Square, Vec<Square>)> {
        let mut moves = Vec::new();
        for i in 0..64 {
            if (self.squares[0] | self.squares[1]) & 1 << i != 0 {
                continue;
            }
            let flips = self.get_flips(Square(i));
            if !flips.is_empty() {
                moves.push((Square(i), flips));
            }
        }
        moves
    }

    pub fn get_moves(&self) -> Vec<Square> {
        self.get_possible_moves().iter().map(|m| m.0).collect()
    }

    fn get_flips(&self, position: Square) -> Vec<Square> {
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
            match (position.0 % 8, x, y) {
                (7, 9 | 1, 1) => continue,
                (7, 7, -1) => continue,
                (0, 7, 1) => continue,
                (0, 9 | 1, -1) => continue,
                _ => (),
            }

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
                        match (square % 8, x, y) {
                            (7, 9 | 1, 1) => break None,
                            (7, 7, -1) => break None,
                            (0, 7, 1) => break None,
                            (0, 9 | 1, -1) => break None,
                            _ => (),
                        }
                    }
                    (_, _) => (),
                }
            };
            if let Some(mut flips) = flips {
                if !flips.is_empty() {
                    flipped_pieces.append(&mut flips);
                }
            }
        }
        flipped_pieces
    }

    fn winner(&mut self) -> Option<Color> {
        let mut white = 0;
        let mut black = 0;
        let mut empty = false;
        for i in 0..64 {
            if self.squares[0] & 1 << i != 0 {
                black += 1;
            } else if self.squares[1] & 1 << i != 0 {
                white += 1;
            } else {
                empty = true;
            }
        }
        if empty {
            if !self.get_moves().is_empty() {
                return None;
            }
            self.turn.flip();
            if !self.get_moves().is_empty() {
                self.turn.flip();
                return None;
            }
            self.turn.flip()
        }
        Some(if white > black {
            Color::White
        } else {
            Color::Black
        })
    }

    pub fn copy(&self) -> Self {
        Self {
            squares: self.squares,
            turn: self.turn,
        }
    }
}

impl Default for Othello {
    fn default() -> Self {
        Self::new()
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
        board.pop();
        write!(f, "{}", board)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Square(pub u64);

impl Square {
    pub fn from(coords: &str) -> Result<Self, &str> {
        if coords.len() != 2 {
            return Err("Invalid Coordinates");
        }
        let mut chars = coords.chars();
        let row = chars.next().unwrap();
        let ans;
        if ('A'..='H').contains(&row) {
            ans = row as u64 - 65;
        } else if ('a'..='h').contains(&row) {
            ans = row as u64 - 97;
        } else {
            return Err("Invalid Coordinates");
        }
        let col = chars.next().unwrap();
        if !('1'..='8').contains(&col) {
            return Err("Invalid Coordinates");
        }
        let col = col as u64 - 49;
        Ok(Self(col * 8 + ans))
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            (self.0 % 8 + 65) as u8 as char,
            (self.0 / 8 + 49) as u8 as char
        )
    }
}

#[derive(Debug)]
pub enum GameInfo {
    IsOver(Color),
    Ok(Color),
    Pass(Color),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn flip(&mut self) {
        *self = self.flipped()
    }

    pub fn flipped(self) -> Self {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
        }
    }

    pub fn to_bin(self) -> usize {
        match self {
            Self::Black => 0,
            Self::White => 1,
        }
    }
}

fn flip_bit(bitboard: &mut u64, bit: u64) {
    *bitboard ^= 1 << (bit)
}

enum GameState {
    Ok,
    Pass,
}
