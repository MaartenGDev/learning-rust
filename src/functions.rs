pub fn run() {
    greeting("hello", "maarten");
    println!("{}", add_numbers(1,2));

    let get_sum = |num: i32| add_numbers(2, num);

    println!("sum: {}", get_sum(4));

    let offset = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + offset;
    println!("{}", add_nums(3,3))

}

fn greeting(greet: &str, name: &str){
    println!("{} {}, nice to meet!", greet, name)
}

fn add_numbers(number1: i32, number2: i32) -> i32 {
    number1 + number2
}