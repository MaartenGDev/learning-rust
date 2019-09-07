pub fn run() {
    let mut count = 0;

    loop {
        count += 1;
        println!("count: {}", count);

        if count == 20 {
            break;
        }
    }

    let mut fizzer = 0;

    while  fizzer <= 100 {
        if fizzer % 15 == 0 {
            println!("Fizzbuzz")
        } else if fizzer % 3 == 0 {
            println!("Fizz")
        } else if fizzer % 5 == 0 {
            println!("Buzz")
        }else{
            println!("{}", fizzer)
        }
        fizzer += 1;
    }

    for num in 0..100{
        println!("{}",num);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    println!("{:?}", v)

}