#![allow(unused)]
#![allow(non_snake_case)]

mod game;
mod constants;
mod engine;
mod display;

use std::io::{stdin, stdout, Read, Write};
use game::{Mode, Game};

use constants::START_MESSAGE;
fn main() {
    println!("{}",START_MESSAGE);
    stdout().flush();

    let mut choice : [u8; 10] = [0; 10];
    stdin().read(&mut choice);

    choice[0] -= 48;

    let opp = match choice[0] {
        1 => {
            Ok(Mode::Offline)
        },
        2 => {
            Ok(Mode::Online)
        },
        3 => {
            Ok(Mode::AI)
        },
        _ => {
            Err("Choose a valid option")
        }
    };

    if (opp.is_err()) {
        println!("Try Again !!");
    }
    let opp = opp.unwrap();

    
    let game = Game::new(opp);
    game.start();


}
