use rand::Rng;
use std::io::{self, Write};

pub fn dice() {
    print!("Please Enter your name: ");
    io::stdout().flush().ok().expect("Could not flush stdout");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to take input.");
    println!("Rolling the dice for {}", name);
    loop {
        let dice = rand::thread_rng().gen_range(1..7);
        println!("You got: {}", dice);
        if dice != 6 { break; }
        else {
            println!("Rolling again...");
        }
    }
}
