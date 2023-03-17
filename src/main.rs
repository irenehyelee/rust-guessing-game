use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    // 1) generate the secret number
    let secret: i32 = rand::thread_rng().gen_range(1..101);

    // 2) ask user number
    println!("Guess the number (hint {}):", secret);

    loop {
        // 3) read user number (as String)
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // 4) verify user number and tell user
        let guess: i32 = guess.trim().parse().expect("Failed, please type a number.");
        println!("You guessed: {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    // 5) loop (on 3 & 4)

    // 6) goodbye message
    println!("\nGoodbye\n");
}
