// enum
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
fn way(go: Direction) {
    match go {
        Direction::Up => println!("up"),
        Direction::Down => println!("down"),
        Direction::Left => println!("left"),
        Direction::Right => println!("right"),
    };
}

fn main() {
    let go = Direction::Up;
    way(go)
}
