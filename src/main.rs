extern crate rand;

use rand::thread_rng;
use rand::Rng;

// generate grid of given width, height with given number of bombs in random places
fn gen_grid (width: usize, height: usize, mut num_bombs: u64) -> Vec<Vec<u8>> {
    let mut grid: Vec<Vec<u8>> = vec![vec![0; width]; height];

    //randomly generate bomb location
    let mut rng = thread_rng();
    let mut x: usize = rng.gen_range(0..width);
    let mut y: usize = rng.gen_range(0..height);

    loop {
        // if square is not already bomb
        if grid[x][y] == 0 {
            // place bomb in square
            grid[x][y] = 29;
            num_bombs-=1;
            // if all bombs placed
            if num_bombs == 0 {
                // add numbers to grid
                for row in 0..height {
                    for col in 0..width {
                        if grid[row][col] >= 29 {
                            let mut allow_left = 1;
                            let mut allow_right = 1;
                            let mut allow_up = 1;
                            let mut allow_down = 1;
                            // check if against edge of grid
                            if col == 0 {
                                allow_left = 0;
                            }
                            if col == width-1 {
                                allow_right = 0;
                            }
                            if row == 0 {
                                allow_up = 0;
                            }
                            if row == height-1 {
                                allow_down = 0;
                            }
                            // increase adjacent bomb number for all adjacent squares
                            // left
                            if allow_left == 1 {
                                grid[row][col-1] += 1;
                                if allow_up == 1 {
                                    grid[row-1][col-1] += 1;
                                }
                                if allow_down == 1 {
                                    grid[row+1][col-1] += 1;
                                }
                            }
                            // right
                            if allow_right == 1 {
                                grid[row][col+1] += 1;
                                if allow_up == 1 {
                                    grid[row-1][col+1] += 1;
                                }
                                if allow_down == 1 {
                                    grid[row+1][col+1] += 1;
                                }
                            }
                            // up
                            if allow_up == 1 {
                                grid[row-1][col] += 1;
                            }
                            // down
                            if allow_down == 1 {
                                grid[row+1][col] += 1;
                            }
                        }
                    }
                }
                break;
            }
        }

        x = rng.gen_range(0..width);
        y = rng.gen_range(0..height);
    }

    return grid;
}

// prints game grid, with or without bombs shown
fn print_grid (width: usize, height: usize, show_bombs: bool, grid: &Vec<Vec<u8>>) {
    let mut symbol: char;

    // print top coord bar
    print!("  ");
    for col in 0..width {
        print!("{}", col);
    }
    print!("\n");
    print!("  ");
    for _ in 0..width {
        print!("-");
    }
    print!("\n");

    // print rows
    for row in 0..height {
        // print left coord bar
        print!("{}|", row);
        for col in 0..width {
            symbol = match grid[row][col] {
                // bomb symbols
                symbol if symbol >= 29 && symbol <= 38 => {
                    if show_bombs {
                        '*'
                    }
                    else {
                        '.'
                    }
                },
                // flag symbols
                symbol if symbol >= 39 && symbol < 48 => '^',
                10..=20 => '^',
                //unrevealed numbers
                0..=8 => '.',
                // numbers
                _ => grid[row][col] as char
            };
            
            print!("{}", symbol);
        }
        print!("\n");
    }
}

// change value in grid based on move, returns false if bomb hit
fn make_move(x: usize, y: usize, move_type: bool, grid: &mut Vec<Vec<u8>>) -> bool {
    // flag setting move type
    if move_type == true {
        if grid[x][y] >= 39 || grid[x][y] < 20 && grid[x][y] >= 10 {
            grid[x][y] -= 10;
        }
        else {
            grid[x][y] += 10;
        }
    }
    // normal move type
    else {
        // hit bomb
        if grid[x][y] >= 29 && grid[x][y] < 48 {
            return false;
        }
        else if grid[x][y] <= 8 {
            grid[x][y] += 48;
        }
    }
    return true;
}

fn main() {
    // setup grid
    let width = 10;
    let height = 10;
    let num_bombs = 10;
    let mut grid = gen_grid(width, height, num_bombs);
    print_grid(width, height, false, &grid);

    let mut x;
    let mut y;
    let mut game_over = 0;
    let mut move_type = false;
    let mut line;

    // main game loop
    while game_over == 0 {
        loop {
            // get input data
            line = String::new();
            println!("Enter x y coords in format x y, or type flag to switch to flag placing mode");
            std::io::stdin().read_line(&mut line).unwrap();
            if line == "flag\n" {
                move_type = !move_type;
                line = String::new();
                println!("Enter x y coords in format x y");
                std::io::stdin().read_line(&mut line).unwrap();
            }

            // parse input
            let split: Vec<_> = line.split_whitespace().collect();
            if split.len() == 2 {
                let mut fail = false;
                x = split[0].parse::<usize>().unwrap_or_else(|_| {
                    println!("x coord wrong"); fail = true; 0
                });
                y = split[1].parse::<usize>().unwrap_or_else(|_| {
                    println!("y coord wrong"); fail = true; 0
                });
                // if valid number input
                if !fail {
                    if x < width && y < height {
                        println!("x: {}, y: {}", x, y);
                        break;
                    }
                    if x < width {
                        println!("x coord out of bounds");
                    }
                    if y < height {
                        println!("y coord out of bounds");
                    }
                }
            }
        }

        // check if lost
        if make_move(x, y, move_type, &mut grid) == false {
            game_over = 1;
            print_grid(width, height, true, &grid);
        }
        else {
            print_grid(width, height, false, &grid);
        }
    }
}

/*TODO: 
win condition
clear all adjacent 0s
*/