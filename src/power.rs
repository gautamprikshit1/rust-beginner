use rand::Rng;

pub fn powers() {
    let num1 = rand::thread_rng().gen_range(1..11);
    let num2 = rand::thread_rng().gen_range(1..11);
    let result: i128 = i128::pow(num1, num2);
    println!("{}^{} = {}", num1, num2, result);
}