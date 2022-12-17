use crate::{board::Board, helpers::Player};

pub struct Game {
    pub next_player: Player,
    pub board: Board,
    pub movenum: u8,
}

impl Game {
    pub fn new(player: Player) -> Game {
        Game {
            next_player: player,
            board: Board::new(),
            movenum: 0,
        }
    }

    pub fn next_turn(&mut self) {
        self.next_player = match self.next_player {
            Player::X => Player::O,
            _ => Player::X,
        };
        self.movenum = self.movenum + 1;
    }
}
