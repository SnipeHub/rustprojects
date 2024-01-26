use std::io;

fn main() 
{
    let a= [1, 2, 3, 4, 5]; // Array's are good when you have a fixed number of elements, otherwise a vector type is more flexible.
                                       // A vector is a smilar collection type provided by the standard library that is allowed to grow or shrink.

    /* When doing arrays you type using square brackets. The below example works well for an array as you have fixed elements */

    let months = ["January", "February", "March", "April", "May", "June", "July", "August",
                            "Spetember", "October", "November", "December"];

    /* another example */
    //let a: [i32; 5] = [1, 2, 3, 4, 5]; // This indicates an i32 type and 5 in the array
    /* or */
    let a = [3; 5]; /* The array named 'a' will contain 5 elements that will be set to the value of 3 initially.
    This is the same as writing let a = [3, 3, 3, 3, 3]; */

    /* Accessing array elements, you can access an array using indexing */
    {
        let a = [1, 2, 3, 4, 5];

        let first = a[0];
        let second = a[1];
    }

    /* Invalid array element access */
   
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

