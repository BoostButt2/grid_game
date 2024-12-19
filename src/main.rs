use std::io;

struct Player {
    name: String,
    p: u8,
    points: u8
}

fn create_player(name: String, p: u8) -> Player {
    let player = Player { name, p, points: 0};
    player
}

fn create_grid(rows: usize, cols: usize) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = vec![vec!['#'; rows]; cols]; 
    grid
}

fn make_move(mut grid: Vec<Vec<char>>, player: Player, x: usize, y: usize) -> Vec<Vec<char>> {
    if (is_out_of_bounds(&grid, &x, &y)){
        println!("Invalid move. Try again!");
        return grid;
    }
    match grid[x][y] { //AI helped with this check. This is more readable 
        '#' => {
            match player.p {
                1 => grid[x][y] = 'X',
                2 => grid[x][y] = 'O',
                _ => println!("Please don't"),
            }
        }
        _ => println!("Cell already occupied!"),
    }
    grid
}

fn is_out_of_bounds(grid: &Vec<Vec<char>>, x: &usize, y: &usize) -> bool {
    if(x >= &grid.len() || y >= &grid[0].len()) {
        return true;
    }
    false
}

fn print_grid(grid: &Vec<Vec<char>>){
    for row in grid {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

fn start_tic_tact_toe(grid: Vec<Vec<char>>, player1: Player, player2: Player){
    println!("{} vs {}", player1.name, player2.name);
    
    print_grid(&grid);
}

fn main() {
    println!("Hello, world!");
    let rows: usize = 3;
    let cols: usize = 3;

    let mut grid = create_grid(rows, cols);
    let mut player1 = create_player("Ferrero".to_string(), 1);
    let mut player2 = create_player("Rocher".to_string(), 2);

    start_tic_tact_toe(grid, player1, player2);
}
