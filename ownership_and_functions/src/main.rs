/* The mechanics of passing a value to a function are similar to those 
when assigning a value to a variable. Passing a variable to a function will move or copy,
just as assinment does. */

fn main() 
{
   let s = String::from("our first string"); // s comes into scope

   takes_ownership(s);                 // s value moves into the function
                                       // and so is no longer valid here

    let x = 5;                         // x come sinto scope

    makes_copy(x);                     // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} /* Here, x goes out of scope, then s. But because s value was moved, nothing
  special happens. */

fn takes_ownership(some_string: String) /* some_string comes into scope, this function now has ownership
If I wanted to change the string to something else, I can, and also still call the original */
{
    let s = "re-using s again"; /* this is a new variable, as the original has gone out of scope,
    due to the complexity of strings, the `String` variable is passed to the function, ownership is transferred,
    so now I can keep using it. */
    println!("Our string from the function: {some_string}");
    println!("{s}");
    let f = some_string;
    println!("our f variable using `{f}`");

    let some_string = String::from("Our first string changed");
    println!("{some_string}");

} /* Here, some_string goes out of scope and `drop` is called. The backing
    memory is freed. */

fn makes_copy(some_integer: i32) // some_integer comes into scope
{
    let x = some_integer * 2; /* re-using and modying the x variable, this is my own personal example to show that
    x = 5 is being used and modified in this function */
    println!("here is the integer: {some_integer}, and now variable x (5) multiplyed by 2 = {x}");
} // Here, some_integer goes out of scope. Nothing special happens.
