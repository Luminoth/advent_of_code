use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Clone)]
struct Grid {
    grid: Vec<Vec<usize>>,
}

impl Grid {
    fn size(&self) -> usize {
        self.grid.len() * self.grid[0].len()
    }

    fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut usize> {
        self.grid.get_mut(row)?.get_mut(col)
    }

    fn step(&mut self) -> usize {
        let mut flashed = HashSet::new();

        // step 1: increase energy level
        for row in &mut self.grid {
            for col in row {
                *col += 1;
            }
        }

        // step 2: flash
        loop {
            // find everything that can flash this pass of this step
            let flash: Vec<(usize, usize)> = self
                .grid
                .iter()
                .enumerate()
                .flat_map(|(y, row)| {
                    row.iter().enumerate().filter_map(move |(x, &v)| {
                        let idx = (x, y);
                        if v > 9 {
                            Some(idx)
                        } else {
                            None
                        }
                    })
                })
                .filter(|&idx| {
                    // can only flash once per step
                    if flashed.contains(&idx) {
                        return false;
                    }

                    flashed.insert(idx);
                    true
                })
                .collect();

            if flash.is_empty() {
                break;
            }

            // increase adjacent energy
            for idx in flash {
                if idx.1 > 0 {
                    // up left
                    if idx.0 > 0 {
                        *self.get_mut(idx.1 - 1, idx.0 - 1).unwrap() += 1;
                    }

                    // up
                    *self.get_mut(idx.1 - 1, idx.0).unwrap() += 1;

                    // up right
                    if let Some(r) = self.get_mut(idx.1 - 1, idx.0 + 1) {
                        *r += 1;
                    }
                }

                // right
                if let Some(r) = self.get_mut(idx.1, idx.0 + 1) {
                    *r += 1;
                }

                // down right
                if let Some(r) = self.get_mut(idx.1 + 1, idx.0 + 1) {
                    *r += 1;
                }

                // down
                if let Some(r) = self.get_mut(idx.1 + 1, idx.0) {
                    *r += 1;
                }

                if idx.0 > 0 {
                    // down left
                    if let Some(r) = self.get_mut(idx.1 + 1, idx.0 - 1) {
                        *r += 1;
                    }

                    // left
                    *self.get_mut(idx.1, idx.0 - 1).unwrap() += 1;
                }
            }
        }

        // step 3: everything that flashed goes back to 0
        for flash in &flashed {
            *self.get_mut(flash.1, flash.0).unwrap() = 0;
        }

        flashed.len()
    }
}

impl From<Vec<Vec<usize>>> for Grid {
    fn from(grid: Vec<Vec<usize>>) -> Self {
        Self { grid }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.grid {
            for v in row {
                write!(f, "{}", v)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn part1(mut grid: Grid) {
    let mut total = 0;
    for _ in 0..100 {
        total += grid.step();
    }

    assert!(total == 1757);
    println!("There were {} flashes in total", total);
}

fn part2(mut grid: Grid) {
    let mut step = 0;
    loop {
        step += 1;

        let flashed = grid.step();
        if flashed == grid.size() {
            break;
        }
    }

    assert!(step == 422);
    println!("First synchronized flash step is {}", step);
}

fn main() {
    let input = include_str!("../input.txt");

    let grid: Vec<Vec<usize>> = input
        .lines()
        .filter_map(|x| {
            let x = x.trim();
            if x.is_empty() {
                return None;
            }

            let row = x
                .chars()
                .map(|ch| ch.to_digit(10).unwrap() as usize)
                .collect();
            Some(row)
        })
        .collect();

    let grid: Grid = grid.into();
    part1(grid.clone());
    part2(grid);
}
