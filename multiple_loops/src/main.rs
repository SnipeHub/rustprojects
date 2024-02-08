fn main() // loop labels to disambiguate between multiple loops
{
    /* if you have loops within loops, break and continue apply to the innermost loop at that point
    You can optionally specify a 'loop label', which will apply to the loop you want, rather than the innermost one. */

    let mut count = 0;
    'counting_up: loop // our loop label ''counting _up'. The first 'break' that doesn't specify a label will exit the inner loop only.
    {
        println!("count = {count}");
        let mut remaining = 10;

        // second loop or inner loop
        loop 
        {
            println!("remaining = {remaining}");
            if remaining == 9
            {
                break;
            }    
            if count == 2
            {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
