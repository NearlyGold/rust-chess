use std::fmt::{Display, Formatter, write};
use std::io;
use rand::Rng;


#[derive(Copy, Clone, Eq, PartialEq)]
enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl Rank {
    fn value(&self) -> usize {
        match *self {
            Rank::One => 7,
            Rank::Two => 6,
            Rank::Three => 5,
            Rank::Four => 4,
            Rank::Five => 3,
            Rank::Six => 2,
            Rank::Seven => 1,
            Rank::Eight => 0,
        }
    }
}


#[derive(Copy, Clone, Eq, PartialEq)]
enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl File {
    fn value(&self) -> usize {
        match *self {
            File::A => 0,
            File::B => 1,
            File::C => 2,
            File::D => 3,
            File::E => 4,
            File::F => 5,
            File::G => 6,
            File::H => 7,
        }
    }
}


#[derive(Copy, Clone)]
enum Colour {
    White,
    Black,
}

impl Display for Colour {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Colour::White => write!(f, "White"),
            Colour::Black => write!(f, "Black"),
        }
    }
}

enum Square {
    Empty,
    Piece(Piece),
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Square::Empty => write!(f, " "),
            Square::Piece(piece) => write!(f, "{}", piece),
        }
    }
}

struct Piece {
    colour: Colour,
    kind: PieceKind,
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.colour, get_piece_name(self))
    }
}

#[derive(Copy, Clone)]
enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}


type Board = [[Square; 8]; 8];
type Position = (File, Rank);


fn get_piece_name(piece: &Piece) -> String {
    match piece.kind {
        PieceKind::Pawn => String::from("Pawn"),
        PieceKind::Knight => String::from("Knight"),
        PieceKind::Bishop => String::from("Bishop"),
        PieceKind::Rook => String::from("Rook"),
        PieceKind::Queen => String::from("Queen"),
        PieceKind::King => String::from("King"),
    }
}

fn get_piece_symbol(piece: &Piece) -> char {
    match piece.kind {
        PieceKind::Pawn => 'P',
        PieceKind::Knight => 'N',
        PieceKind::Bishop => 'B',
        PieceKind::Rook => 'R',
        PieceKind::Queen => 'Q',
        PieceKind::King => 'K',
    }
}

fn generate_piece_set(colour: Colour) -> Vec<Piece> {
    const PIECE_SET: [PieceKind; 16] = [
        PieceKind::Pawn, PieceKind::Pawn, PieceKind::Pawn, PieceKind::Pawn,
        PieceKind::Pawn, PieceKind::Pawn, PieceKind::Pawn, PieceKind::Pawn,
        PieceKind::Rook, PieceKind::Rook, PieceKind::Knight, PieceKind::Knight,
        PieceKind::Bishop, PieceKind::Bishop, PieceKind::Queen, PieceKind::King,
    ];

    let mut new_piece_set: Vec<Piece> = Vec::new();

    for i in 0..15 {
        new_piece_set.push(
            Piece {
                colour,
                kind: PIECE_SET[i]
            }
        );
    }

    new_piece_set
}

