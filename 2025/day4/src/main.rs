fn can_access(grid: impl AsRef<[Vec<u32>]>, x: usize, y: usize) -> bool {
    let grid = grid.as_ref();

    let mut count = 0;

    if y > 0 {
        // top left
        if x > 0 {
            count += grid[y - 1][x - 1];
        }

        // top
        count += grid[y - 1][x];

        // top right
        if x < grid[y].len() - 1 {
            count += grid[y - 1][x + 1];
        }
    }

    // left
    if x > 0 {
        count += grid[y][x - 1]
    }

    // right
    if x < grid[y].len() - 1 {
        count += grid[y][x + 1]
    }

    if y < grid.len() - 1 {
        // bottom left
        if x > 0 {
            count += grid[y + 1][x - 1];
        }

        // bottom
        count += grid[y + 1][x];

        // bottom right
        if x < grid[y].len() - 1 {
            count += grid[y + 1][x + 1];
        }
    }

    count < 4
}

fn part1(grid: impl AsRef<[Vec<u32>]>) {
    let grid = grid.as_ref();

    let mut total = 0;

    for y in 0..grid.len() {
        for (x, v) in grid[y].iter().enumerate() {
            // is there a roll here?
            if *v == 0 {
                continue;
            }

            if can_access(grid, x, y) {
                total += 1;
            }
        }
    }

    assert!(total == 1549);
    println!("Total: {}", total);
}

fn remove(mut grid: impl AsMut<[Vec<u32>]>) -> bool {
    let grid = grid.as_mut();

    for y in 0..grid.len() {
        for (x, v) in grid[y].iter().enumerate() {
            // is there a roll here?
            if *v == 0 {
                continue;
            }

            if can_access(&grid, x, y) {
                grid[y][x] = 0;
                return true;
            }
        }
    }

    false
}

fn part2(mut grid: Vec<Vec<u32>>) {
    let mut removed = 0;
    while remove(&mut grid) {
        removed += 1;
    }

    assert!(removed == 8887);
    println!("Removed: {}", removed);
}

fn main() {
    let input = include_str!("../input.txt");

    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| if ch == '@' { 1 } else { 0 })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    part1(&grid);
    part2(grid);
}
