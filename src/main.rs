use std::io;
use std::env;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let args: Vec<String> = env::args().collect();

    let low: i32 = args[1].parse().unwrap();
    let high: i32 = args[2].parse().unwrap();

    println!("Low: {}", low);
    println!("High: {}", high);

    let mut rng = rand::thread_rng();
    let magic_number = rng.gen_range(low..high);
    println!("magic_number = {}", magic_number);

    loop {
        let mut line = String::new();
        println!("Enter guess: ");
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let guess: i32 = line.trim().parse().expect("Please type a number!");

        println!("guess = {}", guess);

        match guess.cmp(&magic_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
