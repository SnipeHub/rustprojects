use std::io;

fn main() 
{ // setting up user input
    let mut input_a = String::new(); // Don't forget to make these mutable
    let mut input_b = String::new();
    let mut op = String::new(); // Our operator variable, this will print out our options

    println!("Enter the first number:"); io::stdin().read_line(&mut input_a) // our user input for the first number
    .expect("Invalid Input, please enter a valid number"); // our error handling, in case of a dodgy input by user :-)

    println!("Enter the second number:"); io::stdin().read_line(&mut input_b) // our user input for the second number
    .expect("Invalid Input, please enter a valid number");

    println!("List of operators:"); // Our list of options for the user to choose from, choosing the calculation
    println!(" (1) Add +");
    println!(" (2) Subract -");
    println!(" (3) Multiply *");
    println!(" (4) Divide /");
    println!("Select the number associated with the desired operation:");
        io::stdin().read_line(&mut op)
        .expect("Invalid Input: Please enter a number between 1 - 4");

    /* Parsing inputs
    We need to parse the input strings into numeric values */
    let input_a: f32 = input_a.trim().parse() // here we are declaring the variable as a type f32, can do positive and negative +0.0 and -0.0, floating-point
    .expect("Invalid Input");

    let input_b: f32 = input_b.trim().parse()
    .expect("Invalid Input");

    let op: u32 = op.trim().parse()
    .expect("Invalid Input");
/* we use .trim() method here which returns a copy of the string with leading and trailing whitespace removed */
/* the .parse() method parses a string into some kind of value, in this case, to parse the trimmed string into a f32/u32 value */

    /* Performing Calculations */
match op 
{
    1 => println!("Result: {}", input_a + input_b), // Addition
    2 => println!("Result: {}", input_a - input_b), // Subtraction
    3 => println!("Result: {}", input_a * input_b), // Multiplication
    4 => 
    {
        if input_b != 0.0 
        {
            println!("Results: {}", input_a / input_b); // Division
        }
            else 
            {
                println!("Error: Cannot divide by zero");
            }
    }
    _ => println!("Invalid operation selection"),
}
}
