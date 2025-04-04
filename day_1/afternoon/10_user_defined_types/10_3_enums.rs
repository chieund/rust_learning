#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
enum PlayerMove {
    Pass,
    Run(Direction),
    Teleport { x: u32, y: u32 }, // Struct variant
}

fn main() {
    let dir = Direction::Left;
    println!("dir: {:?}", dir);
    let player_move: PlayerMove = PlayerMove::Run(dir);
    println!("On this turn: {:?}", player_move); // Corrected line
}
