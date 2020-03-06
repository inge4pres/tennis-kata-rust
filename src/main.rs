use std::cmp::PartialEq;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Point {
    Love,
    Fifteen,
    Thirty,
    Forty,
    Other(u32),
}

#[derive(Debug, PartialEq)]
enum Score {
    LoveLove,
    Playing(Point, Point),
    PlayerWins(Point, Point),
    Advantage(Point, Point),
    Deuce,
}

impl Point {
    fn to_int(self) -> u32 {
        return match self {
            Point::Love => 0,
            Point::Fifteen => 15,
            Point::Thirty => 30,
            Point::Forty => 40,
            Point::Other(x) => x
        };
    }
}

fn score(p1: Point, p2: Point) -> Score {
    return match (p1, p2) {
        (Point::Love, Point::Love) => Score::LoveLove,
        (Point::Forty, Point::Love) => Score::PlayerWins(p1, p2),
        (Point::Forty, Point::Thirty) => Score::PlayerWins(p1, p2),
        (Point::Forty, Point::Fifteen) => Score::PlayerWins(p1, p2),
        (Point::Love, Point::Forty) => Score::PlayerWins(p1, p2),
        (Point::Fifteen, Point::Forty) => Score::PlayerWins(p1, p2),
        (Point::Thirty, Point::Forty) => Score::PlayerWins(p1, p2),
        (Point::Forty, Point::Forty) => Score::Deuce,
        (Point::Other(x), Point::Other(y)) => has_win(x, y),
        _ => Score::Playing(p1, p2)
    };
}

fn has_win(x: u32, y: u32) -> Score {
    let ix = x as i32;
    let iy = y as i32;
    match (ix - iy).abs() {
        0 => Score::Deuce,
        1 => Score::Advantage(Point::Other(x), Point::Other(y)),
        _ => Score::PlayerWins(Point::Other(x), Point::Other(y))
    }
}

#[cfg(test)]
mod test;
