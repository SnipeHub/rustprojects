fn main() 
{
    print_labeled_measurement(5, 'h'); // enter our parameters, a number 5 and a char h
}

fn print_labeled_measurement(value: i32, unit_label: char) // we define the parameter types
{
    println!("The measurement is: {value}{unit_label}"); // this will print to screen: 5h
}
