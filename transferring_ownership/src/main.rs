/* Here is an example of transferring a value in a function using a tuple */

fn main() 
{
    let s1 = String::from("first string");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) // this is counting how many characters in our string
{
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
/* But this is too much work for a concept that should be common. Rust has a better way
in the next section. There is a feature for using a value without transferring ownership,
called references. */