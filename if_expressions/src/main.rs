fn main() 
{
    let number = 7; // change this number higher if you wish to trigger the false statement

    if number < 5 // if number is less than 5 print to screen
    {
        println!("Condition was true, is less than 5");
    }
    else // if you don't provide an 'else' expression and the condition is false, the program will just skip the 'if' block and move on
    {
        println!("Condition was false, is greater than 5");
    }

    fail_example();

    fn fail_example()
    {
        let number = 3;

        if number != 0      // to correct this previous error, we can add '!= 0' which means not equal to.
                            // if we place in a number that makes it false, it will skip over this 'if'
        {
            println!("For our second example, which errored previously, the number was three"); // this will throw an error, it expected bool, instead gets an integer
                                          // With RUST, you must always be explicit and provide 'if' with a Boolean as it's condition.
        }
    }

}
