
#[derive(Default)]
pub struct Settings {
    pub timebank: u64,
    pub time_per_move: u64,
    pub your_bot: String,
    pub field_width: u8,
    pub field_height: u8,
}

#[derive(Default)]
pub struct Game {
    pub round: u16,
    pub this_piece_type: PieceType,
    pub next_piece_type: PieceType,
    pub this_piece_position: Location,

    pub my_player: Player,
    pub other_player: Player,

    pub time_left: u64,
}

#[derive(Default)]
pub struct Player {
    pub row_points: u8,
    pub combo: u8,
    pub field: Field,
}

pub enum PieceType {
    I, J, L, O, S, T, Z,
}

impl Default for PieceType {
    fn default() -> Self {
        PieceType::I
    }
}

#[derive(Default)]
pub struct Location {
    pub x: i8,
    pub y: i8,
}

#[derive(Clone)]
pub enum CellType {
    Empty = 0,
    Shape = 1,
    Block = 2,
    Solid = 3,
}

#[derive(Default)]
pub struct Field {
    pub data: Box<[CellType]>,
}

pub enum Move {
    Down,
    Left,
    Right,
    TurnLeft,
    TurnRight,
    Drop,
}
