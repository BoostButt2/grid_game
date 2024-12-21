use std::io;

struct Player {
    name: String,
    p: u8,
    points: u8,
}

fn create_player(name: String, p: u8) -> Player {
    let player = Player { name, p, points: 0 };
    player
}

fn create_grid(rows: usize, cols: usize) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = vec![vec!['#'; rows]; cols];
    grid
}

fn make_move(
    mut grid: Vec<Vec<char>>,
    player: &Player,
    x: usize,
    y: usize,
) -> (Vec<Vec<char>>, bool) {
    if is_out_of_bounds(&grid, &x, &y) {
        println!("Invalid move. Try again!");
        return (grid, false);
    }
    match grid[x][y] {
        '#' => {
            match player.p {
                1 => grid[x][y] = 'X',
                2 => grid[x][y] = 'O',
                _ => {
                    println!("Invalid player!");
                    return (grid.clone(), false);
                }
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
    println!("coords_string: {} {}", coords_string[0], coords_string[1]);
    let mut coords: Vec<usize> = Vec::new();

    for c in coords_string {
        coords.push(c.trim().parse::<usize>().unwrap()); //AI: unwrap
    }

    return coords;
}

fn is_out_of_bounds(grid: &Vec<Vec<char>>, x: &usize, y: &usize) -> bool {
    if (x >= &grid.len() || y >= &grid[0].len()) {
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

        if is_successful {
            match current_player.p {
                1 => current_player = &player2,
                2 => current_player = &player1,
                _ => println!("Player doesn't exist"),
            }
        }

        if check_win(&grid) {
            println!("{} won the game!", current_player.name);
            is_active = false;
        }
    }
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
