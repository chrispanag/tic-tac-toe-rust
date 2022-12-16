use std::io::stdin;

use crate::board::Board;
use crate::helpers::{print_player, Player};

mod board;
mod helpers;

enum ParseError {
    OutOfBounds,
    Malformed,
}

fn parse_move(input: &str) -> Result<(u8, u8), ParseError> {
    let mut it = input.trim().split(' ');
    let x = match it.next() {
        None => return Err(ParseError::Malformed),
        Some(x) => x,
    };
    let y = match it.next() {
        None => return Err(ParseError::Malformed),
        Some(y) => y,
    };

    let x = match x.parse() {
        Ok(x) => x,
        Err(_) => return Err(ParseError::Malformed),
    };
    let y = match y.parse() {
        Ok(y) => y,
        Err(_) => return Err(ParseError::Malformed),
    };

    if x > 2 || y > 2 {
        return Err(ParseError::OutOfBounds);
    }

    Ok((x, y))
}

pub fn input_move(board: &Board) -> (u8, u8) {
    loop {
        let mut buff = String::new();
        stdin().read_line(&mut buff).expect("Input!");
        let coord = match parse_move(&buff) {
            Ok(coord) => coord,
            Err(_) => {
                println!("Malformed input!");
                continue;
            }
        };

        if board.check_move_possible(coord) {
            return coord;
        }

        println!("There is already a move there!");
    }
}

fn main() {
    let mut board: Board = Board::new();

    let mut player = Player::X;
    let mut movenum: u8 = 0;
    let mut winner: Option<Player> = None;

    while winner == None {
        println!();
        println!();
        board.print();
        println!();
        println!(
            "{}: Player {} is making a move!",
            movenum + 1,
            print_player(&player)
        );
        println!();

        let coord = match player {
            Player::X => input_move(&board),
            Player::O => match board.engine() {
                Some(coord) => coord,
                None => break,
            },
        };

        board.board_move(coord, player).expect("incorrect");

        // Next player
        player = match player {
            Player::X => Player::O,
            _ => Player::X,
        };
        movenum = movenum + 1;
        if movenum > 9 {
            break;
        }
        winner = board.finish_condition()
    }

    board.print();
    println!();
    println!("Game finished!");
    match winner {
        Some(w) => println!("Player {} won!", print_player(&w)),
        None => println!("No winners!"),
    }
}
