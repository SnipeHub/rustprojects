fn main() {

    let x = 5; // a value of 5 is assigned
    let x = x + 1; // we now add a 1, x now becomes a 6

    {
        let x = x * 2; // we now take the 6 and multyply by 2, this inner scope is now shadowing the variable x
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}"); // now x becomes 6 again, as the shadow scope above is not affecting anything outside of it, very cool!

    {
        let x = x * 50;
        let x = x + 5;

        println!("This second inner scope is: {x}"); // also keep in mind the variable is still immutable, 
                                                     //if we set it to mut, we will get a runtime error
    }

    println!("The value of x is: {x}");

}
