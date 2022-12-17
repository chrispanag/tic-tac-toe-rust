use core::panic;

use crate::helpers::{print_player, Player};

const BOARD_SEPARATOR: char = '┃';
const EMPTY_POSITION: char = '－';

#[derive(Copy, Clone)]
pub struct Board {
    pub board: [[Option<Player>; 3]; 3],
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: [[None; 3]; 3],
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

    pub fn print(&self) {
        for (j, line) in self.board.iter().enumerate() {
            if j > 0 {
                println!("－－－－－－－－－{BOARD_SEPARATOR}－－－－－－－－－{BOARD_SEPARATOR}－－－－－－－－－");
            }
            for (i, el) in line.iter().enumerate() {
                match el {
                    Some(x) if i < 2 => {
                        print!("        {}        {BOARD_SEPARATOR}", print_player(x))
                    }
                    Some(x) => print!("        {}", print_player(x)),
                    None if i < 2 => {
                        print!("        {EMPTY_POSITION}        {BOARD_SEPARATOR}")
                    }
                    None => print!("        {EMPTY_POSITION}"),
                }
            }
            println!();
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

    fn gen_score(&self, coord: (u8, u8), player: Player) -> i32 {
        let mut count: i32 = 0;
        for (i, column) in self.board.iter().enumerate() {
            if column[coord.0 as usize] == None {
                count = count + 1;
            }
            if column[coord.0 as usize] == Some(player) {
                count = count + 2;
            }

            for (j, row) in column.iter().enumerate() {
                if i == coord.1 as usize {
                    if *row == None {
                        count = count + 1;
                    }
                    if *row == Some(player) {
                        count = count + 2;
                    }
                }

                if coord.0 + coord.1 == 2 {
                    if i + j == 2 {
                        if *row == None {
                            count = count + 1;
                        }
                        if *row == Some(player) {
                            count = count + 2;
                        }
                    }
                }

                if coord.0 == coord.1 {
                    if i == j {
                        if *row == None {
                            count = count + 1;
                        }
                        if *row == Some(player) {
                            count = count + 2;
                        }
                    }
                }
            }
        }

        return count;
    }

    fn get_possible_moves_for_player(
        &self,
        possible_moves: &mut [(Option<(u8, u8)>, i32); 9],
        player: Player,
    ) -> usize {
        let mut sum = 0;
        for (i, row) in self.board.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                let coord = (i as u8, j as u8);
                if Self::check_move_possible(self, coord) {
                    let score = Self::gen_score(self, coord, player);
                    possible_moves[sum] = (Some(coord), score);
                    sum = sum + 1;
                }
            }
        }

        return sum;
    }

    // Naive AI engine. Right now it only chooses the first available move.
    pub fn engine_v1(&self, player: Player) -> Option<(u8, u8)> {
        let mut possible_moves: [(Option<(u8, u8)>, i32); 9] = [(None, 0); 9];
        Self::get_possible_moves_for_player(&self, &mut possible_moves, player);

        let mut max_score = 0;
        let mut max_coord: Option<(u8, u8)> = None;
        for (coord, score) in possible_moves.iter().filter(|(coord, _)| *coord != None) {
            match coord {
                Some((x, y)) => {
                    println!("{x}, {y}, {score}");
                    if max_score < *score {
                        max_score = *score;
                        max_coord = *coord;
                    }
                }
                None => continue,
            }
        }
        if max_coord == None {
            max_coord = match possible_moves.iter().find(|(coord, _)| *coord != None) {
                Some((coord, _)) => *coord,
                None => panic!("unexpected!"),
            }
        }

        return max_coord;
    }

    // There's surely a better way to do that.
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
