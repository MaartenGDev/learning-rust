enum Movement {
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Movement){
    match m {
        Movement::Up => println!("Up!"),
        Movement::Down => println!("Down!"),
        Movement::Left=> println!("Left!"),
        Movement::Right=> println!("Right!")
    }
}

pub fn run(){
    move_avatar(Movement::Left)
}