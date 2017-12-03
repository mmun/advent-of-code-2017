const SIZE: usize = 13; // must be odd
type Grid = [[i64; SIZE]; SIZE];

fn solve(n: i64) -> i64 {
    let ref mut grid = [[0; SIZE]; SIZE];
    let mut i = SIZE/2;
    let mut j = SIZE/2;

    grid[i][j] = 1;
    
    for level in 1..SIZE/2 {
        i += 1;
        j += 1;

        for _ in 0..2*level {
            i -= 1;
            fill(grid, i, j);
            if grid[i][j] > n { return grid[i][j] }
        }

        for _ in 0..2*level {
            j -= 1;
            fill(grid, i, j);
            if grid[i][j] > n { return grid[i][j] }
        }

        for _ in 0..2*level {
            i += 1;
            fill(grid, i, j);
            if grid[i][j] > n { return grid[i][j] }
        }

        for _ in 0..2*level {
            j += 1;
            fill(grid, i, j);
            if grid[i][j] > n { return grid[i][j] }
        }
    }

    -1
}

fn fill(grid: &mut Grid, i: usize, j: usize) {
    grid[i][j] = grid[i][j-1] + grid[i][j+1]
        + grid[i-1][j-1] + grid[i-1][j] + grid[i-1][j+1]
        + grid[i+1][j-1] + grid[i+1][j] + grid[i+1][j+1];
}

fn main() {
    println!("{}", solve(361527));
}
