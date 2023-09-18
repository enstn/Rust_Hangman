/*
TO BE FIXED:
-sometimes has error when already guessed char is put in
-can not handle multiple words / special keys
-can not handle uppercase inputs 

TO BE ADDED:
-string where already guessed characters are contained
-ascii hangman animation!
-libraries (momentan werden noch Vektoren benutzt)
-levels (verschiedene libraries mit unterschiedlich schweren Wörtern)
-eigenes random number generator
-option for multiple rounds
*/

use std::io;
use rand::Rng;

fn main() {
  
    for _n in 1..30 {
        println!("");
    }
    println!("Welcome to Hangman Version 1.3!"); 
    println!("Please select a modi: ");
    println!(" ");
    println!("1 = Player Versus Computer");
    println!("2 = Player Versus Player");
    for _n in 1..20 {
        println!("");
    }
    
    //user input read in
    let mut _modi_input = String::new();
    io::stdin().read_line(&mut _modi_input).expect("Failed to read in user input");
    _modi_input = _modi_input.trim().to_string();

    if (_modi_input == "1") {
        for _n in 1..50 {
            println!("");
        }
        println!("You have selected: Player Versus Computer. The game starts now!");
        pvc_mode();
    } else if (_modi_input == "2") {
        for _n in 1..50 {
            println!("");
        }
        println!("You have selected: Player Versus Player. The game starts now!");
        pvp_mode();
    } else {
        println!("Failed to read in user input");
    }

    println!("Thank you for playing Hangman! Until next time :)");
    for _n in 1..23 {
        println!("");
    }
   

    
}

fn pvc_mode() {
    // Game library (to be somehow replaced by incorporating library.txt)
    let library = vec!["Jazz", "Why", "Are", "You", "Gay"];

    // Random number generator (to be replaced by a manual linear congruential generator, because why not)
    let random_number = rand::thread_rng().gen_range(0, library.len());

    // Number-to-guess conversion (to be modified once library.txt is incorporated)
    let word_to_guess = library[random_number].to_lowercase(); 

    game(word_to_guess);

}

fn pvp_mode() {

    println!("Player One, please input the word to be guessed: ");
    for _n in 1..23 {
        println!("");
    }
    let mut _pvp_word = String::new();
    io::stdin().read_line(&mut _pvp_word).expect("Failed to read in your word");
    _pvp_word.pop();

    for _n in 1..30 {
        println!("");
    }
    println!("Player 2 shall start now!");
    for _n in 1..20 {
        println!("");
    }

    game(_pvp_word.to_lowercase());

}
 
fn game(word_to_guess:String) {

    let mut thing_on_display = vec!['_'; word_to_guess.len()];

    let mut attempts = 5;
    //let mut player_guessed_chars: Vec<char> = Vec::new();

    while attempts > 0 {
        println!("_______________________________________________________");
        println!("You have {} remaining attempts.", attempts);
        println!("{}", thing_on_display.iter().collect::<String>()); //iterator for underscore placement
        println!(" ");
        println!("Please input your guess:");
        for _n in 1..19 {
            println!("");
        }

        let mut guess_input = String::new();
        io::stdin().read_line(&mut guess_input).expect("Failed to read in guess input");
        let guess = guess_input.trim().chars().next().unwrap(); //thank you Stackoverflow; checks for single char input

        if word_to_guess.contains(guess) {
            for _n in 1..20 {
                println!("");
            }
            for (i, c) in word_to_guess.chars().enumerate() { //Stackoverflow at it again
                if c == guess {
                    thing_on_display[i] = c;
                }
            }
            if (!thing_on_display.contains(&'_')) { 
                for _n in 1..20 {
                    println!("");
                }
                println!("Correct, the word was {}!", word_to_guess);
                break;
            }
        } else {
            for _n in 1..20 {
                println!("");
            }
            println!("False! Try again");
            attempts -= 1;
        }
    }

    if attempts == 0 {
        println!("Game over :( The word was: {}", word_to_guess);
        for _n in 1..20 {
            println!("");
        }
    }

}