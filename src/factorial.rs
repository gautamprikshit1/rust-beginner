use rand::Rng;

fn factorial(num: i32)-> i32 {
    if num == 1 { return 1; }
    else { return num * factorial(num-1); }
}

pub fn fact_random_number() {
    let num = rand::thread_rng().gen_range(1..11);
    println!("Factorial of {} is {}", num, factorial(num));
}