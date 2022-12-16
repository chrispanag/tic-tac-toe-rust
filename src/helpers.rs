#[derive(Copy, Clone, PartialEq)]
pub enum Player {
    X,
    O,
}

pub fn print_player(p: &Player) -> char {
    match p {
        Player::X => 'Ｘ',
        Player::O => '〇',
    }
}

