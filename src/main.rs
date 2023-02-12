use rand::Rng;

fn main() {
    println!("Length of your Password: ");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    if let Ok(number) = line.trim().parse::<i32>() {
        let password = generate_password(number.try_into().unwrap());
        println!("You entered a number: {}", password);
    } else {
        println!("You entered a string please enter length in number");
    }
}

fn generate_password(length: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| rng.gen_range(0..10).to_string())
        .collect()
}
