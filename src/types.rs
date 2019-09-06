pub fn run() {
    let my_float = 1.2;

    println!("Max i32: {}", std::i32::MAX);
    println!("Max i32: {}", std::i64::MAX);

    let is_greater = 2 > 1;

    let mood = '🎨';

    if is_greater {
        println!("{}, Is greater!", mood)
    }
}