enum Movement {
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movement) {
    match m {
        Movement::Up => println!("Up!"),
        Movement::Down => println!("Down!"),
        Movement::Left => println!("Left!"),
        Movement::Right => println!("Right!")
    }
}
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

pub fn run() {
    move_avatar(Movement::Left);


    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}