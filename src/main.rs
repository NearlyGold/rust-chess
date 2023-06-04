use std::fmt::{Display, Formatter, write};
use std::io;
use rand::Rng;


#[derive(Copy, Clone, PartialEq)]
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
    fn index(&self) -> usize {
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

    fn value(&self) -> i8 {
        match *self {
            Rank::One => 0,
            Rank::Two => 1,
            Rank::Three => 2,
            Rank::Four => 3,
            Rank::Five => 4,
            Rank::Six => 5,
            Rank::Seven => 6,
            Rank::Eight => 7,
        }
    }
}


#[derive(Copy, Clone, PartialEq)]
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
    fn index(&self) -> usize {
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

    fn value(&self) -> i8 {
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


#[derive(Copy, Clone, PartialEq)]
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
    Piece(GamePiece),
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Square::Empty => write!(f, " "),
            Square::Piece(piece) => write!(f, "{}", piece),
        }
    }
}

struct GamePiece {
    colour: Colour,
    kind: PieceKind,
}

impl Display for GamePiece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.colour, self.kind.name())
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

impl PieceKind {
    fn name(&self) -> String {
        match *self {
            PieceKind::Pawn => String::from("Pawn"),
            PieceKind::Knight => String::from("Knight"),
            PieceKind::Bishop => String::from("Bishop"),
            PieceKind::Rook => String::from("Rook"),
            PieceKind::Queen => String::from("Queen"),
            PieceKind::King => String::from("King"),
        }
    }

    fn symbol(&self) -> char {
        match *self {
            PieceKind::Pawn => 'P',
            PieceKind::Knight => 'N',
            PieceKind::Bishop => 'B',
            PieceKind::Rook => 'R',
            PieceKind::Queen => 'Q',
            PieceKind::King => 'K',
        }
    }
}


type Board = [[Square; 8]; 8];
type Position = (File, Rank);


