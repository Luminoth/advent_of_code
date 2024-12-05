fn part1(puzzle: &[Vec<char>]) {
    let mut count = 0;

    let mut y = 0;
    while y < puzzle.len() {
        let mut x = 0;
        while x < puzzle[y].len() {
            if puzzle[y][x] == 'X' {
                // W
                if x > 0 && puzzle[y][x - 1] == 'M' {
                    if x > 1 && puzzle[y][x - 2] == 'A' {
                        if x > 2 && puzzle[y][x - 3] == 'S' {
                            count += 1;
                        }
                    }
                }

                // E
                if x < puzzle[y].len() - 1 && puzzle[y][x + 1] == 'M' {
                    if x < puzzle[y].len() - 2 && puzzle[y][x + 2] == 'A' {
                        if x < puzzle[y].len() - 3 && puzzle[y][x + 3] == 'S' {
                            count += 1;
                        }
                    }
                }

                // N
                if y > 0 && puzzle[y - 1][x] == 'M' {
                    if y > 1 && puzzle[y - 2][x] == 'A' {
                        if y > 2 && puzzle[y - 3][x] == 'S' {
                            count += 1;
                        }
                    }
                }

                // S
                if y < puzzle.len() - 1 && puzzle[y + 1][x] == 'M' {
                    if y < puzzle.len() - 2 && puzzle[y + 2][x] == 'A' {
                        if y < puzzle.len() - 3 && puzzle[y + 3][x] == 'S' {
                            count += 1;
                        }
                    }
                }

                // NW
                if y > 0 && x > 0 && puzzle[y - 1][x - 1] == 'M' {
                    if y > 1 && x > 1 && puzzle[y - 2][x - 2] == 'A' {
                        if y > 2 && x > 2 && puzzle[y - 3][x - 3] == 'S' {
                            count += 1;
                        }
                    }
                }

                // SW
                if y < puzzle.len() - 1 && x > 0 && puzzle[y + 1][x - 1] == 'M' {
                    if y < puzzle.len() - 2 && x > 1 && puzzle[y + 2][x - 2] == 'A' {
                        if y < puzzle.len() - 3 && x > 2 && puzzle[y + 3][x - 3] == 'S' {
                            count += 1;
                        }
                    }
                }

                // NE
                if y > 0 && x < puzzle[y].len() - 1 && puzzle[y - 1][x + 1] == 'M' {
                    if y > 1 && x < puzzle[y].len() - 2 && puzzle[y - 2][x + 2] == 'A' {
                        if y > 2 && x < puzzle[y].len() - 3 && puzzle[y - 3][x + 3] == 'S' {
                            count += 1;
                        }
                    }
                }

                // SE
                if y < puzzle.len() - 1 && x < puzzle[y].len() - 1 && puzzle[y + 1][x + 1] == 'M' {
                    if y < puzzle.len() - 2
                        && x < puzzle[y].len() - 2
                        && puzzle[y + 2][x + 2] == 'A'
                    {
                        if y < puzzle.len() - 3
                            && x < puzzle[y].len() - 3
                            && puzzle[y + 3][x + 3] == 'S'
                        {
                            count += 1;
                        }
                    }
                }
            }
            x += 1;
        }
        y += 1;
    }

    assert!(count == 2370);
    println!("Count: {}", count);
}

fn main() {
    let input = include_str!("../input.txt");

    let puzzle = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    part1(&puzzle);
}
