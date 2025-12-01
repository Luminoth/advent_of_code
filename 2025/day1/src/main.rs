#![allow(dead_code)]

fn wrap_mod(a: i32, b: i32) -> i32 {
    ((a % b) + b) % b
}

fn floor_div(a: i64, b: i64) -> i64 {
    let (q, r) = (a / b, a % b);
    if r < 0 { q - 1 } else { q }
}

fn part1(rotations: impl AsRef<[i32]>) {
    let rotations = rotations.as_ref();

    let mut zero_count = 0;
    let mut value = 50;
    for rotation in rotations {
        value = wrap_mod(value + rotation, 100);
        if value == 0 {
            zero_count += 1;
        }
    }

    assert!(zero_count == 1036);
    println!("Zero stop count: {}", zero_count);
}

// not sure how much more optimized this actually is
// but the idea of using an accumulator is neat
fn optimized_part1(rotations: impl AsRef<[i32]>) {
    let rotations = rotations.as_ref();

    let mut zero_count = 0;
    let mut accum = 50_i64;

    for rotation in rotations {
        accum += *rotation as i64;
        if accum % 100 == 0 {
            zero_count += 1;
        }
    }

    assert!(zero_count == 1036);
    println!("Optimized Zero stop count: {}", zero_count);
}

// TODO: I don't like this and I wonder if there's a way to do it with an accumulator
// the upside is, this is avoiding arithmetic simulation, so that's good
fn part2(rotations: impl AsRef<[i32]>) {
    let rotations = rotations.as_ref();

    let mut zero_count = 0;
    let mut value = 50;
    for rotation in rotations {
        // we might wrap 0 a few times before landing on the final value
        let passes = (rotation / 100).abs();
        zero_count += passes;

        let prev = value;

        value = wrap_mod(value + rotation, 100);
        if value == 0 {
            zero_count += 1;
        } else if *rotation < 0 {
            // if we landed on the opposite side, we crossed zero
            if prev != 0 && value > prev {
                zero_count += 1;
            }
        } else if *rotation > 0 {
            // if we landed on the opposite side, we crossed zero
            if prev != 0 && value < prev {
                zero_count += 1;
            }
        }
    }

    assert!(zero_count == 6228);
    println!("Zero click count: {}", zero_count);
}

fn main() {
    let input = include_str!("../input.txt");

    let values = input
        .lines()
        .map(|line| line.replace("R", "").replace("L", "-").parse().unwrap())
        .collect::<Vec<_>>();

    optimized_part1(&values);
    part2(&values);
}
