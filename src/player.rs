extern crate rand;

use self::rand::Rng;

use types;
use types::Move::*;
use types::PieceType::*;

use ::std::io::Write;

macro_rules! println_stderr(
    ($($arg:tt)*) => (
        match writeln!(&mut ::std::io::stderr(), $($arg)* ) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
    )
);



pub fn make_moves(settings: &types::Settings, game: &types::Game) -> Vec<types::Move> {
    let mut rng = rand::thread_rng();

    let mut moves = Vec::new();

    // First get the complete bounding box inside the field
    for _ in (0..-game.this_piece_position.y) {
        moves.push(Down);
    }

    // Randomly move our block counter-clockwise
    while rng.gen() {
        moves.push(TurnLeft);
    }

    // Randomly move our block clockwise
    while rng.gen() {
        moves.push(TurnRight);
    }

    // Take the size of a block
    let size = match game.this_piece_type {
        I => 4,
        J | L | S | T | Z => 3,
        O => 2,
    };

    // Pick a target column for the left side of our block
    let mut column = rng.gen_range(0, settings.field_width - size + 1) as i8;

    println_stderr!("I want to drop the left side of my block in column {}", column);

    while column > game.this_piece_position.x {
        moves.push(Right);
        column -= 1;
    }

    while column < game.this_piece_position.x {
        moves.push(Left);
        column += 1;
    }

    // Drop it
    moves.push(Drop);

    return moves;
}
