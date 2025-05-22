enum Direction {
    North,
    East,
    West,
    South,
}
fn main() {
    let directionE = Direction::East;
    let directionN = Direction::North;
    let directionS = Direction::South;
    let directionW = Direction::West;
    // let directionW = Direction::Up;
    // We will get error because uo is not an enum
    // print!("{}{}{}{}", directionE, directionW, directionN, directionS)
}
