use std::{io};

struct Player {
    name: String,
    p: P
}

#[derive(Default)]
pub enum P {
    #[default]
    Player1,
    Player2
}

#[derive(Default, PartialEq)]
pub enum CellValue {
    #[default]
    Empty,
    X,
    O
}

#[derive(Default)]
pub struct Game {
    pub grid: [[CellValue; 3]; 3],
    pub current_player: P,
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
                game_active = false;
            }
        }
    }

    pub fn make_move(&mut self, coord: [usize; 2]) {
        let [x, y] = coord;
        if !self.is_valid_move(coord){
            return;
        }
        
        self.grid[x][y] = match self.current_player {
            P::Player1 => {
                self.current_player = P::Player2;
                CellValue::X
            },
            P::Player2 => {
                self.current_player = P::Player1;
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

    pub fn is_valid_move(&self, coord: [usize; 2]) -> bool {
        let empty = self.is_empty(coord);
        let off_board = self.is_out_of_bounds(coord);

        if empty && !off_board{
            return true;
        }
        false
    }

    pub fn is_empty(&self, coord: [usize; 2]) -> bool {
        let [x, y] = coord;
        match self.grid[x][y] {
            CellValue::Empty => true,
            _ => false
        }
    }

    pub fn is_out_of_bounds(&self, coord: [usize; 2]) -> bool {
        let [x, y] = coord;
        if x >= self.grid.len() || y >= self.grid[0].len() {
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

fn create_player(name: String, p: P) -> Player {
    let player = Player { name, p };
    player
}

fn create_grid(rows: usize, cols: usize) -> Vec<Vec<char>> {
    let grid: Vec<Vec<char>> = vec![vec!['#'; rows]; cols];
    grid
}

fn make_move(
    mut grid: Vec<Vec<char>>, player: &Player, x: usize, y: usize) -> (Vec<Vec<char>>, bool) {
    if is_out_of_bounds(&grid, &x, &y) {
        println!("Invalid move. Try again!");
        return (grid, false);
    }
    match grid[x][y] {
        '#' => {
            match player.p {
                P::Player1 => grid[x][y] = 'X',
                P::Player2 => grid[x][y] = 'O',
            }
            (grid.clone(), true)
        }
        _ => {
            println!("Cell already occupied!");
            (grid.clone(), false)
        }
    }
}

fn get_move() -> Vec<usize> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Input couldn't be read");
    let coords_string: Vec<&str> = input.split(',').collect();
    let mut coords: Vec<usize> = Vec::new();

    for c in coords_string {
        coords.push(c.trim().parse::<usize>().unwrap()); //AI: unwrap
    }

    return coords;
}

fn is_out_of_bounds(grid: &Vec<Vec<char>>, x: &usize, y: &usize) -> bool {
    if x >= &grid.len() || y >= &grid[0].len() {
        return true;
    }
    false
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

fn check_win(grid: &Vec<Vec<char>>) -> bool {
    for row in grid {
        if row[0] != '#' && row[0] == row[1] && row[1] == row[2] {
            return true;
        }
    }

    for col in 0..3 {
        if grid[0][col] != '#' && grid[0][col] == grid[1][col] && grid[1][col] == grid[2][col]
        {
            return true;
        }
    }

    if grid[0][0] != '#' && grid[0][0] == grid[1][1] && grid[1][1] == grid[2][2] {
        return true;
    }

    if grid[0][2] != '#' && grid[0][2] == grid[1][1] && grid[1][1] == grid[2][0] {
        return true;
    }

    false
}

fn start_tic_tact_toe(mut grid: Vec<Vec<char>>, player1: Player, player2: Player) {
    println!("{} vs {}", player1.name, player2.name);
    let mut is_active = true;
    let mut current_player = &player1;

    while is_active {
        let mut is_successful = true;
        println!("{}'s turn", current_player.name);
        print_grid(&grid);
        let player_move = get_move();
        (grid, is_successful) = make_move(grid, current_player, player_move[0], player_move[1]);

        if check_win(&grid) {
            println!("{} won the game!", current_player.name);
            print_grid(&grid);
            is_active = false;
        }

        if is_successful {
            match current_player.p {
                P::Player1 => current_player = &player2,
                P::Player2 => current_player = &player1
            }
        }
    }
}

fn main() {
    // println!("Hello, world!");
    // let rows: usize = 3;
    // let cols: usize = 3;

    // let grid = create_grid(rows, cols);
    // let player1 = create_player("Ferrero".to_string(), P::Player1);
    // let player2 = create_player("Rocher".to_string(), P::Player2);

    // start_tic_tact_toe(grid, player1, player2);

    let mut game = Game::new();
    game.run_game();
}