fn generate_board() -> Board {
    [
        [
            Square::Piece(Piece{colour: Colour::Black, kind: PieceKind::Rook}),
            Square::Piece(Piece{colour: Colour::Black, kind: PieceKind::Knight}),
            Square::Piece(Piece{colour: Colour::Black, kind: PieceKind::Bishop}),
            Square::Piece(Piece{colour: Colour::Black, kind: PieceKind::King}),
            Square::Piece(Piece{colour: Colour::Black, kind: PieceKind::Queen}),
            Square::Piece(Piece{colour: Colour::Black, kind: PieceKind::Bishop}),
            Square::Piece(Piece{colour: Colour::Black, kind: PieceKind::Knight}),
            Square::Piece(Piece{colour: Colour::Black, kind: PieceKind::Rook}),
        ],
        [
            Square::Piece(Piece{colour: Colour::Black, kind: PieceKind::Pawn}),
            Square::Piece(Piece{colour: Colour::Black, kind: PieceKind::Pawn}),
            Square::Piece(Piece{colour: Colour::Black, kind: PieceKind::Pawn}),
            Square::Piece(Piece{colour: Colour::Black, kind: PieceKind::Pawn}),
            Square::Piece(Piece{colour: Colour::Black, kind: PieceKind::Pawn}),
            Square::Piece(Piece{colour: Colour::Black, kind: PieceKind::Pawn}),
            Square::Piece(Piece{colour: Colour::Black, kind: PieceKind::Pawn}),
            Square::Piece(Piece{colour: Colour::Black, kind: PieceKind::Pawn}),
        ],
        [
            Square::Empty, Square::Empty, Square::Empty, Square::Empty,
            Square::Empty, Square::Empty, Square::Empty, Square::Empty,
        ],
        [
            Square::Empty, Square::Empty, Square::Empty, Square::Empty,
            Square::Empty, Square::Empty, Square::Empty, Square::Empty,
        ],
        [
            Square::Empty, Square::Empty, Square::Empty, Square::Empty,
            Square::Empty, Square::Empty, Square::Empty, Square::Empty,
        ],
        [
            Square::Empty, Square::Empty, Square::Empty, Square::Empty,
            Square::Empty, Square::Empty, Square::Empty, Square::Empty,
        ],
        [
            Square::Piece(Piece{colour: Colour::White, kind: PieceKind::Pawn}),
            Square::Piece(Piece{colour: Colour::White, kind: PieceKind::Pawn}),
            Square::Piece(Piece{colour: Colour::White, kind: PieceKind::Pawn}),
            Square::Piece(Piece{colour: Colour::White, kind: PieceKind::Pawn}),
            Square::Piece(Piece{colour: Colour::White, kind: PieceKind::Pawn}),
            Square::Piece(Piece{colour: Colour::White, kind: PieceKind::Pawn}),
            Square::Piece(Piece{colour: Colour::White, kind: PieceKind::Pawn}),
            Square::Piece(Piece{colour: Colour::White, kind: PieceKind::Pawn}),
        ],
        [
            Square::Piece(Piece{colour: Colour::White, kind: PieceKind::Rook}),
            Square::Piece(Piece{colour: Colour::White, kind: PieceKind::Knight}),
            Square::Piece(Piece{colour: Colour::White, kind: PieceKind::Bishop}),
            Square::Piece(Piece{colour: Colour::White, kind: PieceKind::Queen}),
            Square::Piece(Piece{colour: Colour::White, kind: PieceKind::King}),
            Square::Piece(Piece{colour: Colour::White, kind: PieceKind::Bishop}),
            Square::Piece(Piece{colour: Colour::White, kind: PieceKind::Knight}),
            Square::Piece(Piece{colour: Colour::White, kind: PieceKind::Rook}),
        ],
    ]
}

fn print_board_square(board: &Board, file: File, rank: Rank) {
    println!("{}", board[rank.value()][file.value()]);
}

// Rules
// * The dest square must not be the source square
// * A piece must exist on the source square
// * The piece must be the current player's colour
// * The dest square must be empty, or have a piece of the opponent's colour that is not the king.
// * The piece must be allowed to move in the way required to reach the destination position.
// * After moving, the player's king must not be in check.
fn is_valid_move(player_colour: Colour, board: &Board, source_position: Position, dest_position: Position) -> bool {
    // The source and destination positions must not be the same
    if source_position == dest_position {
        return false;
    }

    let source_square: &Square = &board[source_position.1.value()][source_position.0.value()];

    // A piece must exist on the source square
    let piece: &Piece = match source_square {
        Square::Empty => return false,
        Square::Piece(p) => p,
    };

    true
}

fn main() {
    let my_colour = Colour::White;

    println!("My colour is: {}", my_colour);

    let my_pieces = generate_piece_set(Colour::White);
    let opp_pieces = generate_piece_set(Colour::Black);

    for piece in my_pieces {
        println!("{} ", piece);
    }

    let mut board = generate_board();

    print_board(&board);

    print_board_square(&board, File::D, Rank::One);
    print_board_square(&board, File::E, Rank::Two);

    println!("{}", is_valid_move(my_colour, &board, (File::A, Rank::One), (File::A, Rank::One))); // Fail -- same square
    println!("{}", is_valid_move(my_colour, &board, (File::A, Rank::One), (File::A, Rank::Two))); // Pass -- piece exists
    println!("{}", is_valid_move(my_colour, &board, (File::D, Rank::Four), (File::A, Rank::Two))); // Fail -- no piece
}

fn print_board(board: &Board) {
    for rank in board {
        for square in rank {
            print!("| ");
            // print!("{square:14} | ");
            print!("{square:<width$}", square=square.to_string(), width=14);
        }
        println!("|");
    }
}
