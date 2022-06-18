// We are calling a library, the 'use' will help us call the library and 'std' is the Rust Standard library 
use std::io;
use rand::Rng;
use std::cmp::Ordering;

// In this part of the game we are creating the main function
// In Rust, we always start with the main function whenever we are creating a new program. 
fn main() {
    // We are printing this information for the user 
    println!("This is the Guessing game: ");
    println!("Instructions: !");
    println!("The game will select a random number between 1 - 100 and you have to guess what number it is !");
    println!("Guess the number!");

    // Inmutable variable using the random library between 1 to 100
    let secret_number = rand::thread_rng().gen_range(1..101); 

    // Printing the secret number just to verify the code works in the game. 
    println!("The secret number is: {}", secret_number); 

    loop {
        println!("Please type your guess.");

        // This is a mutable variable, meaning this variable could change.
        let mut guess = String::new();  // We are using the string::new to direct it as a new string. 

        // We are calling the library
        io::stdin()
            .read_line(&mut guess) // This help us to out the input in the mutable guess variable
            .expect("Failed to read line");

            /* 'guess' is a mutable number and we are passing the variable
            into a integer to avoid errors with strings and integers 
            if we do not do that the program will crash. 
            Trim will eliminate any whitespace and parse will parse the method 
            to a an integer.
            expect will help us to recover the program just in case there's an error 
            let sliced_value = &data_structure[start_index..end_index]
            */
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

        println!("You guessed: {}", guess); // We are printing the variable guess. 
        
        match guess.cmp(&secret_number) { // Using the secret number as an arg with the match 
            Ordering::Less => println!("The number is too small!"), // if the number is less than the arg the prints 
            Ordering::Greater => println!("The number is too big!"), // if the number is greater than the arg the prints 
            Ordering::Equal => {
                println!("You got it, chief!"); // if the number is the exact arg then prints
                break; 
            }
        }
    }
    
    println!("Good work, the secret number was: {}", secret_number); 
}
