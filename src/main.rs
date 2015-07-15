pub mod types;
mod parser;
mod player;

use std::io;
use types::Move;



fn print_moves(moves: Vec<Move>) {
    if moves.len() == 0 {
        println!("no_moves");
        return;
    }

    for m in moves {
        let s = match m {
            Move::Down => "down",
            Move::Left => "left",
            Move::Right => "right",
            Move::TurnLeft => "turnleft",
            Move::TurnRight => "turnright",
            Move::Drop => "drop",
        };
        print!("{},",s);
    }
    println!("");

}


fn main() {
    let s = io::stdin();
    let br = s.lock();
    let mut p = parser::Parser::new(br);

    let mut game = types::Game::default();
    let mut settings = types::Settings::default();

    loop {
        match p.parse_until_action(&mut settings, &mut game) {
            Ok(a) => match a {
                parser::Action::Move => print_moves(player::make_moves(&settings, &game)),
                parser::Action::Quit => break,
            },
            Err(e) => {
                println!("Parsing error: {}", e.message);
                break;
            },
        }
    }
}
