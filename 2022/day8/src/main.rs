fn part1(grid: &Vec<Vec<u32>>) {
    let mut visible = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, &h) in row.iter().enumerate() {
            let left_visible = x == 0 || row.iter().take(x).all(|&v| v < h);
            if left_visible {
                visible += 1;
                continue;
            }

            let right_visible = x == row.len() - 1 || row.iter().skip(x + 1).all(|&v| v < h);
            if right_visible {
                visible += 1;
                continue;
            }

            let top_visible = y == 0 || grid.iter().take(y).all(|v| v[x] < h);
            if top_visible {
                visible += 1;
                continue;
            }

            let bottom_visible = y == grid.len() - 1 || grid.iter().skip(y + 1).all(|v| v[x] < h);
            if bottom_visible {
                visible += 1;
                continue;
            }
        }
    }

    assert!(visible == 1679);
    println!("Visible trees: {}", visible);
}

fn part2(grid: &Vec<Vec<u32>>) {
    let mut scores = Vec::with_capacity(grid.len() * grid[0].len());
    for (y, row) in grid.iter().enumerate() {
        for (x, &h) in row.iter().enumerate() {
            let mut left_score = 0;
            for &v in row.iter().take(x).rev() {
                left_score += 1;
                if v >= h {
                    break;
                }
            }

            let mut right_score = 0;
            for &v in row.iter().skip(x + 1) {
                right_score += 1;
                if v >= h {
                    break;
                }
            }

            let mut top_score = 0;
            for v in grid.iter().take(y).rev() {
                top_score += 1;
                if v[x] >= h {
                    break;
                }
            }

            let mut bottom_score = 0;
            for v in grid.iter().skip(y + 1) {
                bottom_score += 1;
                if v[x] >= h {
                    break;
                }
            }

            let score = left_score * right_score * top_score * bottom_score;
            scores.push(score);
        }
    }

    let highest = scores.iter().max().unwrap();
    assert!(*highest == 536625);
    println!("Highest score: {}", highest);
}

fn main() {
    let input = include_str!("../input.txt");

    let values = input
        .lines()
        .filter_map(|x| {
            let x = x.trim();
            if x.is_empty() {
                return None;
            }

            let row = x
                .chars()
                .filter_map(|x| {
                    if x.is_whitespace() {
                        return None;
                    }

                    Some(x.to_digit(10).unwrap())
                })
                .collect::<Vec<_>>();

            Some(row)
        })
        .collect::<Vec<_>>();

    part1(&values);
    part2(&values);
}
