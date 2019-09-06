pub fn run() {
    let welcome_message = "helloooo world!";
    println!("{}", welcome_message.len());

    let mut expandable_string = String::from("hello from mars!");
    expandable_string.push_str("nice jobdddd!");

    println!("{}", expandable_string);
    println!("Capacity {}", expandable_string.capacity());

    println!("contains, {}",welcome_message.contains("world"));
    println!("replaced, {}",welcome_message.replace("world", "space"));

    for token in welcome_message.split_whitespace(){
        println!("token: {}", token)
    }

    let mut growable_string = String::with_capacity(10);
    growable_string.push('a');
    growable_string.push('b');
    growable_string.push('c');

    assert_eq!(5, 10 / 2);

    println!("{}", growable_string)
}