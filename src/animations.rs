//use std::collections::btree_map::IterMut;
use std::fs::File;
use std::io::{BufReader, BufRead, Error};
//use std::{io, vec};
use std::thread::sleep;
use std::time::Duration;
//use std::time::{SystemTime, UNIX_EPOCH};




// animations
pub fn heart_animation(a: &u8, c: &String, t: &Vec<char>) {
    
    heart_display_state(&a,&c,&t);   
    sleep(Duration::from_millis(300));
    clear_display();
    current_display_state(&a,&c,&t);
    sleep(Duration::from_millis(300));
    clear_display();

    heart_display_state(&a,&c,&t);   
    sleep(Duration::from_millis(300));
    clear_display();
    current_display_state(&a,&c,&t);
    clear_display();
}

pub fn branch_animation(branch: u8, a: &u8, c: &String, t: &Vec<char>) {

    match branch {

        1 => {
            clear_display();
            branch_display_state(10, &a, &c, &t);
            sleep(Duration::from_millis(100));

            clear_display();
            branch_display_state(11, &a, &c, &t);
            sleep(Duration::from_millis(100));

            clear_display();
            branch_display_state(12, &a, &c, &t);
            sleep(Duration::from_millis(100));

            clear_display();
            branch_display_state(13, &a, &c, &t);
            sleep(Duration::from_millis(100));

            clear_display();
            branch_display_state(14, &a, &c, &t);

            clear_display();
        },

        2 => {
            clear_display();
            branch_display_state(21, &a, &c, &t);
            sleep(Duration::from_millis(100));

            clear_display();
            branch_display_state(22, &a, &c, &t);
            sleep(Duration::from_millis(100));

            clear_display();
            branch_display_state(23, &a, &c, &t);
            
            clear_display();

        },

        _ => println!("branch animation failed"),
        
    }
}

// easy prints
pub fn clear_display() {
    for _ in 0..23 {
        println!(" ");
    }
}

pub fn clear_for(n: u8) {
    for _ in 0..n {
        println!(" ");
    }
}

pub fn print_heart() {

    let path = "res/heart.txt".to_string();
    let file = File::open(path).expect("failed to open words file");
    let reader = BufReader::new(file);
    let argument = |r: Result<String, Error>| {
        match r {
            Ok(line) => line,
            Err(..) => { "".to_string() },
        }
    };
    let heart_logo: Vec<String> = reader.lines().map(argument).collect();
    for line in heart_logo.iter() {
        println!("{line}");
    }
    println!(" ");
}

pub fn print_over() {

    let path = "res/over.txt".to_string();
    let file = File::open(path).expect("failed to open words file");
    let reader = BufReader::new(file);
    let argument = |r: Result<String, Error>| {
        match r {
            Ok(line) => line,
            Err(..) => { "".to_string() },
        }
    };
    let over_logo: Vec<String> = reader.lines().map(argument).collect();
    for line in over_logo.iter() {
        println!("{line}");
        sleep(Duration::from_millis(50));
    }
    println!(" ");
}

pub fn print_logo() {

    let path = "res/logo.txt".to_string();
    let file = File::open(path).expect("failed to open words file");
    let reader = BufReader::new(file);
    let argument = |r: Result<String, Error>| {
        match r {
            Ok(line) => line,
            Err(..) => { "".to_string() },
        }
    };
    let start_logo: Vec<String> = reader.lines().map(argument).collect();
    for line in start_logo.iter() {
        println!("{line}");
        sleep(Duration::from_millis(50));
    }
    println!(" ");
}

pub fn ascii_arts(n: u8) {

    println!(" ");
    match n {

        // hangman 
        0 => print!("       +---+\n       |   |\n       O   |\n      /|L  |\n       /L  |\n           |\n     =========\n"),
        1 => print!("       +---+\n       |   |\n       O   |\n      /|L  |\n       /   |\n           |\n     =========\n"),
        2 => print!("       +---+\n       |   |\n       O   |\n      /|L  |\n           |\n           |\n     =========\n"),
        3 => print!("       +---+\n       |   |\n       O   |\n      /|   |\n           |\n           |\n     =========\n"),
        4 => print!("       +---+\n       |   |\n       O   |\n       |   |\n           |\n           |\n     =========\n"),
        5 => print!("       +---+\n       |   |\n       O   |\n           |\n           |\n           |\n     =========\n"),
        6 => print!("       +---+\n       |   |\n           |\n           |\n           |\n           |\n     =========\n"),
        7 => print!("       +---+\n           |\n           |\n           |\n           |\n           |\n     =========\n"),
        8 => print!("           +\n           |\n           |\n           |\n           |\n           |\n     =========\n"),
        9 => print!("            \n            \n            \n            \n            \n            \n     =========\n"),

        // first 
        10 => print!("            \n            \n            \n            \n            \n           |\n     =========\n"),
        11 => print!("            \n            \n            \n            \n           |\n           |\n     =========\n"),
        12 => print!("            \n            \n            \n           |\n           |\n           |\n     =========\n"),
        13 => print!("            \n            \n           |\n           |\n           |\n           |\n     =========\n"),
        14 => print!("            \n           |\n           |\n           |\n           |\n           |\n     =========\n"),

        // second
        21 => print!("          -+\n           |\n           |\n           |\n           |\n           |\n     =========\n"),
        22 => print!("         --+\n           |\n           |\n           |\n           |\n           |\n     =========\n"),
        23 => print!("        ---+\n           |\n           |\n           |\n           |\n           |\n     =========\n"),

        _ => println!("ascii animation not found"),
    }
}


// display states
pub fn current_display_state(attempts: &u8, player_guessed_chars: &String, thing_on_display: &Vec<char> ) {

    let a = attempts;
    let c = player_guessed_chars;
    let t = thing_on_display;

    clear_for(4);
    println!("{attempts} remaining attempts");
    print!(" ");
    for _ in 0..*a {
        print!("♥︎ ");
    }
    println!(" ");
    ascii_arts(*a);
    println!(" ");
    println!("  {} [ {c}]", t.iter().collect::<String>()); //iterator for underscore conversion
    clear_for(6);
    println!("Please input your guess:");

}

pub fn heart_display_state(attempts: &u8, player_guessed_chars: &String, thing_on_display: &Vec<char> ) {

    let a = attempts;
    let c = player_guessed_chars;
    let t = thing_on_display;

    clear_for(4);
    println!("{attempts} remaining attempts");
    print!(" ");
    for _ in 0..*a - 1 {
        print!("♥︎ ");
    }
    print!("██");
    println!(" ");
    ascii_arts(*a);
    println!(" ");
    println!("  {} [ {c}]", t.iter().collect::<String>()); //iterator for underscore conversion
    clear_for(6);
    println!("Please input your guess:");

}

pub fn branch_display_state(branch: u8, attempts: &u8, player_guessed_chars: &String, thing_on_display: &Vec<char> ) {

    let a = attempts;
    let c = player_guessed_chars;
    let t = thing_on_display;

    clear_for(4);
    println!("{attempts} remaining attempts");
    print!(" ");
    for _ in 0..*a {
        print!("♥︎ ");
    }
    println!(" ");
    ascii_arts(branch);
    println!(" ");
    println!("  {} [ {c}]", t.iter().collect::<String>()); //iterator for underscore conversion
    clear_for(6);
    println!("Please input your guess:");

}



