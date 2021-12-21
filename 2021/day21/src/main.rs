use std::cell::RefCell;
use std::collections::HashMap;

use cached::proc_macro::cached;
use regex::Regex;

#[derive(Debug)]
struct DeterministicDie {
    sides: usize,
    counter: RefCell<usize>,
    rolls: RefCell<usize>,
}

impl DeterministicDie {
    fn new(sides: usize) -> Self {
        Self {
            sides,
            counter: RefCell::new(0),
            rolls: RefCell::new(0),
        }
    }

    fn roll(&self) -> usize {
        let counter = *self.counter.borrow();
        let ret = 1 + counter;

        *self.counter.borrow_mut() = (counter + 1) % self.sides;
        *self.rolls.borrow_mut() += 1;

        ret
    }

    fn roll_count(&self) -> usize {
        *self.rolls.borrow()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pawn {
    position: usize,
    score: usize,
}

impl Pawn {
    fn new(position: usize) -> Self {
        Self { position, score: 0 }
    }

    fn move_spaces(&mut self, track_len: usize, spaces: usize) {
        self.position = (self.position + spaces) % track_len;
        self.score += self.position + 1;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Universe {
    player1: Pawn,
    player2: Pawn,
}

impl Universe {
    fn new(player1: Pawn, player2: Pawn) -> Self {
        Self { player1, player2 }
    }
}

fn part1(mut player1: Pawn, mut player2: Pawn, track_len: usize, max_score: usize) {
    let die = DeterministicDie::new(100);

    let loser_score = loop {
        let spaces = die.roll() + die.roll() + die.roll();
        player1.move_spaces(track_len, spaces);
        if player1.score >= max_score {
            break player2.score;
        }

        let spaces = die.roll() + die.roll() + die.roll();
        player2.move_spaces(track_len, spaces);
        if player2.score >= max_score {
            break player1.score;
        }
    };

    let total = loser_score * die.roll_count();
    assert!(total == 998088);
    println!(
        "Loser total {}*{} = {}",
        loser_score,
        die.roll_count(),
        total
    );
}

const SPACES: [(usize, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

#[cached]
fn simulate_player1(
    universe: Universe,
    track_len: usize,
    max_score: usize,
) -> (u64, HashMap<Universe, u64>) {
    let mut spawned = vec![universe; SPACES.len()];
    let mut universes = HashMap::new();
    let mut player1wins = 0;
    for (i, universe) in spawned.iter_mut().enumerate() {
        let spaces = SPACES[i];
        universe.player1.move_spaces(track_len, spaces.0);
        if universe.player1.score >= max_score {
            player1wins += spaces.1;
        } else {
            *universes.entry(universe.clone()).or_insert(0) += spaces.1;
        }
    }
    (player1wins, universes)
}

#[cached]
fn simulate_player2(
    universe: Universe,
    track_len: usize,
    max_score: usize,
) -> (u64, HashMap<Universe, u64>) {
    let mut spawned = vec![universe; SPACES.len()];
    let mut universes = HashMap::new();
    let mut player2wins = 0;
    for (i, universe) in spawned.iter_mut().enumerate() {
        let spaces = SPACES[i];
        universe.player2.move_spaces(track_len, spaces.0);
        if universe.player2.score >= max_score {
            player2wins += spaces.1;
        } else {
            *universes.entry(universe.clone()).or_insert(0) += spaces.1;
        }
    }
    (player2wins, universes)
}

#[cached]
fn simulate(
    universe: Universe,
    track_len: usize,
    max_score: usize,
) -> (u64, u64, HashMap<Universe, u64>) {
    let mut player1wins = 0;
    let mut player2wins = 0;

    // spawn all of the possible universes for player 1
    let (wins, mut spawned) = simulate_player1(universe, track_len, max_score);
    player1wins += wins;

    // spawn all of the possible universes for player 2
    let spawned: Vec<HashMap<Universe, u64>> = spawned
        .drain()
        .map(|(universe, count)| {
            let (wins, mut spawned) = simulate_player2(universe, track_len, max_score);
            player2wins += wins * count;
            spawned.values_mut().for_each(|c| *c *= count);
            spawned
        })
        .collect();

    // combine
    let mut universes = HashMap::new();
    for entry in spawned {
        for (universe, count) in entry {
            *universes.entry(universe).or_insert(0) += count;
        }
    }

    (player1wins, player2wins, universes)
}

fn part2(player1: Pawn, player2: Pawn, track_len: usize, max_score: usize) {
    let mut universes = HashMap::new();
    universes.insert(Universe::new(player1, player2), 1);

    let mut player1wins = 0_u64;
    let mut player2wins = 0_u64;

    while !universes.is_empty() {
        // spawn all of the possible universes
        let spawned: Vec<HashMap<Universe, u64>> = universes
            .drain()
            .map(|(universe, count)| {
                let (p1wins, p2wins, mut spawned) = simulate(universe, track_len, max_score);
                player1wins += p1wins * count;
                player2wins += p2wins * count;
                spawned.values_mut().for_each(|c| *c *= count);
                spawned
            })
            .collect();

        // combine
        for entry in spawned {
            for (universe, count) in entry {
                *universes.entry(universe).or_insert(0) += count;
            }
        }
    }

    assert!(player1wins == 306621346123766);
    assert!(player2wins == 166105651528183);
    println!(
        "Player 1 won {} universes, player 2 won {} universes",
        player1wins, player2wins
    );
}

fn main() {
    let input = include_str!("../input.txt").trim();

    let re = Regex::new(
        r"Player 1 starting position: (\d+)
Player 2 starting position: (\d+)",
    )
    .unwrap();
    let captures = re.captures(input).unwrap();

    let p1start: usize = (&captures[1]).parse().unwrap();
    let p2start: usize = (&captures[2]).parse().unwrap();

    let player1 = Pawn::new(p1start - 1);
    let player2 = Pawn::new(p2start - 1);

    part1(player1, player2, 10, 1000);
    part2(player1, player2, 10, 21);
}
