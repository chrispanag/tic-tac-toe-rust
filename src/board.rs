use std::io::stdin;
use crate::helpers::{Player, print_player, parse_move};

type BoardType = [[Option<Player>; 3]; 3];

pub struct Board {
    pub board: BoardType,
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: [[None; 3]; 3],
        }
    }

    pub fn print(&self) {
        for line in self.board {
            for (i, el) in line.iter().enumerate() {
                match el {
                    Some(x) if i < 2 => print!("{}\t|\t", print_player(x)),
                    Some(x) => print!("{}", print_player(x)),
                    None if i < 2 => print!("_\t|\t"),
                    None => print!("_"),
                }
            }
            println!()
        }
    }

    pub fn check_move_possible(&self, coord: (u8, u8)) -> bool {
        let (x, y) = coord;
        let (x, y) = (x as usize, y as usize);
        match self.board[x as usize][y as usize] {
            Some(_) => false,
            None => true,
        }
    }

    pub fn board_move(&mut self, coord: (u8, u8), player: Player) -> Result<(), &str> {
        let (x, y) = coord;
        let (x, y) = (x as usize, y as usize);
        if Self::check_move_possible(self, coord) {
            self.board[x][y] = Some(player);
            return Ok(());
        }
        return Err("There is already a move there!");
    }

   pub fn engine(&self) -> Option<(u8, u8)> {
        for (i, row) in self.board.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                let coord = (i as u8, j as u8);
                if Self::check_move_possible(self, coord) {
                    return Some(coord);
                }
            }
        }

        return None;
    }

    pub fn input_move(&self) -> (u8, u8) {
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

            if Self::check_move_possible(self, coord) {
                return coord;
            }

            println!("There is already a move there!");
        }
    }

    pub fn finish_condition(&self) -> Option<Player> {
        let mut conditions_lines: [Option<Player>; 3] = [None; 3];
        let mut conditions_rows: [Option<Player>; 3] = [None; 3];
        let mut conditions_diagonal_1: Option<Player> = None;
        let mut conditions_diagonal_2: Option<Player> = None;
        for (i, line) in self.board.iter().enumerate() {
            for (j, el) in line.iter().enumerate() {
                if j == 0 {
                    conditions_rows[i] = *el;
                    if i == 0 {
                        conditions_diagonal_1 = *el;
                    }
                }
                if i == 0 && j == 2 {
                    conditions_diagonal_2 = *el;
                }

                if i + j == 2 {
                    if conditions_diagonal_2 != *el {
                        conditions_diagonal_2 = None;
                    }
                }

                if i == j {
                    if conditions_diagonal_1 != *el {
                        conditions_diagonal_1 = None;
                    }
                }

                if conditions_rows[i] != *el {
                    conditions_rows[i] = None;
                }

                if i == 0 {
                    conditions_lines[j] = *el;
                }

                if conditions_lines[j] != *el {
                    conditions_lines[j] = None;
                }
            }

            if conditions_rows[i] != None {
                return conditions_rows[i];
            }
        }

        if conditions_diagonal_1 != None {
            return conditions_diagonal_1;
        }

        if conditions_diagonal_2 != None {
            return conditions_diagonal_2;
        }

        for i in conditions_lines {
            if i != None {
                return i;
            }
        }

        None
    }
}
