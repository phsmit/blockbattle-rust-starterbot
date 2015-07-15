use types::{Settings,Game,PieceType,Field,Location,CellType};
use std::{fmt,io};
use std::error::Error;
use std::str::{SplitWhitespace,FromStr};

pub enum Action {
    Move,
    Quit,
}

#[derive(Debug)]
pub struct ParserError {
    pub message: String,
}

impl Error for ParserError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, _: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        unimplemented!()
    }
}

fn unknown_option(option: &str, line: &str, line_no: usize) -> ParserError {
    ParserError{message: format!("Unknown option {} on line {} ({})", option, line_no, line.trim())}
}

pub struct Parser<T> where T: io::BufRead {
    r : T,
    line_number: usize,
}

impl From<io::Error> for ParserError {
    fn from(e: io::Error) -> ParserError {
        ParserError{message: format!("Generic IO error: {}", e.description())}
    }
}


impl From<::std::num::ParseIntError> for ParserError {
    fn from(_: ::std::num::ParseIntError) -> ParserError {
        ParserError{message: "Invalid data type".to_string()}
    }
}

impl FromStr for PieceType {
    type Err = ParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "I" => Ok(PieceType::I),
            "J" => Ok(PieceType::J),
            "L" => Ok(PieceType::L),
            "O" => Ok(PieceType::O),
            "S" => Ok(PieceType::S),
            "T" => Ok(PieceType::T),
            "Z" => Ok(PieceType::Z),
            e @ _ => Err(ParserError{message:format!("Unknown piece type: {}",e)}),
        }
    }
}

impl FromStr for CellType {
    type Err = ParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(CellType::Empty),
            "1" => Ok(CellType::Shape),
            "2" => Ok(CellType::Block),
            "3" => Ok(CellType::Solid),
            e @ _ => Err(ParserError{message:format!("Unknown cell type: {}",e)}),
        }
    }
}

impl FromStr for Location {
    type Err = ParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.split(',').collect();
        if v.len() != 2 {
            return Err(ParserError{message:format!("Can't parse {} into Location", s)})
        }

        Ok(Location{x:try!(parse(v[0])), y:try!(parse(v[1]))})
    }
}

impl FromStr for Field {
    type Err = ParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.split(|c: char| c == ',' || c==';').collect();
        let mut ints: Vec<CellType> = vec![CellType::Empty; v.len()];
        for (i, s) in v.iter().enumerate() {
            ints[i] = try!(parse::<CellType>(s));
        }

        Ok(Field{data: ints.into_boxed_slice()})
    }

}

fn parse<T>(s: &str) -> Result<T,ParserError> where T: FromStr, T::Err: Error {
    match s.parse::<T>() {
        Ok(x) => Ok(x),
        Err(x) => Err(ParserError{message: format!("Could not parse {}, {}", s, x.description())}),
    }
}

fn parse_line<'a>(line: &'a str, line_number: usize, settings: &mut Settings, game: &mut Game) -> Result<Option<Action>, ParserError> {
    let mut t = &mut line.split_whitespace();

    let next = |t: &mut SplitWhitespace<'a>| t.next().ok_or(ParserError{message: format!("Missing token on line {} ({})", line_number, line.trim())});
    let rest = |t: &mut SplitWhitespace<'a>| t.collect::<Vec<&str>>().connect(" ");

    match try!(next(t)) {
        "settings" => match try!(next(t)) {
                "timebank" => settings.timebank = try!(parse(&rest(t))),
                "time_per_move" => settings.time_per_move = try!(parse(&rest(t))),
                "player_names" => {},
                "your_bot" => settings.your_bot = rest(t),
                "field_width" => settings.field_width = try!(parse(&rest(t))),
                "field_height" => settings.field_height = try!(parse(&rest(t))),
                e @_ => return Err(unknown_option(e, line, line_number)),
            },
        "update" => match try!(next(t)) {
            "game" => match try!(next(t)) {
                "round" => game.round = try!(parse(&rest(t))),
                "this_piece_type" => game.this_piece_type = try!(parse(&rest(t))),
                "next_piece_type" => game.next_piece_type = try!(parse(&rest(t))),
                "this_piece_position" => game.this_piece_position = try!(parse(&rest(t))),
                e @_ => return Err(unknown_option(e, &line, line_number)),
                },
            p @ _ => {
                let player = match p {
                    x if x == settings.your_bot => &mut game.my_player,
                    _ => &mut game.other_player,
                };
                match try!(next(t)) {
                    "row_points" => player.row_points = try!(parse(&rest(t))),
                    "combo" => player.combo = try!(parse(&rest(t))),
                    "field" => player.field = try!(parse(&rest(t))),
                    e @_ => return Err(unknown_option(e, &line, line_number)),
                    }
                },
            },
        "action" => match try!(next(t)) {
            "moves" => {
                game.time_left = try!(parse(&rest(t)));
                return Ok(Some(Action::Move));
            }
            e @_ => return Err(unknown_option(e, &line, line_number)),
            },
        "quit" => return Ok(Some(Action::Quit)),
        e @_ => return Err(unknown_option(e, &line, line_number)),
    }

    return Ok(None);
}

impl<T> Parser<T> where T: io::BufRead {
    pub fn new(r: T) -> Parser<T> {
        Parser{r : r, line_number: 0}
    }

    pub fn parse_until_action(&mut self, settings: &mut Settings, game: &mut Game) -> Result<Action, ParserError> {
        let mut line = String::new();

        while {line.clear(); try!(self.r.read_line(&mut line)) > 0} {
            self.line_number += 1;

            let l = line.trim();

            //skip empty lines
            if l.is_empty() {continue}
            
            match try!(parse_line(l, self.line_number, settings, game)) {
                Some(x) => return Ok(x),
                None => continue,
            }
        }
        println!("Running out of lines");
        return Ok(Action::Quit);
    }
}
