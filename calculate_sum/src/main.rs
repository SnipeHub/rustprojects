use std::io;

/* Calculate the sum of two numbers */
fn main() 
{
    // get the first number from the user
    println!("Enter the first number:");
    let mut num1 = String::new(); // we create our variable num1 and give it a string funcion for user input
    io::stdin().read_line(&mut num1).expect("Failed to read line");

    // Convert the first number to a floating-point number
    let num1: f64 = num1.trim().parse().expect("Failed to convert number");

    // Get the second number from the user
    println!("Enter the second number:");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("Failed to read line");

    // Convert the second number to a floating-point number
    let num2: f64 = num2.trim().parse().expect("Failed to convert number");

    // Calculate the sum of the two numbers then print it out
    let sum = num1 + num2;
    println!("The sum of {num1} and {num2} is {sum}");
}
