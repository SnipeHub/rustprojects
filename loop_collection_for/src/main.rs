use std::thread;
use std::time::Duration;

/* Looping Through a Collection with for
You can choose to use the while construct to loop over the elements of a collection, such as an array.
Here, the code counts up through the elements in the array. It starts at index 0, and then loops until it reaches the final index in the array 
(that is, when index < 5 is no longer true). */
fn main() 
{
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 
    {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    better_for_loop(); /* See below for a better way to do the above example */
    range_example(); /* a for loop using 'range', provided by the standard library */
    extra_real_example();
}

fn better_for_loop() // a cleaner way to do it
{
    let a = [10, 20, 30, 40, 50];

    for element in a  /* This way decreases the chance for bugs or errors, much simpler. Also we could always add to the array without touching
    any other code. This variable 'element' does not need to be decalred, which has confused me, but apparentyl the way for loops are design in Rust.
    It automatically knows this is a variable */
    {
        println!("ex2:the value is: {element}");
    }
}
/* The safety and conciseness of 'for' loops makes them the most commonly used loop construct in Rust */
fn range_example()
{
    for number in (1..11).rev() // This is a reverse iteration
    {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn extra_real_example() // In this example, it will count down, but there will be a pause of 1 sec
                        // Also I had to add some extra options into the scope, see above the main function()
{
    for number in (1..4).rev()
    {
        println!("{number}!");
        thread::sleep(Duration::from_secs(1));
    }
    println!("Everyone, eyes on me!");
}