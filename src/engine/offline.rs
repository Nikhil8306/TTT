use rand::Rng;
use super::Player;

pub struct OfflineEngine {
    pub board: [u8; 9],
    pub turn: usize,
    pub players: [Player; 2],
    pub finished: bool,
    pub won: u8,
    pub count: u8,
    onGameOver: fn(u8)
}

impl OfflineEngine {
    pub fn new() -> Self {
        let turn= rand::thread_rng().gen_range(0..=1);
        let p1 = Player {
            mark: 1
        };
        let p2 = Player {
            mark: 2
        };

        let (play1, play2) = match turn {
            0 => {
                (p1, p2)
            }
            _ => {
                (p2, p1)
            }
        };

        return Self {
            board: [0; 9], // 1 = X and 2 = 0
            turn,
            finished: false,
            won: 0,
            players: [play1, play2],
            onGameOver: |_| {},
            count : 0
        };
    }
}

impl OfflineEngine {
    pub fn mark(&mut self, row: u8, col: u8) -> Result<(), String> {
        let ind = Self::getInd(row, col);
        if self.board[ind] != 0 {
            return Err(String::from("Choose empty block"));
        }

        self.board[ind] = self.players[self.turn].mark;
        self.count += 1;

        if let Some(mark) = self.haveWon() {
            self.finished = true;
            self.won = mark;
            (self.onGameOver)(self.won);
        }
        
        if self.count >= 9 {
            self.finished = true;
            self.won = 0;
            (self.onGameOver)(self.won);
        }

        self.turn = (self.turn+1) % 2;

        Ok(())

    }   

    fn haveWon(&self) -> Option<u8>{
        let winning_combinations = [
            // Rows
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            // Columns
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            // Diagonals
            [0, 4, 8],
            [2, 4, 6],
        ];

        for &combo in &winning_combinations {
            let [a, b , c] = combo;

            if self.board[a] != 0 && self.board[a] == self.board[b] && self.board[a] == self.board[c] {
                return Some(self.board[a])
            }
        }


        None
    }

    pub fn onGameOver(&mut self, func: fn(u8)) {
        self.onGameOver = func;
    }

    fn getInd(row: u8, col: u8) -> usize {
        let ind = (row*3) + col;

        return ind as usize;
    }
    
}