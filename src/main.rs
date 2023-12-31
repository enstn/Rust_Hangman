
//use std::collections::btree_map::IterMut;
use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use std::{io, vec};
use std::thread::sleep;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

mod animations;

// linear congruential generator for "random" number used in pvc mode
fn lcg(n:u64) -> u64 {       

    // lcg parameters (from Wikipedia sourced by Numerical Recipes, seems to work fine)
    let m:u64 = u64::pow(2, 32); // modulus
    let a:u64 = 1664525; // multiplier
    let c:u64 = 1013904223; // increment

    // seed generated by current time 
    let time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time not found"); 
    let s: u64 = time.as_secs() as u64; // seed

    // somehow this crap works
    let mut current = s;
    for _ in 0..19 {
        current = (a * current + c) % m;
        //let result = current % n;         // used for debug to check for range & probability
        //println!("{}", result);

    }
    let result = current % n;

    result
}

fn select_language() -> String {

    // preperations
    animations::clear_display();
    println!("You have selected: Player Versus Computer.");
    animations::clear_display();
    sleep(Duration::from_millis(1000));
    println!("You have selected: Player Versus Computer.");
    println!("remember: please only input lowercase chars");
    animations::clear_for(22);
    sleep(Duration::from_millis(1000));

    println!("Please choose what language your word should be from");
    println!(" ");
    println!("1 = English");
    println!("2 = German");
    animations::clear_for(10);

    //user input read in
    loop {
        let mut language_input = String::new();
        io::stdin().read_line(&mut language_input).expect("failed to read in user input");
        
        language_input = language_input.trim().to_string();


        if language_input == "1" {
            let result = "res/english.txt".to_string();
            println!("You have selected: ENGLISH");
            animations::clear_for(23);
            sleep(Duration::from_millis(1400));
            return result;
        } else if language_input == "2" {
            let result = "res/german.txt".to_string();
            println!("You have selected: GERMAN");
            animations::clear_for(23);
            sleep(Duration::from_millis(1400));
            return result;
        } else {
            println!("Failed to read in user input");
        }
    }
    
}

// player versus computer -> provides one word "randomly" from dictionary to game()
fn pvc_mode(sl: String) {

    // Gets game dictionary into a vector
    let path = sl;
    let file = File::open(path).expect("failed to open words file");
    let reader = BufReader::new(file);
    let argument = |r: Result<String, Error>| {
        match r {
            Ok(line) => line,
            Err(..) => { "".to_string() },
        }
    };

    // Game dictionary (to be somehow replaced by incorporating dictionary.txt)
    let dictionary: Vec<String> = reader.lines().map(argument).collect();
    
    // Random number provided by linear congruential generator
    let size = dictionary.len();
    let random_number = lcg(size as u64);
    //print!("THE RANDOM NUMBER IS: {} ", random_number); // for debug purposes

    // Number-to-guess conversion (to be modified once dictionary.txt is incorporated)
    let word_to_guess = dictionary[random_number as usize].to_lowercase(); 

    // game function
    game(word_to_guess);
    
}

// player versus player - provides one word by user input to game()
fn pvp_mode() {

    // preperations
    animations::clear_display();
    println!("You have selected: Player Versus Player. The game starts now!");
    animations::clear_display();
    sleep(Duration::from_millis(1000));
    println!("You have selected: Player Versus Player. The game starts now!");
    println!("remember: please only input lowercase chars");
    animations::clear_for(22);
    sleep(Duration::from_millis(2000));
    animations::clear_display();
    
    // aesthethics
    println!("PLAYER ONE, PLEASE INPUT THE WORD TO BE GUESSED ");
    animations::clear_for(19);

    // user input
    let mut _pvp_word = String::new();
    io::stdin().read_line(&mut _pvp_word).expect("Failed to read in your word");
    _pvp_word.pop();

    // check whether word has only chars or not
    let mut _pvp_word_checked = _pvp_word.clone();
    _pvp_word_checked.retain(|c| c.is_alphabetic());

    // aesthethics
    animations::clear_for(29);
    println!("Player 2 shall start now!");
    animations::clear_for(19);
    
    // game function
    game(_pvp_word_checked.to_lowercase());
    
}

