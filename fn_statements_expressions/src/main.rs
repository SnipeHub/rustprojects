fn main() 
{
    /* Statements are instructions that perform some action and do not return a value.
       Expressions evaluate to a resultant value. Let’s look at some examples. */

    let x = 6; // declaring one statement, function definitions are also statements
    println!("The value of x is: {x}");
    /* because statements do not retunr values, you can't assign a let statement to another variable, you will get an error */
    // let x = (let y = 6);
    expressions();
}

fn expressions()
{
    /* Expressions example */
    let y = { let x = 3;
            x + 1 // notice how there is no ; end line, expressions do not have these, if you add that, it becoems a statemment and then does not return a value.
    };
    println!("The value of y is: {y}"); // the value output is 4
}