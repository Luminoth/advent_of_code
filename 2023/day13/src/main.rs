use std::cell::RefCell;

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<char>>,

    reflection_row: RefCell<Option<usize>>,
    reflection_col: RefCell<Option<usize>>,
}

impl From<&str> for Grid {
    fn from(v: &str) -> Self {
        let grid = v
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Self {
            grid,
            reflection_row: RefCell::new(None),
            reflection_col: RefCell::new(None),
        }
    }
}

fn is_horizontal_reflection(grid: &[Vec<char>], row: usize) -> bool {
    for v in 0..=row {
        let a = row - v;
        let b = row + v + 1;
        if b >= grid.len() {
            break;
        }
        //println!("checking row {} against row {}", a, b);

        if !grid[a].iter().zip(grid[b].iter()).all(|(a, b)| a == b) {
            return false;
        }
    }
    true
}

fn is_vertical_reflection(grid: &[Vec<char>], col: usize) -> bool {
    for v in 0..=col {
        let a = col - v;
        let b = col + v + 1;
        if b >= grid[0].len() {
            break;
        }
        //println!("checking col {} against col {}", a, b);

        let mut reflected = true;
        for y in 0..grid.len() {
            if grid[y][a] != grid[y][b] {
                reflected = false;
                break;
            }
        }

        if !reflected {
            return false;
        }
    }
    true
}

fn score_grid(grid: &Grid) -> usize {
    let mut score = 0;

    for y in 0..(grid.grid.len() - 1) {
        if *grid.reflection_row.borrow() == Some(y) {
            continue;
        }

        //println!("checking row {}", y);
        if is_horizontal_reflection(&grid.grid, y) {
            let value = 100 * (y + 1);
            /*println!(
                "found horizontal reflection at row {}, value += {}",
                y, value
            );*/

            grid.reflection_row.replace(Some(y));
            score += value;
            break;
        }
    }

    for x in 0..(grid.grid[0].len() - 1) {
        if *grid.reflection_col.borrow() == Some(x) {
            continue;
        }

        //println!("checking col {}", x);
        if is_vertical_reflection(&grid.grid, x) {
            let value = x + 1;
            //println!("found vertical reflection at col {}, value += {}", x, value);

            grid.reflection_col.replace(Some(x));
            score += value;
            break;
        }
    }

    score
}

fn part1(grids: &[Grid]) {
    let mut total = 0;

    for grid in grids {
        let score = score_grid(grid);
        //println!("scored: {}", score);
        total += score;
        //println!("");
    }

    //assert!(total == 36041);
    println!("Total: {}", total);
}

fn part2(mut grids: Vec<Grid>) {
    let mut total = 0;

    for grid in &mut grids {
        'outer: for y in 0..grid.grid.len() {
            if *grid.reflection_row.borrow() == Some(y) {
                continue;
            }

            for x in 0..grid.grid[0].len() {
                if *grid.reflection_col.borrow() == Some(x) {
                    continue;
                }

                let prev = grid.grid[y][x];
                match prev {
                    '#' => grid.grid[y][x] = '.',
                    '.' => grid.grid[y][x] = '#',
                    _ => unreachable!(),
                }

                let score = score_grid(&grid);
                grid.grid[y][x] = prev;

                if score != 0 {
                    //println!("scored grid after changing ({}, {}): {}", x, y, score);
                    total += score;
                    break 'outer;
                }
            }
        }
        //println!("");
    }

    assert!(total == 35915);
    println!("Total: {}", total);
}

fn main() {
    let input = include_str!("../input.txt");
    let grids = input.split("\n\n").map(Grid::from).collect::<Vec<_>>();

    part1(&grids);
    part2(grids);
}