// has hangman rules implemented and checks / updates the scores
fn game(word_to_guess:String) {
    
    let mut thing_on_display = vec!['_'; word_to_guess.len()]; 
    let mut attempts = 9;
    let mut player_guessed_chars = String::new(); 
    
    while attempts > 0 {

        animations::clear_for(4);
        println!("{attempts} remaining attempts");
        print!(" ");
        for _ in 0..attempts {
            print!("♥︎ ");
        }
        println!(" ");
        animations::ascii_arts(attempts);
        println!(" ");
        println!("  {} [ {player_guessed_chars}]", thing_on_display.iter().collect::<String>()); //iterator for underscore conversion
        animations::clear_for(6);
        println!("Please input your guess:");

        // reads in user guess
        let mut guess_input = String::new();
        io::stdin().read_line(&mut guess_input).expect("Failed to read in guess input");
        let guess = guess_input.trim().chars().next().unwrap(); // checks and trims for single char input
        
        /*
        let mut guess_input = String::new();
        let mut guess;
        loop {
            io::stdin().read_line(&mut guess_input).expect("Failed to read in guess input");

            guess_input.retain(|c| c.is_alphabetic());
            guess = guess_input.trim().chars().next().unwrap(); // checks and trims for single char input
            if guess.is_alphabetic() {
                break;
            } else {
                println!("Please input a valid char");
            }
        }
        */

        // checks player input according to hangman rules
        if word_to_guess.contains(guess) {
            animations::clear_for(19);
            for (i, c) in word_to_guess.chars().enumerate() { // replaces the underscores with guessed char
                if c == guess {
                    thing_on_display[i] = c;
                }
            }
            if !thing_on_display.contains(&'_') { // checks whether word has been found or not
                animations::clear_for(19);
                println!("Correct, the word was [ {word_to_guess} ]!");
                break;
            }
        } else {
            player_guessed_chars.push(guess);
            player_guessed_chars.push(' ');
            animations::clear_for(19);
            animations::heart_animation(&attempts, &player_guessed_chars, &thing_on_display);
            if attempts == 9 {
                animations::branch_animation(1, &attempts, &player_guessed_chars, &thing_on_display);
            }
            if attempts == 8 {
                animations::branch_animation(2, &attempts, &player_guessed_chars, &thing_on_display);
            }
            
            attempts -= 1;
            //if attempts > 0 {println!("False! Try again");}
        }
    }
    
    if attempts == 0 {
        animations::print_over();
        println!("The word was: [ {word_to_guess} ]");
        println!(" ")
    }

}


fn main() {

    let version: f32 = 1.5; 

    animations::clear_display();
    println!("Welcome to version {version}!");
    animations::print_logo();
    println!(" ");
    println!("Please select a modi: ");
    println!(" ");
    println!("1 = Player Versus Computer");
    println!("2 = Player Versus Player");
    for _ in 0..10 {
        println!(" ");
        sleep(Duration::from_millis(50));
    }
    
    //user input read in
    let mut modi_input = String::new();
    io::stdin().read_line(&mut modi_input).expect("failed to read in user input");
    modi_input = modi_input.trim().to_string();

    if modi_input == "1" {
        pvc_mode(select_language());
    } else if modi_input == "2" {
        pvp_mode();
    } else {
        println!("Failed to read in user input");
    }

    println!("Round ended, do you want to start a new round? [ y / n ]");
    let mut round_input = String::new();
    io::stdin().read_line(&mut round_input).expect("failed to read in user input");
    let char = round_input.trim().chars().next().unwrap();

    match char {

        'y' => main(),
        'n' => {
                animations::clear_display();
                animations::print_heart();
                println!("Thank you for testing Hangman Version {version}! Until next time :)");
                println!(" ");
            },
        _ => println!("unvalid input"),

    }
    
}



 /* 
    let app = App::new();

    for _ in 0..29 {
        println!(" ");
    }

    pub struct App {
        hangman_logo: Vec<String>,
    }
    
    impl App { 
        pub fn new() -> Self {
            let path = "res/logo.txt".to_string();
            let file = File::open(path).expect("failed to open logo file");
            let reader = BufReader::new(file);
            let my_closure = |r: Result<String, Error>| {
                match r {
                    Ok(line) => line,
                    Err(err) =>  {
                        println!("We got an error: {err}");
                        "".to_string()
                    },
                }
            };
            let hangman_logo: Vec<String> = reader.lines().map(my_closure).collect();
            Self { hangman_logo }
        } 
    
    }
*/