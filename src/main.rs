use notify_rust::Notification;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // Givin user instruction to input value
    println!("Welcome to the guessing game...! \n Enter your guess in this line");

    // Render a random digit to 1..10
    let radgit: u32 = rand::rng().random_range(..10);

    // Create a infinite loop to give infinite attempts to user
    loop {
        // Initialize a placeholder as a string
        let mut place_holder: String = String::new();

        // read the terminal user input using io read_line method with err handling
        io::stdin()
            .read_line(&mut place_holder)
            .expect("Oops somting went wrong...");

        // Trasform the input to match with the machine guess with err handling
        let parsed_in: u32 = match place_holder.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // printing user inputs
        // println!("Machine guess is ... {}", radgit);
        println!("Anddd your guess is ... {}", parsed_in);

        // Writing a match statement with Ordering arms and a function notify that run and end on success
        match parsed_in.cmp(&radgit) {
            Ordering::Greater => println!("Too large"),
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                notify("You succeed, Your guess is correct", radgit);
                // End the loop when the guess is correct also end the game
                break;
            }
        }
    }
}

fn notify(s: &str, n: u32) -> () {
    // Generating notification msg
    let msg: String = format!("Your guess is {}", n);

    //using notify crate to notify the success msg in native machine notification calls
    let _ = Notification::new().summary(s).body(&msg).show();
}
