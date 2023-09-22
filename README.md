# Rust Hangman
> first rust-project. i am confusion. but in a good way.

________________________________________________________________________

**bugs / to be fixed eventually**

- [ ] sometimes has error when already guessed char is put in
- [ ] can not handle uppercase inputs
- [ ] dealing with unwraps 


**idea dump for things i want to / might add**

- levels (verschiedene dictionaries mit unterschiedlich schweren Wörtern)
- buttons to click on
- tips in pvc mode


________________________________________________________________________

## Version History

### 1.1 - 1.3 [^1]

- bare structure, can read in user input, store it in a string & compare with specific single chars
- a lot of println! & Hello Worlds
- added rand from rust standart library for random chars (first pvc attempt)
- added attempts limitation & guesses containing multiple chars (second pvc attempt) with underscore tranformation & thing_on_display
- added pvp
- restructered into functions for better overview between pvc & pvp

### 1.4 

- chars already guessed by user is stored and shown
- a lot of println!(" ") for terminal aesthetics
- hangman ascii arts hell yeahhh
- [linear congruential generator](https://en.wikipedia.org/wiki/Linear_congruential_generator) for "random" number generation

### 1.5 

- even more ascii art yay
- option for multiple rounds
- dictionary extension
- heart system

### 1.6

- code restructure
- game over & drawing hangman animation
- option to switch language bases for pvc mode

[^1]: can`t remember the exact time line anymore.
