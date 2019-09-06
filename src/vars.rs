pub fn run()  {
    let name = "Maarten";
    let age = 20;

    let next_age = age + 20;

    println!("{} is {} years old",name, next_age);

    const ID: i32 = 22;
    println!("{}",ID);

    let (name, age) = ("maarten", 20);
    println!("{}, {}", name, age);

}