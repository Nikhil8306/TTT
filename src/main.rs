#![allow(unused)]
#![allow(non_snake_case)]

mod constants;
mod game;


use std::io::{stdin, stdout, Read, Write};
use game::{Turn};

fn main() {
    println!("Welcome to Tic Tac Toe");
    println!("Choose your game mode : ");

    let choices = constants::GAMECHOICE;

    for (ind, choice) in choices.iter().enumerate() {
        println!("{}.  {}", ind+1, choice);
    }

    print!(">>> ");
    stdout().flush();

    let mut choice : [u8; 1] = [0];
    stdin().read(&mut choice);


    let players = match choice[0] {
        1 => {  
            Ok([Turn::Offline, Turn::Offline])
        },
        2 => {
            Ok([Turn::Offline, Turn::Online])
        },
        3 => {
            Ok([Turn::Offline, Turn::AI])
        },
        _ => {
            Err("Choose from the provided options")
        }
    };

    



}
