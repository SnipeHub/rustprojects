use std::io; // input/output from the standard library
use rand::Rng; // random number generator trait
use std::cmp::Ordering;

fn main() 
{
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // we add our random generator, 1 to 100

    //println!("The secret number is: {secret_number}"); // we print it to screen, use this for testing purposes

    loop // We add this section into a loop, so the user can guess more than once
    {
        println!("Please input your guess.");

        let mut guess = String::new(); // ::new this syntax means that new is an associated function of the string type

        io::stdin()
            .read_line(&mut guess) // This grabs user input
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() /* Here we are using Shadowing, which we re-use an existing variable, 
        saving us having to create a new variable. We are also converting the string to use only an integer, it will error if anything other
        than a number is typed. */
        { // We also added in a check for invalid input
            Ok(num) => num,
            Err(_) => continue, // (_) this is a catchall
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) // this will compare two values, in this case, the input from the user, then compare it to the random number
        {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => 
            {
                println!("You win!");
                break; // we adda break here so the application ends once you guess correctly, otherwise the loop won't end
            }
        }
    }

}