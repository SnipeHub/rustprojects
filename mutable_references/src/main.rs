// mutable references

fn main()
{
    let mut s = String::from("First String");

    change(&mut s); //here we change s to be mutable, which means we are allowing it to change
}

fn change(some_string: &mut String)
{
    some_string.push_str(",world");
}
/* Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other
references to that value. 
In rust, this prevents data races where by two or more pointers access the same data at the same time.*/