fn generate_piece_set(colour: Colour) -> Vec<GamePiece> {
    const PIECE_SET: [PieceKind; 16] = [
        PieceKind::Pawn, PieceKind::Pawn, PieceKind::Pawn, PieceKind::Pawn,
        PieceKind::Pawn, PieceKind::Pawn, PieceKind::Pawn, PieceKind::Pawn,
        PieceKind::Rook, PieceKind::Rook, PieceKind::Knight, PieceKind::Knight,
        PieceKind::Bishop, PieceKind::Bishop, PieceKind::Queen, PieceKind::King,
    ];

    let mut new_piece_set: Vec<GamePiece> = Vec::new();

    for i in 0..15 {
        new_piece_set.push(
            GamePiece {
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
            Square::Piece(GamePiece {colour: Colour::Black, kind: PieceKind::Rook}),
            Square::Piece(GamePiece {colour: Colour::Black, kind: PieceKind::Knight}),
            Square::Piece(GamePiece {colour: Colour::Black, kind: PieceKind::Bishop}),
            Square::Piece(GamePiece {colour: Colour::Black, kind: PieceKind::Queen}),
            Square::Piece(GamePiece {colour: Colour::Black, kind: PieceKind::King}),
            Square::Piece(GamePiece {colour: Colour::Black, kind: PieceKind::Bishop}),
            Square::Piece(GamePiece {colour: Colour::Black, kind: PieceKind::Knight}),
            Square::Piece(GamePiece {colour: Colour::Black, kind: PieceKind::Rook}),
        ],
        [
            Square::Piece(GamePiece {colour: Colour::Black, kind: PieceKind::Pawn}),
            Square::Piece(GamePiece {colour: Colour::Black, kind: PieceKind::Pawn}),
            Square::Piece(GamePiece {colour: Colour::Black, kind: PieceKind::Pawn}),
            Square::Piece(GamePiece {colour: Colour::Black, kind: PieceKind::Pawn}),
            Square::Piece(GamePiece {colour: Colour::Black, kind: PieceKind::Pawn}),
            Square::Piece(GamePiece {colour: Colour::Black, kind: PieceKind::Pawn}),
            Square::Piece(GamePiece {colour: Colour::Black, kind: PieceKind::Pawn}),
            Square::Piece(GamePiece {colour: Colour::Black, kind: PieceKind::Pawn}),
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
            Square::Piece(GamePiece {colour: Colour::White, kind: PieceKind::Pawn}),
            Square::Piece(GamePiece {colour: Colour::White, kind: PieceKind::Pawn}),
            Square::Piece(GamePiece {colour: Colour::White, kind: PieceKind::Pawn}),
            Square::Piece(GamePiece {colour: Colour::White, kind: PieceKind::Pawn}),
            Square::Piece(GamePiece {colour: Colour::White, kind: PieceKind::Pawn}),
            Square::Piece(GamePiece {colour: Colour::White, kind: PieceKind::Pawn}),
            Square::Piece(GamePiece {colour: Colour::White, kind: PieceKind::Pawn}),
            Square::Piece(GamePiece {colour: Colour::White, kind: PieceKind::Pawn}),
        ],
        [
            Square::Piece(GamePiece {colour: Colour::White, kind: PieceKind::Rook}),
            Square::Piece(GamePiece {colour: Colour::White, kind: PieceKind::Knight}),
            Square::Piece(GamePiece {colour: Colour::White, kind: PieceKind::Bishop}),
            Square::Piece(GamePiece {colour: Colour::White, kind: PieceKind::Queen}),
            Square::Piece(GamePiece {colour: Colour::White, kind: PieceKind::King}),
            Square::Piece(GamePiece {colour: Colour::White, kind: PieceKind::Bishop}),
            Square::Piece(GamePiece {colour: Colour::White, kind: PieceKind::Knight}),
            Square::Piece(GamePiece {colour: Colour::White, kind: PieceKind::Rook}),
        ],
    ]
}

fn print_board_square(board: &Board, file: File, rank: Rank) {
    println!("{}", board[rank.index()][file.index()]);
}

// Rules
// * The dest square must not be the source square
// * A piece must exist on the source square
// * The piece must be the current player's colour
// * The dest square must be empty, or have a piece of the opponent's colour that is not the king.
// * TODO: The piece must be allowed to move in the way required to reach the destination position.
// * TODO: After moving, the player's king must not be in check.
fn is_valid_move(player_colour: Colour, board: &Board, source_position: Position, dest_position: Position) -> bool {
    // The source and destination positions must not be the same
    if source_position == dest_position {
        return false;
    }

    let source_square: &Square = &board[source_position.1.index()][source_position.0.index()];

    // A piece must exist on the source square
    let source_piece: &GamePiece = match source_square {
        Square::Empty => return false,
        Square::Piece(p) => p,
    };

    // The source piece must be the player's colour
    if source_piece.colour != player_colour {
        return false;
    }

    let dest_square: &Square = &board[dest_position.1.index()][dest_position.0.index()];

    // The dest square must either be empty or have a non-king piece of the opponent's colour.
    if let Square::Piece(dest_piece) = dest_square {
        if dest_piece.colour == player_colour {
            return false;
        }
        if let PieceKind::King = dest_piece.kind {
            return false;
        }
    }

    true
}

fn calculate_move_vector(source_position: Position, dest_position: Position) -> (i8, i8) {
    let x1 = source_position.0.value();
    let y1 = source_position.1.value();

    let x2 = dest_position.0.value();
    let y2 = dest_position.1.value();

    return (x2-x1, y2-y1);
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_to_same_square() {
        let result = is_valid_move(
            Colour::White,
            &generate_board(),
            (File::A, Rank::One),
            (File::A, Rank::One)
        );
        assert_eq!(result, false);
    }

    #[test]
    fn move_to_empty_square() {
        let result = is_valid_move(
            Colour::White,
            &generate_board(),
            (File::E, Rank::Two),
            (File::E, Rank::Three)
        );
        assert_eq!(result, true);
    }

    #[test]
    fn move_opponent_piece() {
        let result = is_valid_move(
            Colour::Black,
            &generate_board(),
            (File::B, Rank::One),
            (File::C, Rank::Three)
        );
        assert_eq!(result, false);
    }

    #[test]
    fn move_non_existent_piece() {
        let result = is_valid_move(
            Colour::White,
            &generate_board(),
            (File::D, Rank::Four),
            (File::D, Rank::Five)
        );
        assert_eq!(result, false);
    }

    #[test]
    fn move_onto_player_piece() {
        let result = is_valid_move(
            Colour::Black,
            &generate_board(),
            (File::D, Rank::Eight),
            (File::D, Rank::Seven)
        );
        assert_eq!(result, false);
    }

    #[test]
    fn move_onto_opponent_king() {
        let result = is_valid_move(
            Colour::White,
            &generate_board(),
            (File::D, Rank::One),
            (File::E, Rank::Eight)
        );
        assert_eq!(result, false);
    }

    #[test]
    fn calc_move_vector_same_square() {
        let result = calculate_move_vector((File::A, Rank::One), (File::A, Rank::One));
        assert_eq!(result, (0, 0));
    }

    #[test]
    fn calc_move_vector_one_forward() {
        let result = calculate_move_vector((File::A, Rank::One), (File::A, Rank::Two));
        assert_eq!(result, (0, 1));
    }

    #[test]
    fn calc_move_vector_one_backwards() {
        let result = calculate_move_vector((File::D, Rank::Four), (File::D, Rank::Three));
        assert_eq!(result, (0, -1));
    }

    #[test]
    fn calc_move_vector_one_left() {
        let result = calculate_move_vector((File::E, Rank::One), (File::D, Rank::One));
        assert_eq!(result, (-1, 0));
    }

    #[test]
    fn calc_move_vector_one_right() {
        let result = calculate_move_vector((File::D, Rank::One), (File::E, Rank::One));
        assert_eq!(result, (1, 0));
    }

    #[test]
    fn calc_move_vector_one_diag_forward_right() {
        let result = calculate_move_vector((File::C, Rank::One), (File::D, Rank::Two));
        assert_eq!(result, (1, 1));
    }
}
