use std::mem;

pub fn run() {
    let mut numbers = vec![1,2,3];

    numbers.push(22);
    numbers.push(22);

    println!("{:?} {}",numbers, mem::size_of_val(&numbers));

    for item in numbers {
        println!("{}",item);
    }
}