fn main() {
    let mut x = 5; // By default, variables are immutable, meaning they can not change
    println!("The value of x is: {x}");
    x = 6; //To allow this variable to change, we must add "mut" above, this is short for mutable
    println!("The value of x is: {x}");
}
