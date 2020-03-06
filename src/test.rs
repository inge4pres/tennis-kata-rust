use super::*;

#[test]
fn player_point() {
    let p1 = Point::Love;
    let p2 = Point::Forty;

    assert_eq!(p1.to_int(), 0);
    assert_eq!(p2.to_int(), 40);
}

#[test]
fn game_score() {
    let p1 = Point::Fifteen;
    let p2 = Point::Forty;

    assert_eq!(score(p1, p2), Score::PlayerWins(p1, p2));
}

#[test]
fn game_score_deuce() {
    let p1 = Point::Forty;
    let p2 = Point::Forty;

    assert_eq!(score(p1, p2), Score::Deuce);

    let p1 = Point::Other(4);
    let p2 = Point::Other(4);

    assert_eq!(score(p1, p2), Score::Deuce);
}

#[test]
fn game_score_surpasses_deuce() {
    let p1 = Point::Other(4);
    let p2 = Point::Other(6);

    assert_eq!(score(p1, p2), Score::PlayerWins(p1, p2));
}