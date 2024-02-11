/* Normally you would do a loop, if, else, and break, this pattern is so common that Rust has a built-in language construct for it,
called a 'while' loop. Below we are going to use 'while' to loop the program 3 times, counting down each time, and then, after the loop, print a message
and exit.*/

fn main()
{
    let mut number = 3;

    while number != 0 
    {
        println!("{number}!");

        number -= 1; // decreases 'number' variable by 1 each time it is run 
    }

    println!("LIFTOFF!!!");
}