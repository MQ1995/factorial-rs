use std::io::Write;
use num_bigint::BigUint;
use num_traits::One;

fn main() {
    println!("\nThis program will calculate the factorial of a given integer.\n");
    
    let prompt = "Please input an integer: ";
    let retry_msg = "Not an integer, retry. \n";

    let chosen_integer: u128 = get_int(prompt, retry_msg);

    println!("You chose {chosen_integer}. Here's the calculation:\n");

    let mut counter: BigUint = One::one();

    for i in (1..=chosen_integer).rev() {
        if i > 1 {
            print!("{i} × ");
        }
        else {
            print!("{i} =\n");
        }
        counter *= i;
    }
    std::io::stdout().flush()
                     .expect("Unable to flush output buffer");
    
    println!("{counter}");
}

// Tries to get an integer from user
// returns it on success, else tries again

fn get_int(prompt: &str, retry_message: &str) -> u128 {
    loop {

        print!("{prompt}");
        std::io::stdout().flush()
                     .expect("Unable to flush output buffer");

        let mut input: String = String::new();

        std::io::stdin().read_line(&mut input)
                        .expect("Failed to read line.");

        match input.trim().parse::<u128>() {
            Ok(int) => return int,
            Err(_) => {
                println!("{retry_message}");
                input.clear();
                continue;
            },
        };
    }
}
