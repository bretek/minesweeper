extern crate rand;

use rand::thread_rng;
use rand::Rng;

fn gen_grid (width: usize, height: usize, mut num_bombs: u64) -> Vec<Vec<u8>> {
    let mut grid: Vec<Vec<u8>> = vec![vec![0; width]; height];
    let mut rng = thread_rng();
    let mut x: usize = rng.gen_range(0..width);
    let mut y: usize = rng.gen_range(0..height);
    loop {
        if grid[x][y] == 0 {
            //place bomb at random location
            grid[x][y] = 29;
            num_bombs-=1;
            if num_bombs == 0 {
                //add numbers to grid
                for row in 0..height {
                    for col in 0..width {
                        if grid[row][col] >= 29 {
                            let mut allow_left = 1;
                            let mut allow_right = 1;
                            let mut allow_up = 1;
                            let mut allow_down = 1;
                            //check if against edge of grid
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
                            //left
                            if allow_left == 1 {
                                grid[row][col-1] += 1;
                                if allow_up == 1 {
                                    grid[row-1][col-1] += 1;
                                }
                                if allow_down == 1 {
                                    grid[row+1][col-1] += 1;
                                }
                            }
                            //right
                            if allow_right == 1 {
                                grid[row][col+1] += 1;
                                if allow_up == 1 {
                                    grid[row-1][col+1] += 1;
                                }
                                if allow_down == 1 {
                                    grid[row+1][col+1] += 1;
                                }
                            }
                            //up
                            if allow_up == 1 {
                                grid[row-1][col] += 1;
                            }
                            //down
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

fn print_grid (width: usize, height: usize, show_bombs: bool, grid: &Vec<Vec<u8>>) {
    let mut symbol: char;
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
    for row in 0..height {
        print!("{}|", row);
        for col in 0..width {
            symbol = match grid[row][col] {
                symbol if symbol >= 29 && symbol <= 38 => {
                    if show_bombs {
                        '*'
                    }
                    else {
                        '.'
                    }
                },
                symbol if symbol >= 39 && symbol < 48 => '^',
                10..=20 => '^',
                0..=8 => '.',
                _ => grid[row][col] as char
            };
            
            print!("{}", symbol);
        }
        print!("\n");
    }
}

fn make_move(x: usize, y: usize, move_type: bool, grid: &mut Vec<Vec<u8>>) -> bool {
    if move_type == true {
        if grid[x][y] >= 39 || grid[x][y] < 20 && grid[x][y] >= 10 {
            grid[x][y] -= 10;
        }
        else {
            grid[x][y] += 10;
        }
    }
    else {
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
    let width = 10;
    let height = 10;
    let num_bombs = 10;
    let mut grid = gen_grid(width, height, num_bombs);
    print_grid(width, height, false, &grid);
    let mut x;
    let mut y;
    let mut game_over = 0;
    let mut move_type = false;
    let mut line = String::new();
    while game_over == 0 {
        println!("Enter x y coords in format x y, or type flag to switch to flag placing mode");
        std::io::stdin().read_line(&mut line).unwrap();
        if line == "flag\n" {
            move_type = !move_type;
            line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            println!("Enter x y coords in format x y");
        }

        let split: Vec<_> = line.split_whitespace().collect();
        x = split[0].parse::<usize>().unwrap();
        y = split[1].parse::<usize>().unwrap();
        line = String::new();

        println!("x: {}, y: {}", x, y);

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
fix bug when selecting same square twice
input validation
win condition
clear all adjacent 0s
*/