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
    name: String,
    symbol: char,
    colour: Colour,
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.colour, self.name)
    }
}

fn generate_first_rank() -> [Piece; 5] {
    let rook = Piece {
        name: String::from("Rook"),
        symbol: 'R',
        colour: Colour::White,
    };
    let knight = Piece {
        name: String::from("Knight"),
        symbol: 'N',
        colour: Colour::White,
    };
    let bishop = Piece {
        name: String::from("Bishop"),
        symbol: 'B',
        colour: Colour::White,
    };
    let queen = Piece {
        name: String::from("Queen"),
        symbol: 'Q',
        colour: Colour::White,
    };
    let king = Piece {
        name: String::from("King"),
        symbol: 'K',
        colour: Colour::White,
    };

    [rook, knight, bishop, queen, king]
}

fn main() {
    let my_colour = Colour::White;
    let my_piece = Piece {
        name: String::from("King"),
        symbol: 'K',
        colour: my_colour,
    };
    let my_square: Square = Square::Piece(my_piece);

    println!("My colour is: {}", my_colour);
    println!("My square is: {}", my_square);

    let first_rank = generate_first_rank();

    for piece in first_rank {
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
