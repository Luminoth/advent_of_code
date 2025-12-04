fn part1(grid: impl AsRef<[Vec<u32>]>) {
    let grid = grid.as_ref();

    let mut total = 0;

    for y in 0..grid.len() {
        let row = &grid[y];
        for x in 0..row.len() {
            // is there a roll here?
            if row[x] == 0 {
                continue;
            }

            let mut count = 0;

            if y > 0 {
                // top left
                if x > 0 {
                    count += grid[y - 1][x - 1];
                }

                // top
                count += grid[y - 1][x];

                // top right
                if x < row.len() - 1 {
                    count += grid[y - 1][x + 1];
                }
            }

            // left
            if x > 0 {
                count += row[x - 1]
            }

            // right
            if x < row.len() - 1 {
                count += row[x + 1]
            }

            if y < grid.len() - 1 {
                // bottom left
                if x > 0 {
                    count += grid[y + 1][x - 1];
                }

                // bottom
                count += grid[y + 1][x];

                // bottom right
                if x < row.len() - 1 {
                    count += grid[y + 1][x + 1];
                }
            }

            if count < 4 {
                total += 1;
            }
        }
    }

    assert!(total == 1549);
    println!("Total: {}", total);
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
}
