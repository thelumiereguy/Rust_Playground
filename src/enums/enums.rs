pub fn run() {
    move_avatar(Movement::Up);
    move_avatar(Movement::Right);
    move_avatar(Movement::Down);
}


enum Movement {
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(movement: Movement) {
    match movement {
        Movement::Up => {
            println!("Moving Up")
        }
        Movement::Down => {
            println!("Moving Down")
        }
        Movement::Left => { println!("Moving Left") }
        Movement::Right => { println!("Moving Right") }
    }
}
