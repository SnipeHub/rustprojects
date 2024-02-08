fn main() /* Loops are all about repating code, where by the code is executed to the end, then loops back to the beginning
          RUST has three kinds of loops: loop, while, and for. */

          // loop
{
   loop 
   {
        println!("Looping!"); //I have added a break to this, otherwise this will loop with no end

        break;
   }


return_value_loop();
}
fn return_value_loop() 
{
    let mut counter = 0; // mutable variable called counter declared

    let result = loop   // declare a variable named result to hold the value returned from the loop
    {
        counter += 1; // each loop we add + 1 to the counter variable

        if counter == 10 // we now check the counter variable if it equals 10, if it does, it will use the break 
        {
            break counter * 2; // it will now multyply counter by 2, then print the results
        }
    }; // important not to forget the semicolon to end the statement

    println!("The result is {result}");
}