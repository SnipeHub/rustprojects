fn five() -> i32 // we declare our function with a return value as an i32 type
{
    5 // value, we also do not name our values, this is an expression, no new line
}

fn main() 
{
    let x = five(); // we define our variable and call the function

    println!("The value of x is: {x}");

    // for example 2
        let y = plus_one(5); // we define our value inside the function brackets

        println!("The value of y for our second example is: {y}");

    /*example 2 */
    fn plus_one(y: i32) -> i32 // we declare the variable name and it's type inside the brackets
    {
        y + 1 // if you were to add a semicolon to the end of this, turning it into a statement, it will error, remember this is an expression.
    }
}