use std::mem;

pub fn run() {
    let numbers= [1, 2];


    println!("nums: {:?}", numbers);
    println!("first elem: {:?}", numbers[0]);

    println!("array occupies {} bytes", mem::size_of_val(&numbers));

    let items_to_slice: &[i32]= &numbers[0..1];
    println!("{:?}", items_to_slice)
}