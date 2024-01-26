// Rust has two primitive compound types: tuples and arrays

    // the tuple type
    fn main()
    {
        let _tup: (i32, f64, u8) = (500, 6.4, 1); // one example, defined

        let tup = (500, 6.4, 1); // another example

        let (x, y, z) = tup; // this example is called destructuring

        println!("The value of y is: {y}, the value of x: {x} and the value of z: {z}");

        /* Example 2 */
        {   // we can also access a tuple element directly by using a period . followed by the index of the value we want to access
            let x: (i32, f64, u8) = (500, 6.4, 10);
            // it has defined each tuple element with an index; 0,1 and 2. 0 is i32, 1 is f64 and u8 is 2
            let five_hundred = x.0;
            let six_point_four = x.1;
            let one = x.2;

            println!("i32: {five_hundred}, f64: {six_point_four}, u8: {one}");
        }
    }
