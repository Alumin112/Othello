/*
I did not write this
Some random guy on github did
*/


pub fn dumb7fill(mut fill: u64, empty: u64, dir: u64) -> u64 {
    let mut flood: u64 = 0;

    while fill > 0 {
        flood |= fill;
        fill = fill.shift(dir) & empty;
    }

    flood
}

pub fn bishop_attacks(from: Square, occupied: u64) -> u64 {
    let fill = 1 << from;
    let mut targets = 0;

    let occluded = dumb7fill(fill, !occupied & 0x7F7F7F7F7F7F7F7F, UP + LEFT);
    targets |= 0x7F7F7F7F7F7F7F7F & occluded.shift(UP + LEFT);
    let occluded = dumb7fill(fill, !occupied & 0x7F7F7F7F7F7F7F7F, DOWN + LEFT);
    targets |= 0x7F7F7F7F7F7F7F7F & occluded.shift(DOWN + LEFT);
    let occluded = dumb7fill(fill, !occupied & 0xFEFEFEFEFEFEFEFE, DOWN + RIGHT);
    targets |= 0xFEFEFEFEFEFEFEFE & occluded.shift(DOWN + RIGHT);
    let occluded = dumb7fill(fill, !occupied & 0xFEFEFEFEFEFEFEFE, UP + RIGHT);
    targets |= 0xFEFEFEFEFEFEFEFE & occluded.shift(UP + RIGHT);

    targets
}

pub fn rook_attacks(from: Square, occupied: u64) -> u64 {
    let fill = Bitboard::from_square(from);
    let mut targets = 0;

    let occluded = dumb7fill(fill, !occupied & 0xFFFFFFFFFFFFFFFF, UP);
    targets |= 0xFFFFFFFFFFFFFFFF & occluded.shift(UP);
    let occluded = dumb7fill(fill, !occupied & 0xFFFFFFFFFFFFFFFF, DOWN);
    targets |= 0xFFFFFFFFFFFFFFFF & occluded.shift(DOWN);
    let occluded = dumb7fill(fill, !occupied & 0x7F7F7F7F7F7F7F7F, LEFT);
    targets |= 0x7F7F7F7F7F7F7F7F & occluded.shift(LEFT);
    let occluded = dumb7fill(fill, !occupied & 0xFEFEFEFEFEFEFEFE, RIGHT);
    targets |= 0xFEFEFEFEFEFEFEFE & occluded.shift(RIGHT);

    targets
}
