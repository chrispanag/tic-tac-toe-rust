pub enum ParseError {
    OutOfBounds,
    Malformed,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Player {
    X,
    O,
}

pub fn print_player(p: &Player) -> char {
    match p {
        Player::X => 'X',
        Player::O => 'O',
    }
}

pub fn parse_move(input: &str) -> Result<(u8, u8), ParseError> {
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
