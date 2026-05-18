use rand::RngExt;
use std::io;

fn get_random_number() -> u32 {
    let mut rng = rand::rng();
    return rng.random_range(..101);
}

fn main() {
    println!("Welcome to the number guessing game");
    println!("You have only 6 attempts to get guess the number");

    let number_to_be_guessed = get_random_number();
    
    let available_user_choices = 6;

    let mut current_user_choices = 0;
    let mut _is_user_guessed_correct = false;
    

    while current_user_choices != available_user_choices {
        println!("Your chance {}", current_user_choices + 1);

        let mut input = String::new();

        println!("Enter the guess between 0 to 100! ");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read Integer");

        let user_choice: u32 = input.trim().parse().expect("Please Enter a valid Integer");

        if user_choice == number_to_be_guessed {
            _is_user_guessed_correct = true;
            break;
        } else if user_choice > number_to_be_guessed {
            println!("Your choice is higher than the guessed number ");
        } else {
            println!("Your choice is lower than the guessed number ");
        }

        current_user_choices += 1;
    }

    if _is_user_guessed_correct {
        println!("You Guessed the number correctly");
        match current_user_choices + 1 {
            1 => println!("Amazing! First try!"),
            2 | 3 => println!("Great job!"),
            4 | 5 => println!("Good effort!"),
            6 => println!("Just made it!"),
            _ => println!("Well played!"),
        }
    } else {
        println!("You did not guessed the number correctly");
        println!("Actual correct answer {}", number_to_be_guessed);
        println!("Try again!!");
    }
}
