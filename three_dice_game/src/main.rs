use std::io;
use rand::Rng;

fn main() 
{
    let mut total_money = 1000; // Initial total start money, change this to whatever you like
    let mut rng_roll = rand::thread_rng(); // Initialize random number generator

    loop 
    {
        println!("Total Money: {total_money}. Welcome to 3 Dice! Please place your bet!"); // where the player chooses their options
        println!("1. Play Dice (500 bet)");
        println!("2. Play Dice (1000 bet)");
        println!("3. Play Dice (5000 bet)");
        println!("4. Play Dice (10000 bet)");
        println!("5. Quit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let choice: u32 = match input.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice
        {
            1..=4 => 
            {
            let bet = match choice {
            1 => 500,
            2 => 1000,
            3 => 5000,
            4 => 10000,
            _ => 0,
            };

            if total_money < bet 
            {
                println!("Insufficient funds to place the bet.");
            }

            println!("Select your dice number (1-6):");
            let mut selected_number = String::new();
            io::stdin().read_line(&mut selected_number).expect("Failed to read line");
            let selected_number: u32 = match selected_number.trim().parse()
            {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input! Please enter a number between1 and 6.");
                    continue;
                }
            };

            let mut roll_results = Vec::new();
            for _ in 0..3 { // This is determining how many 6 sided dice you are rolling each time, should be 3
                let roll = rng_roll.gen_range(1..=6); // Roll a die (6 sided die or 1-6)
                roll_results.push(roll);
            }
            println!("Dice roll results: {:?}", roll_results);

            let mut matching_count = 0;
            for roll in &roll_results {
                if *roll == selected_number {
                    matching_count += 1;
                }
            }

            let winnings = match matching_count { // This will output our odds, if you match one die, you get what you bet back and increases each time you match up to 3 die
                1 => bet * 2,
                2 => bet * 3,
                3 => bet * 4,
                _ => 0,
            };
            total_money += winnings - bet; // your winnings are calculated and stored in 'total_money'
        }

        5 => {
            println!("Thanks for playing!");
            break;
        }
        _ => continue,
    }

    if total_money <= 0 {
        println!("You have run out of money! Game over.");
        total_money = 1000; // Reset total money
    }
  }
}