
// Fill the blanks
enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North  => { // Matching South or North here
            println!("North");
        }
        Direction::South  => { // Matching South or North here
            println!("South");
        }
        _ => println!("West"),
    };
}

