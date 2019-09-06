pub fn run() {
    let age = 21;
    let check_id = false;
    let knows_person_of_age = true;

    if age >= 21 && check_id || knows_person_of_age {
        println!("Free beer! for {}", age);
    }else if age < 21 && check_id {
        println!("Free water! for {}", age);
    }else{
        println!("lets check your id")
    }

    let is_of_age = if age >= 21 {true} else {false};
    println!("is of age: {}", is_of_age)
}