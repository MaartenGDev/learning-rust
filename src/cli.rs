use std::env;
use rand::Rng;

pub fn run(){
    let args: Vec<String> = env::args().collect();
    let command = if args.len() > 1 { args[1].clone() } else {"default".to_string()};

    println!("Args: {:?}, id: {}", command, rand::thread_rng().gen_range(1, 101))
}