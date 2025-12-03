fn part1(battery_banks: impl AsRef<[Vec<u32>]>) {
    let total: u32 = battery_banks
        .as_ref()
        .iter()
        .map(|battery_bank| {
            let (left, right) =
                battery_bank
                    .windows(2)
                    .fold((0, 0), |(mut left, mut right), pair| {
                        if pair.len() < 2 {
                            if pair[0] > right {
                                right = pair[0];
                            }
                        } else {
                            if pair[0] > left {
                                left = pair[0];
                                right = pair[1];
                            } else if pair[1] > right {
                                right = pair[1];
                            }
                        }

                        (left, right)
                    });

            left * 10 + right
        })
        .sum();

    //assert!(total == 17324);
    println!("Total: {}", total);
}

// this probably could be generalized and re-used as part 1
// if we used a vector instead of an array
fn part2(battery_banks: impl AsRef<[Vec<u32>]>) {
    const BATTERY_COUNT: usize = 2;

    let total: u32 = battery_banks
        .as_ref()
        .iter()
        .map(|battery_bank| {
            let v = battery_bank.windows(BATTERY_COUNT).fold(
                [0; BATTERY_COUNT],
                |mut batteries, window| {
                    // TODO: this needs to loop once we have less than BATTERY_COUNT
                    // overall just this chunk needs re-done
                    if window.len() < BATTERY_COUNT {
                        if window[0] > batteries[1] {
                            batteries[1] = window[0];
                        }
                    } else {
                        if window[0] > batteries[0] {
                            batteries[0] = window[0];
                            batteries[1] = window[1];
                        } else if window[1] > batteries[1] {
                            batteries[1] = window[1];
                        }
                    }

                    batteries
                },
            );

            println!("v={:?}", v);

            let mut joltage = 0;
            for x in 0..BATTERY_COUNT {
                joltage += v[x] * 10_u32.pow((BATTERY_COUNT - x - 1) as u32);
            }
            println!("joltage: {joltage}");

            joltage
        })
        .sum();

    //assert!(total == ???);
    println!("Total: {}", total);
}

fn main() {
    let input = include_str!("../input.txt");

    let battery_banks = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    part1(&battery_banks);
    part2(&battery_banks);
}
