fn main() 
{
    let number = 6;

    if number % 4 == 0
        {
        println!("number is divisible by 4");
        }    
    else if number % 3 == 0  
        {
        println!("number is divisible by 3"); // by default, only this prints out the first true that appears, and ignores the rest. 
                                              //RUST only grabs the first true it gets, and doesnt check after that

                                    /* If you need to use multiple 'else if', better to use a construct called 'match', see Chp 6 of the RUST book */
        }
    else if number % 2 == 0 // if you were to comment out the above 'else if' section, this will now pop next
        {
        println!("number is divisible by 2");
        }
    else 
        {
        println!("number is not divisible by 4, 3, or 2");
        }

    test_two(); /* multiple if expressions example */
    match_test(); /* I cheated and jumped ahead to have a look at a match construct :-) */
    shipping_costs(); /* one final example for when you might need multiple 'if' true checks, for a possible real world example */
}

fn test_two() /* Here I will re-write the above to explicitly check each if, and if true, print it to screen.
                 This is merely one way of doing it, better to use a match construct, in other languages like C# would use a switch statement.
                 Keep in mind neither is faster or better than the other, it just comes down to what you require.
                 so far this appears to be the better option if you have or need multiple true checks and don't want it to skip over*/
{
let is_true = 6;

    if is_true % 4 == 0
        {
        println!("ex2:number is divisible by 4");
        }
    if is_true % 3 == 0
        {
        println!("ex2:number is divisible by 3"); // this will print
        }
    if is_true % 2 == 0
        {
        println!("ex2:number is divisible by 2"); // this will also print, so you will have two answers
        }
    else // if none of the above are true, this one will print, you can test by changing it to a number you know will not divide
        {
        println!("ex2:number is not divisible by 4, 3, or 2")    
        }

}

fn match_test() /* This is probably not a good example, as you would most likely not be using this many 'if' checks, most likely it would be just true or false */
{
let is_true = 6;

    match is_true
    {
        _ if is_true % 4 == 0 => println!("ex3:number is divisible by 4"),
        _ if is_true % 3 == 0 => println!("ex3:number is divisible by 3"), // I noticed this is the first true, and prints, but it is skipping the last 'if'
        _ if is_true % 2 == 0 => println!("ex3:number is divisible by 2"), // this is being skipped, so a match only grabs the first true, then skips the rest
        _ => println!("ex3:number is not divisible by 4, 3, or 2"),
    }
}

fn shipping_costs() // this has helped me understand the possiblity's with if expressions, very interesting, this example also works well
{
let package_weight = 23.0; // in kilograms
let destination = "Europe"; // change this to your liking. If you enter an unknown destination, the 'else' will print

if package_weight > 20.0 // check if package weight is greater than 20 kg 
    {
    println!("ex4:This is a heavy package. Additional charges may apply.");
    }
if destination == "Europe" // check if destination is Europe
    {
    println!("ex4:Shipping to Europe. International shipping rates will apply.");
    }
else if destination == "USA" // by adding an 'else if' it will check for it if Europe is no longer true, test it by changing the destination
    {
    println!("ex4:Shipping to the USA. Domestic shipping rates will apply.");
    }
else if destination == "Australia"
    {
    println!("ex4:Shipping to Australia. International shipping rates will apply.");
    }
else
    {
    println!("ex4:Invalid destination. Please enter a valid destination.");    
    }

}