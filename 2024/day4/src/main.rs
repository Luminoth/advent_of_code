#[inline]
fn matches(puzzle: &[Vec<char>], y: isize, x: isize, v: char) -> bool {
    y >= 0
        && x >= 0
        && y < puzzle.len() as isize
        && x < puzzle[y as usize].len() as isize
        && puzzle[y as usize][x as usize] == v
}

fn part1(puzzle: &[Vec<char>]) {
    let mut count = 0;

    let mut y = 0;
    while y < puzzle.len() as isize {
        let mut x = 0;
        while x < puzzle[y as usize].len() as isize {
            if matches(puzzle, y, x, 'X') {
                // W
                if matches(puzzle, y, x - 1, 'M')
                    && matches(puzzle, y, x - 2, 'A')
                    && matches(puzzle, y, x - 3, 'S')
                {
                    count += 1;
                }

                // E
                if matches(puzzle, y, x + 1, 'M')
                    && matches(puzzle, y, x + 2, 'A')
                    && matches(puzzle, y, x + 3, 'S')
                {
                    count += 1;
                }

                // N
                if matches(puzzle, y - 1, x, 'M')
                    && matches(puzzle, y - 2, x, 'A')
                    && matches(puzzle, y - 3, x, 'S')
                {
                    count += 1;
                }

                // S
                if matches(puzzle, y + 1, x, 'M')
                    && matches(puzzle, y + 2, x, 'A')
                    && matches(puzzle, y + 3, x, 'S')
                {
                    count += 1;
                }

                // NW
                if matches(puzzle, y - 1, x - 1, 'M')
                    && matches(puzzle, y - 2, x - 2, 'A')
                    && matches(puzzle, y - 3, x - 3, 'S')
                {
                    count += 1;
                }

                // SW
                if matches(puzzle, y + 1, x - 1, 'M')
                    && matches(puzzle, y + 2, x - 2, 'A')
                    && matches(puzzle, y + 3, x - 3, 'S')
                {
                    count += 1;
                }

                // NE
                if matches(puzzle, y - 1, x + 1, 'M')
                    && matches(puzzle, y - 2, x + 2, 'A')
                    && matches(puzzle, y - 3, x + 3, 'S')
                {
                    count += 1;
                }

                // SE
                if matches(puzzle, y + 1, x + 1, 'M')
                    && matches(puzzle, y + 2, x + 2, 'A')
                    && matches(puzzle, y + 3, x + 3, 'S')
                {
                    count += 1;
                }
            }
            x += 1;
        }
        y += 1;
    }

    assert!(count == 2370);
    println!("Count: {}", count);
}

fn part2(puzzle: &[Vec<char>]) {
    let mut count = 0;

    let mut y = 0;
    while y < puzzle.len() as isize {
        let mut x = 0;
        while x < puzzle[y as usize].len() as isize {
            if matches(puzzle, y, x, 'A') {
                // NW / SE
                if matches(puzzle, y - 1, x - 1, 'M') && matches(puzzle, y + 1, x + 1, 'S') {
                    // SW / NE
                    if matches(puzzle, y + 1, x - 1, 'M') && matches(puzzle, y - 1, x + 1, 'S') {
                        count += 1;
                    }

                    // NE / SW
                    if matches(puzzle, y - 1, x + 1, 'M') && matches(puzzle, y + 1, x - 1, 'S') {
                        count += 1;
                    }
                }

                // SW / NE
                if matches(puzzle, y + 1, x - 1, 'M') && matches(puzzle, y - 1, x + 1, 'S') {
                    // NW / SE
                    if matches(puzzle, y - 1, x - 1, 'M') && matches(puzzle, y + 1, x + 1, 'S') {
                        count += 1;
                    }

                    // SE / NW
                    if matches(puzzle, y + 1, x + 1, 'M') && matches(puzzle, y - 1, x - 1, 'S') {
                        count += 1;
                    }
                }

                // NE / SW
                if matches(puzzle, y - 1, x + 1, 'M') && matches(puzzle, y + 1, x - 1, 'S') {
                    // NW / SE
                    if matches(puzzle, y - 1, x - 1, 'M') && matches(puzzle, y + 1, x + 1, 'S') {
                        count += 1;
                    }

                    // SE / NW
                    if matches(puzzle, y + 1, x + 1, 'M') && matches(puzzle, y - 1, x - 1, 'S') {
                        count += 1;
                    }
                }

                // SE / NW
                if matches(puzzle, y + 1, x + 1, 'M') && matches(puzzle, y - 1, x - 1, 'S') {
                    // SW / NE
                    if matches(puzzle, y + 1, x - 1, 'M') && matches(puzzle, y - 1, x + 1, 'S') {
                        count += 1;
                    }

                    // NE / SW
                    if matches(puzzle, y - 1, x + 1, 'M') && matches(puzzle, y + 1, x - 1, 'S') {
                        count += 1;
                    }
                }
            }
            x += 1;
        }
        y += 1;
    }

    // this is dumb but it's late lol
    // (we're double counting everything)
    count /= 2;

    assert!(count == 1908);
    println!("Count: {}", count);
}

fn main() {
    let input = include_str!("../input.txt");

    let puzzle = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    part1(&puzzle);
    part2(&puzzle);
}
