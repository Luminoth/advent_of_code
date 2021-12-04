use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Board {
    grid: Vec<Vec<(usize, bool)>>,
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

        Self { grid }
    }

    fn mark(&mut self, drawing: usize) {
        for row in &mut self.grid {
            for mut col in row {
                if col.0 == drawing {
                    col.1 = true;
                    break;
                }
            }
        }
    }

    fn check_winner(&self, drawing: usize) -> Option<usize> {
        for row in &self.grid {
            let mut complete = true;
            for col in row {
                if !col.1 {
                    complete = false;
                    break;
                }
            }

            if complete {
                return Some(self.score(drawing));
            }
        }

        for col in 0..self.grid[0].len() {
            let mut complete = true;
            for row in &self.grid {
                if !row[col].1 {
                    complete = false;
                    break;
                }
            }

            if complete {
                return Some(self.score(drawing));
            }
        }

        None
    }

    fn score(&self, drawing: usize) -> usize {
        let mut score = 0;
        for row in &self.grid {
            for col in row {
                if !col.1 {
                    score += col.0;
                }
            }
        }
        score * drawing
    }
}

fn part1(drawings: impl AsRef<[usize]>, mut boards: Vec<Board>) {
    for (di, drawing) in drawings.as_ref().iter().enumerate() {
        for (bi, board) in boards.iter_mut().enumerate() {
            board.mark(*drawing);
            if let Some(score) = board.check_winner(*drawing) {
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
    let mut last_score = 0;
    let mut last_winner = 0;

    let mut winners = HashSet::new();
    for drawing in drawings.as_ref() {
        for (bi, board) in boards.iter_mut().enumerate() {
            board.mark(*drawing);
            if let Some(score) = board.check_winner(*drawing) {
                if winners.contains(&bi) {
                    continue;
                }
                winners.insert(bi);

                last_score = score;
                last_winner = bi;
            }
        }
    }

    assert!(last_winner == 93);
    assert!(last_score == 9020);
    println!("last winning board is {}: {}", last_winner, last_score);
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
