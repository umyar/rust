use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

// will be mod (module) in fufure
fn guess_number() {
    let secret_num = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Guess the number dumb Goofy! Your number is:");

        let mut guess = String::new();

        stdin()
            .read_line(&mut guess)
            .expect("Failed to read your line stupid human...");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small..."),
            Ordering::Greater => println!("Too big..."),
            Ordering::Equal => {
                println!("*** *** *** \n  Damn good! You right! \n*** *** ***");
                break;
            }
        }
    }
}
