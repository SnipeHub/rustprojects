fn main() /* Variables and Data interacting with clone. As previously with interacting with move, 
with clone we can deep copy the string data from the heap, and not just the stack data */
{
    let s1 = String::from("clone method");
    let s2 = s1.clone(); // this will clone s1 into s2 variable

    println!("s1 = {s1}, s2 = {s2}");
    /* When you see a call to clone, like above, you know that some arbitrary code is being executed
    and that code may be expensive. */

    copy();
}

/* Stack-Only Data: Copy */
fn copy() 
{
    let x = 5;
    let y = x; // here we have two variables, both equal 5, yet they both point to the same data

    println!("Copy example: x = {x}, y = {y}"); /* But this code seems to contradict what we just learned
    we don't have to call to 'clone', but x is still valid and wasn't moved into y */

/* The reason is that types such as intergers that have a known size at compile time are stored entirely
on the stack, so copies of the actual values are quick to make. In other words, there's no difference between
deep and shallow copying here, so calling 'clone' wouldn't do anything different from the usual shallow copying,
and we can leave it out. */
}
/* Here are some of the types that implement Copy:

    All the integer types, such as u32.
    The Boolean type, bool, with values true and false.
    All the floating-point types, such as f64.
    The character type, char.
    Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.
 */