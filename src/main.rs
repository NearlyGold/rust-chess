use std::fmt::{Display, Formatter, write};
use std::io;
use rand::Rng;

const RANK_1: usize = 7;
const RANK_2: usize = 6;
const RANK_3: usize = 5;
const RANK_4: usize = 4;
const RANK_5: usize = 3;
const RANK_6: usize = 2;
const RANK_7: usize = 1;
const RANK_8: usize = 0;

const FILE_A: usize = 0;
const FILE_B: usize = 1;
const FILE_C: usize = 2;
const FILE_D: usize = 3;
const FILE_E: usize = 4;
const FILE_F: usize = 5;
const FILE_G: usize = 6;
const FILE_H: usize = 7;


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

fn generate_pieces(colour: Colour) -> Vec<Piece> {
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

fn main() {
    let my_colour = Colour::White;

    println!("My colour is: {}", my_colour);

    let my_pieces = generate_pieces(Colour::White);

    for piece in my_pieces {
        println!("{} ", piece);
    }

    return;

    let mut board: [[char; 8]; 8] = [
        ['R', 'N', 'B', 'K', 'Q', 'B', 'N', 'R'],
        ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
        ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'],
    ];

    print_board(board);

    println!();
    println!();

    // Move the E2 pawn forward 2 squares
    board[RANK_2][FILE_E] = ' ';
    board[RANK_4][FILE_E] = 'P';
    // board[6][4] = ' ';
    // board[5][4] = 'P';

    print_board(board);

    //
    // println!("Welcome to chess!");
    //
    // println!("Please tell me your favourite piece.");
    //
    // let mut piece = String::new();
    //
    // io::stdin()
    //     .read_line(&mut piece)
    //     .expect("Failed to read line");
    //
    // println!("Your favourite piece is {piece}!");
    //
    // println!("Let's start a game. Would you like to play as white or black?");
    // println!("Or leave blank for a random choice.");
    //
    // let mut choice = String::new();
    //
    // io::stdin().read_line(&mut choice).expect("Failed to read choice");
    // println!("You have chosen {choice}");
    //
    // // Lets start a game
    //
    // let coin_toss = rand::thread_rng().gen_range(1..=2);
    // println!("The coin toss is: {coin_toss}");
}

fn print_board(board: [[char; 8]; 8]) {
    for rank in board {
        for square in rank {
            print!("{} ", square);
        }
        println!();
    }
}
