#[derive(Debug, Clone)]
struct Board {
    grid: Vec<Vec<(usize, bool)>>,

    score: Option<usize>,
}

impl Board {
    fn new(input: impl AsRef<[String]>) -> Self {
        assert!(input.as_ref().len() == 5);
        let grid = input
            .as_ref()
            .iter()
            .map(|line| {
                assert!(line.len() <= 14);
                line.split_whitespace()
                    .map(|x| (x.parse().unwrap(), false))
                    .collect()
            })
            .collect();

        Self { grid, score: None }
    }

    fn mark(&mut self, drawing: usize) {
        if self.score.is_some() {
            return;
        }

        for row in &mut self.grid {
            for mut col in row {
                if col.0 == drawing {
                    col.1 = true;
                    break;
                }
            }
        }

        self.check_winner(drawing);
    }

    fn check_winner(&mut self, drawing: usize) {
        if self.score.is_some() {
            return;
        }

        for row in &self.grid {
            if row.iter().all(|x| x.1) {
                self.score = Some(self.score(drawing));
                return;
            }
        }

        for col in 0..self.grid[0].len() {
            if self.grid.iter().all(|row| row[col].1) {
                self.score = Some(self.score(drawing));
                return;
            }
        }
    }

    fn score(&self, drawing: usize) -> usize {
        let score: usize = self
            .grid
            .iter()
            .flatten()
            .map(|col| if col.1 { 0 } else { col.0 })
            .sum();
        score * drawing
    }
}

fn part1(drawings: impl AsRef<[usize]>, mut boards: Vec<Board>) {
    for (di, drawing) in drawings.as_ref().iter().enumerate() {
        for (bi, board) in boards.iter_mut().enumerate() {
            board.mark(*drawing);
            if let Some(score) = board.score {
                assert!(di == 26);
                assert!(*drawing == 96);
                assert!(bi == 94);
                assert!(score == 63552);

                println!("board {} is the winner: {}", bi, score);
                return;
            }
        }
    }
}

fn part2(drawings: impl AsRef<[usize]>, mut boards: Vec<Board>) {
    // reverse removal from https://github.com/zertosh/
    let mut winners = Vec::new();
    for drawing in drawings.as_ref() {
        for i in (0..boards.len()).rev() {
            boards[i].mark(*drawing);
            if boards[i].score.is_some() {
                let board = boards.remove(i);
                winners.push(board);
            }
        }

        if boards.is_empty() {
            break;
        }
    }

    let last = winners.last().unwrap();

    assert!(last.score.unwrap() == 9020);
    println!("last winning board score is {}", last.score.unwrap());
}

fn main() {
    let input = include_str!("../input.txt");

    let drawings: Vec<usize> = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let mut board_input: Vec<String> = input
        .lines()
        .skip(1)
        .filter_map(|x| {
            let x = x.trim();
            if x.is_empty() {
                return None;
            }

            Some(x.to_string())
        })
        .collect();

    let mut boards = Vec::with_capacity(board_input.len() / 5);
    while !board_input.is_empty() {
        let x: Vec<String> = board_input.drain(0..5).collect();
        let board = Board::new(x);
        boards.push(board);
    }

    part1(&drawings, boards.clone());
    part2(&drawings, boards);
}
