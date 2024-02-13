fn main() /* the main function is the entry point of many programs, the fn keyword allows you to declare new functions */
{
   println!("A simple print to screen in the main function");

   another_function(); /* Here we define another function, which will jump into the new one */
}

fn another_function() // This function can also be inside the fn main(), RUST does not care where they are, as long as they are defined
                      // within the scope to be seen by the caller. They are also called in order of when they appear.
{
    println!("This is the new function.");
}