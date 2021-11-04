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
            grid[x][y] = 30;
            num_bombs-=1;
            if num_bombs == 0 {
                //add numbers to grid
                for row in 0..height {
                    for col in 0..width {
                        if grid[row][col] >= 30 {
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

fn print_grid (width: usize, height: usize, grid: Vec<Vec<u8>>) {
    let mut symbol: char;
    for row in 0..height {
        for col in 0..width {
            symbol = match grid[row][col] {
                symbol if symbol >= 30 && symbol <= 38 => '*',
                symbol if symbol >= 40 => '^',
                10..=20 => '^',
                0 => '.',
                _ => (grid[row][col] + 48) as char
            };
            
            print!("{}", symbol);
        }
        print!("\n");
    }
}

fn main() {
    let width = 10;
    let height = 10;
    let num_bombs = 10;
    let grid = gen_grid(width, height, num_bombs);
    print_grid(width, height, grid);
}
