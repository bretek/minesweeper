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
            grid[x][y] = 10;
            num_bombs-=1;
            if num_bombs == 0 {
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
            match grid[row][col] {
                10 => symbol = '*',
                20 => symbol = '^',
                0 => symbol = '.',
                _ => {
                    symbol = grid[row][col] as char;
                }
            }
            
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
