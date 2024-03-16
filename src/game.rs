use crate::utils::error::print_error;
use crate::utils::input::input;
use std::thread::sleep;
use std::time::Duration;
use termion::{ clear, cursor };

pub struct Game {
    turn: char,
    grid: Vec<Vec<String>>
}

impl Game {

    pub fn new() -> Game {

        Game {
            turn: 'X',
            grid: vec![vec![" ".to_string(); 3]; 3]
        }

    }

    pub fn start(&mut self) {

        loop {
            print!("{}{}", clear::All, cursor::Goto(1, 1));

            println!("========== Tic Tac Toe ==========");
            println!("");

            self.print_board();

            println!("");

            if let Some(value) = self.check_winner() {
                println!("{} is the winner!", value);
                break;
            }

            if self.check_draw() {
                println!("You drew!");
                break;
            }

            println!("Turn: {}", self.turn);
            println!("");
            println!("Enter row and col (11) or 'quit' to end the game");

            let command: String = input(">> ");

            if command == "quit" {
                println!("Game ended!");
                break;
            }

            let (row, col): (usize, usize) = match self.parse_coords(command) {
                Ok(value) => value,
                Err(error) => {
                    print_error(error);
                    sleep(Duration::from_millis(500));
                    continue;
                }
            };

            self.grid[row - 1][col - 1] = self.turn.to_string();

            if self.turn == 'X' {
                self.turn = 'O';
            }
            else {
                self.turn = 'X';
            }
        }

    }
    
    fn print_board(&self) -> () {
        
        println!("   |  1   2   3  |");
        println!("---+-------------+");
        println!(" 1 |  {}  |", self.grid[0].join(" | "));
        println!("   | ---+---+--- |");
        println!(" 2 |  {}  |", self.grid[1].join(" | "));
        println!("   | ---+---+--- |");
        println!(" 3 |  {}  |", self.grid[2].join(" | "));
        println!("---+-------------+");

    }

    fn parse_coords(&self, coords_string: String) -> Result<(usize, usize), &'static str> {

        if coords_string.len() != 2 {
            return Err("Invalid coordinates");
        }

        let row: usize = coords_string[0..1].parse().map_err(|_| "Invalid row")?;
        let col: usize = coords_string[1..2].parse().map_err(|_| "Invalid col")?;

        if row < 1 || row > 3 || col < 1 || col > 3 {
            return Err("Invalid coordinates");
        }

        if self.grid[row - 1][col - 1] != " " {
            return Err("Cell already selected");
        }

        Ok((row, col))

    }

    fn check_draw(&self) -> bool {

        for row in self.grid.iter() {
            for col in row.iter() {
                if col == " " {
                    return false;
                }
            }
        }

        true

    }

    fn check_winner(&self) -> Option<char> {

        for row in self.grid.iter() {
            if row[0] == " " || row[0] != row[1] || row[0] != row[2] {
                continue;
            }
            return Some(row[0].chars().next()?);
        }

        for col in 0..self.grid.len() {
            if self.grid[0][col] == " " || self.grid[0][col] != self.grid[1][col] || self.grid[0][col] != self.grid[2][col] {
                continue;
            }
            return Some(self.grid[0][col].chars().next()?);
        }

        if self.grid[0][0] != " " && self.grid[0][0] == self.grid[1][1] && self.grid[0][0] == self.grid[2][2] {
            return Some(self.grid[0][0].chars().next()?);
        }
        
        if self.grid[0][2] != " " && self.grid[0][2] == self.grid[1][1] && self.grid[0][2] == self.grid[2][0] {
            return Some(self.grid[0][2].chars().next()?);
        }

        None

    }

}
