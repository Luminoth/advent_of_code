#[derive(Debug, Clone)]
struct HeightMap {
    grid: Vec<Vec<usize>>,
}

impl HeightMap {
    fn get_height(&self, row: usize, col: usize) -> Option<usize> {
        let r = self.grid.get(row);
        if let Some(r) = r {
            return r.get(col).copied();
        }
        None
    }

    fn is_lowest_point(&self, row: usize, col: usize) -> bool {
        let height = self.get_height(row, col).unwrap();

        // up
        if row > 0 {
            if let Some(up) = self.get_height(row - 1, col) {
                if height >= up {
                    return false;
                }
            }
        }

        // down
        if let Some(down) = self.get_height(row + 1, col) {
            if height >= down {
                return false;
            }
        }

        // left
        if col > 0 {
            if let Some(left) = self.get_height(row, col - 1) {
                if height >= left {
                    return false;
                }
            }
        }

        // right
        if let Some(right) = self.get_height(row, col + 1) {
            if height >= right {
                return false;
            }
        }

        true
    }

    fn basin_size(&self, row: usize, col: usize, visited: &mut Vec<(usize, usize)>) -> usize {
        if visited.contains(&(row, col)) {
            return 0;
        }
        visited.push((row, col));

        if let Some(height) = self.get_height(row, col) {
            if height >= 9 {
                return 0;
            }
        } else {
            return 0;
        }

        let mut size = 1;

        // up
        if row > 0 {
            size += self.basin_size(row - 1, col, visited);
        }

        // down
        size += self.basin_size(row + 1, col, visited);

        // left
        if col > 0 {
            size += self.basin_size(row, col - 1, visited);
        }

        // right
        size += self.basin_size(row, col + 1, visited);

        size
    }

    fn find_lowest_points(&self) -> Vec<((usize, usize), usize)> {
        self.grid
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter().enumerate().filter_map(move |(x, &v)| {
                    if self.is_lowest_point(y, x) {
                        Some(((y, x), v))
                    } else {
                        None
                    }
                })
            })
            .flatten()
            .collect()
    }
}

impl From<Vec<Vec<usize>>> for HeightMap {
    fn from(grid: Vec<Vec<usize>>) -> Self {
        Self { grid }
    }
}

fn part1(heightmap: &HeightMap) {
    let lowest_points = heightmap.find_lowest_points();

    let risk_levels: Vec<usize> = lowest_points.iter().map(|(_, height)| height + 1).collect();
    let total: usize = risk_levels.iter().sum();

    assert!(total == 528);
    println!("Total risk level: {}", total);
}

fn part2(heightmap: &HeightMap) {
    let lowest_points = heightmap.find_lowest_points();

    let mut visited = Vec::new();
    let mut basin_sizes: Vec<usize> = lowest_points
        .iter()
        .map(|((row, col), _)| heightmap.basin_size(*row, *col, &mut visited))
        .collect();
    basin_sizes.sort_unstable();
    basin_sizes.reverse();

    let total = basin_sizes[0] * basin_sizes[1] * basin_sizes[2];

    assert!(total == 920448);
    println!("Basin size product: {:?}", total);
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

    let heightmap = grid.into();
    part1(&heightmap);
    part2(&heightmap);
}
