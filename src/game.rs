use std::io;
use crate::player::Player;
use crate::cell_value::CellValue;

#[derive(Default)]
pub struct Game {
    pub grid: [[CellValue; 3]; 3],
    pub current_player: Player,
}

impl Game {
    pub fn new() -> Game {
        Default::default()
    }

    pub fn run_game(&mut self){
        self.print_grid();
        let mut game_active = true;

        while game_active {
            self.make_move(self.get_move());
            if self.check_win() {
                print!("{:?} won the game!", self.current_player);
                game_active = false;
            }
        }
    }

    pub fn make_move(&mut self, coord: [usize; 2]) {
        let [x, y] = coord;
        if self.is_out_of_bounds(coord) || !self.is_empty(coord){ //added check on valid moves
            return;
        }
        
        self.grid[x][y] = match self.current_player {
            Player::Player1 => {
                self.current_player = Player::Player2;
                CellValue::X
            },
            Player::Player2 => {
                self.current_player = Player::Player1;
                CellValue::O
            }
        };
        self.print_grid();
    }

    pub fn get_move(&self) -> [usize; 2] { //could be improved with better error handling and input validation
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Input couldn't be read");
        let coords_string: Vec<&str> = user_input.split(',').collect();

        let mut coords = [0; 2]; // Fixed-size array
        for (i, c) in coords_string.iter().enumerate() {
            coords[i] = c.trim().parse::<usize>().expect("Invalid number");
        }
    
        coords
    }

    pub fn is_empty(&self, coord: [usize; 2]) -> bool {
        let [x, y] = coord;
        match self.grid[x][y] {
            CellValue::Empty => true,
            _ => {
                println!("Cell is occupied");
                false
            }
        }
    }

    pub fn is_out_of_bounds(&self, coord: [usize; 2]) -> bool {
        let [x, y] = coord;
        if x >= self.grid.len() || y >= self.grid[0].len() {
            println!("Invalid coordinates");
            return true;
        }
        false
    }

    pub fn print_grid(&self) { //printing now works with the new grid
        for row in &self.grid {
            for cell in row {
                match cell {
                    CellValue::O => print!("O"),
                    CellValue::X => print!("X"),
                    CellValue::Empty => print!("#"),
                }
            }
            println!();
        }
    }

    pub fn check_win(&self) -> bool {
        let empty = CellValue::Empty;
        let grid = &self.grid;
        for row in grid {
            if row[0] != empty && row[0] == row[1] && row[1] == row[2] {
                return true;
            }
        }
    
        for col in 0..3 {
            if grid[0][col] != empty && grid[0][col] == grid[1][col] && grid[1][col] == grid[2][col]
            {
                return true;
            }
        }
    
        if grid[0][0] != empty && grid[0][0] == grid[1][1] && grid[1][1] == grid[2][2] {
            return true;
        }
    
        if grid[0][2] != empty && grid[0][2] == grid[1][1] && grid[1][1] == grid[2][0] {
            return true;
        }
    
        false
    }
}