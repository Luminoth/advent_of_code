use std::collections::HashMap;

// recursive solution taken from https://www.reddit.com/r/adventofcode/comments/1hbmu6q/comment/m1hq0bo/
fn blink_stone(stone: usize, blinks: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    // memoize as we go because a lot of this is going to repeat
    let key = (stone, blinks);
    if cache.contains_key(&key) {
        return cache[&key];
    }

    // base case
    if blinks == 0 {
        return 1;
    }

    // recursive case
    let v = if stone == 0 {
        blink_stone(1, blinks - 1, cache)
    } else {
        let digits: u32 = stone.ilog(10) + 1;
        if digits % 2 == 0 {
            let h = (digits / 2) as usize;

            // TODO: this is so very, very jank
            let t = stone.to_string();
            let a = t[..h].parse::<usize>().unwrap();
            let b = t[h..].parse::<usize>().unwrap();

            blink_stone(a, blinks - 1, cache) + blink_stone(b, blinks - 1, cache)
        } else {
            blink_stone(stone * 2024, blinks - 1, cache)
        }
    };

    cache.insert(key, v);
    v
}

fn part1(stones: &[usize]) {
    let mut cache = HashMap::new();
    let mut count = 0;
    for stone in stones.iter() {
        count += blink_stone(*stone, 25, &mut cache);
    }

    assert!(count == 186996);
    println!("stones: {}", count);
}

#[allow(dead_code)]
fn part2(stones: &[usize]) {
    let mut cache = HashMap::new();
    let mut count = 0;
    for stone in stones.iter() {
        count += blink_stone(*stone, 75, &mut cache);
    }

    assert!(count == 221683913164898);
    println!("stones: {}", count);
}

fn main() {
    let input = include_str!("../input.txt");

    let stones = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    part1(&stones);
    part2(&stones);
}
