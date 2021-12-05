#[derive(Debug, Default, Copy, Clone)]
struct Vec2 {
    x: isize,
    y: isize,
}

impl From<&str> for Vec2 {
    fn from(item: &str) -> Self {
        let v = item.split_once(',').unwrap();
        Self {
            x: v.0.trim().parse().unwrap(),
            y: v.1.trim().parse().unwrap(),
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Grid {
    grid: Vec<Vec<usize>>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        let mut grid = Vec::with_capacity(height + 1);
        for _ in 0..grid.capacity() {
            grid.push(vec![0; width + 1]);
        }
        Self { grid }
    }

    fn set(&mut self, pos: Vec2) {
        let x = pos.x as usize;
        let y = pos.y as usize;

        let row = self.grid.get_mut(y).unwrap();
        row[x] = row.get(x).unwrap() + 1;
    }

    fn apply_path(&mut self, path: (Vec2, Vec2), allow_diagonal: bool) {
        let dx = path.1.x - path.0.x;
        let dy = path.1.y - path.0.y;
        if dx != 0 && dy != 0 && (!allow_diagonal || dx.abs() != dy.abs()) {
            return;
        }

        let mut v = path.0;
        while v.x != path.1.x || v.y != path.1.y {
            self.set(v);

            v.x += dx.signum();
            v.y += dy.signum();
        }
        self.set(v);
    }

    fn dangerous_area_count(&self) -> usize {
        let mut count = 0;
        for row in &self.grid {
            for col in row {
                if *col >= 2 {
                    count += 1;
                }
            }
        }
        count
    }

    #[allow(dead_code)]
    fn render(&self) {
        println!("{} rows", self.grid.len());
        for row in &self.grid {
            for col in row {
                if *col == 0 {
                    print!(".");
                } else {
                    print!("{}", col);
                }
            }
            println!();
        }
    }
}

fn part1(mut grid: Grid, paths: impl AsRef<[(Vec2, Vec2)]>) {
    for path in paths.as_ref() {
        grid.apply_path(*path, false);
    }
    //grid.render();

    assert!(grid.dangerous_area_count() == 6856);
    println!("There are {} dangerous areas", grid.dangerous_area_count());
}

fn part2(mut grid: Grid, paths: impl AsRef<[(Vec2, Vec2)]>) {
    for path in paths.as_ref() {
        grid.apply_path(*path, true);
    }
    //grid.render();

    assert!(grid.dangerous_area_count() == 20666);
    println!("There are {} dangerous areas", grid.dangerous_area_count());
}

fn main() {
    let input = include_str!("../input.txt");

    let paths: Vec<(Vec2, Vec2)> = input
        .lines()
        .filter_map(|x| {
            let x = x.trim();
            if x.is_empty() {
                return None;
            }

            let v = x.split_once("->").unwrap();
            Some((v.0.into(), v.1.into()))
        })
        .collect();

    let width = paths.iter().fold(0, |v, p| p.0.x.max(p.1.x).max(v));
    let height = paths.iter().fold(0, |v, p| p.0.y.max(p.1.y).max(v));
    let grid = Grid::new(width as usize, height as usize);

    part1(grid.clone(), &paths);
    part2(grid, &paths);
}
