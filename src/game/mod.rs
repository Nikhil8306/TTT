use std::io::{Read, Write};

use super::engine::offline::OfflineEngine;

pub enum Mode {
    Offline,
    Online,
    AI
}

pub struct Game {
    opp: Mode,
    ready: bool,
    finished: bool,
}

impl Game {
    pub fn new(opp: Mode) -> Self {
        Self { 
            opp,
            ready:false,
            finished:false
        }
    }
}

impl Game {
    pub fn start(&self) -> Result<(), String> {

        
        match self.opp {
            // Mode::Online => {

            // },
            Mode::Offline => {
                Self::offlineGame();
            },
            // Mode::AI => {

            // },
            _ => {
                return Err(String::from("Not implemented"));
            }
        }


        Ok(())
    }

    fn offlineGame() {
        let mut engine = OfflineEngine::new();
        let [ref p1, ref p2] = engine.players;

        engine.onGameOver(|won| {println!("Player {} won", won)});

        while !engine.finished {
            std::io::stdout().flush();

            println!("Enter your turn : ");
            let mut input : [u8; 10] = [0; 10];
            std::io::stdin().read(&mut input);
            println!("Input is : {}", input[0]);
            input[0] -= 48;
            
            let rcRes = Self::getRowCol(input[0]);

            if rcRes.is_err() {
                println!("{}", rcRes.unwrap_err());
                continue;
            }

            let (row, col) = rcRes.unwrap();

            engine.mark(row, col);
            
            println!("Current board: {:?}", engine.board);
        }
    }



    fn getRowCol(inp: u8) -> Result<(u8, u8), String> {
        let row = match inp {
            7|8|9 => {
                0
            },
            4|5|6 => {
                1
            },
            1|2|3 => {
                2
            },
            _ => {
                return Err(String::from("Enter a valid input"));
            }
        };
        let col = match inp {
            1|4|7 => {
                0
            },
            2|5|8 => {
                1
            },
            3|6|9 => {
                2
            },
            _ => {
                return Err(String::from("Enter a valid input"));
            }
        };
        
        return Ok((row, col));
    }
}


