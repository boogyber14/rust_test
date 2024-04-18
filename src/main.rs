// Import the rand crate
use rand::Rng;

fn main() {
    // Generate a random number between 100 and 1000
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(100..=1000);

    // Print the random number
    println!("Random number: {}", random_number);
}
