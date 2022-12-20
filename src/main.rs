use std::io::{stdin, stdout, Write};

use crate::board::Board;
use crate::game::Game;
use crate::helpers::{print_player, Player};

mod board;
mod game;
mod helpers;

enum ParseError {
    OutOfBounds,
    Malformed,
    NotANumber,
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

    let x: u8 = match x.parse() {
        Ok(x) => x,
        Err(_) => return Err(ParseError::NotANumber),
    };
    let y: u8 = match y.parse() {
        Ok(y) => y,
        Err(_) => return Err(ParseError::NotANumber),
    };

    if x > 3 || y > 3 {
        return Err(ParseError::OutOfBounds);
    }

    if x < 1 || y < 1 {
        return Err(ParseError::OutOfBounds);
    }

    Ok((x - 1, y - 1))
}

pub fn input_move(board: &Board) -> (u8, u8) {
    loop {
        let mut buff = String::new();
        print!("Input your move: ");
        stdout().flush().expect("flush");
        stdin().read_line(&mut buff).expect("Input!");
        let coord = match parse_move(&buff) {
            Ok(coord) => coord,
            Err(er) => {
                match er {
                    ParseError::Malformed => {
                        println!("Input should have the format \"x y\", eg: 1 1.")
                    }
                    ParseError::NotANumber => println!(
                        "Input should only include two numbers and a space between them, eg: 2 3."
                    ),
                    ParseError::OutOfBounds => println!("Coordinates can only be between 1 to 3."),
                }

                continue;
            }
        };

        if board.check_move_possible(coord) {
            return coord;
        }

        println!(
            "The square {} {} is taken, please play another square.",
            coord.0, coord.1
        );
    }
}

fn main() {
    let mut winner: Option<Player> = None;

    println!("Welcome to Rust Tic Tac Toe! 😇\n");
    println!("Input your move by inputting the coordinates of the board position.");
    println!("The coordinates should be inputted in the following format: \"X Y\". Where X the row and Y the column. Coordinates should be from 1 to 3.\n");
    println!("Let's begin 🎉\n");

    print!("Enable 🤖 for X (1), O (2) or both (3)? ");
    stdout().flush().expect("Flush!");

    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("Input error!");

    let ai_switch: u8 = buf.trim().parse().expect("parse!");

    print!("Who plays first? Input X or O: ");
    stdout().flush().expect("Flush!");

    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("Input error!");

    let player = if buf.trim().contains("X") {
        Player::X
    } else {
        Player::O
    };

    let mut game: Game = Game::new(player);

    while winner == None {
        println!("\n");
        game.board.print();
        println!();
        println!(
            "{}: Player {} is making a move!\n",
            game.movenum + 1,
            print_player(&game.next_player)
        );

        let coord = match game.next_player {
            Player::X => match ai_switch {
                2 => input_move(&game.board),
                _ => match game.board.engine_v1(Player::X) {
                    Some(coord) => coord,
                    None => panic!("Can't happen!"),
                },
            },
            Player::O => match ai_switch {
                1 => input_move(&game.board),
                _ => match game.board.engine_v1(Player::O) {
                    Some(coord) => coord,
                    None => panic!("Can't happen!"),
                },
            },
        };
        println!();

        game.board
            .board_move(coord, game.next_player)
            .expect("incorrect");

        game.next_turn();
        if game.movenum > 9 {
            break;
        }
        winner = game.board.finish_condition()
    }

    game.board.print();
    println!();
    println!("Game finished!");
    match winner {
        Some(w) => println!("Player {} won!", print_player(&w)),
        None => println!("No winners!"),
    }
}
