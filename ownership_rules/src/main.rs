fn main() 
{
    /* Ownership rules
    - Each value in Rust has an owner
    - There can only be one onwer at a time
    - When the owner goes out of scope, the value will be dropped */

    // Variable scope. A scope is the range within a program for which an item is valid

let s = "hello"; // s is valid from this point forward
    // do stuff with s
                // this scope is now over, and s is no longer valid

                /* The string type
                Below example is a string literal, where a string value is hardcoded into our program.
                They are convenint, but not alwast suitable for all cases. One reason is they are immutable.
                We may not know our value, for example it requires user input to provide the value */
let s = String::from("hello");

                /* This is our string type, which is mutable */
let mut s = String::from("hello");

s.push_str(", world!"); // push_str() appends a literal to a String

println!("{s}"); // This will print 'hello world!'

multiple_variables(); // Variables and data interacting with move
}

fn multiple_variables() /* We can probably guess what this is doing: “bind the value 5 to x; then make a copy of the value in x and bind it to y.”
We now have two variables, x and y, and both equal 5.  */
{
    let x = 5;
    let y = x; // these two values are both pushed to the stack

    // now for the next example, the string version
    let s1 = String::from("hello");
    let s2 = s1;
    /* This looks very similar to the above, but this is not what's happening.
    essentially what is happening is, the pointer is copying the data, but both s1 and s2 are pointing to
    the same data. For Rust, to ensure memory safety, after the line let s2 = s1;, Rust considers s1 no longer valid
     */
    println!("{s1}, world!"); // this will fail to compile, as Rust will not allow it to compile
}
