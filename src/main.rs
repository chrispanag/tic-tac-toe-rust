use crate::board::Board;
use crate::helpers::{print_player, Player};

mod board;
mod helpers;

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
            Player::X => board.input_move(),
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
