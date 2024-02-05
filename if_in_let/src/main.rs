fn main() 
{
    let condition = true; // change this to true or false, true will be 5, false will be 6
    let number = if condition {5} else {6}; /* remember that blocks of code evaluate to the last expression in them, and numbers by themselves
    are also expressions. In this case, the value of the whole 'if' expression depends on which block of code executes.
    This means the values that have the potential to be results from each arm of the if must be the same type,
    in this case, type i32, which is an integer. You can't have a string in one and a integer in the other. */

    println!("The value of the number is: {number}");
}
