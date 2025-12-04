fn turn_on_batteries(battery_banks: impl AsRef<[Vec<u32>]>, battery_count: usize) -> u64 {
    battery_banks
        .as_ref()
        .iter()
        .map(|battery_bank| {
            let v = battery_bank.windows(battery_count).fold(
                vec![0; battery_count],
                |mut batteries, window| {
                    let start = battery_count - window.len();
                    for idx in start..window.len() {
                        if window[idx] > batteries[idx] {
                            batteries[idx..window.len()].copy_from_slice(&window[idx..]);
                            break;
                        }
                    }

                    batteries
                },
            );

            let mut joltage = 0;
            for (x, battery) in v.iter().enumerate().take(battery_count) {
                joltage += *battery as u64 * 10_u64.pow((battery_count - x - 1) as u32);
            }

            joltage
        })
        .sum()
}

fn part1(battery_banks: impl AsRef<[Vec<u32>]>) {
    let total = turn_on_batteries(battery_banks, 2);

    assert!(total == 17324);
    println!("Total: {}", total);
}

fn part2(battery_banks: impl AsRef<[Vec<u32>]>) {
    let total = turn_on_batteries(battery_banks, 12);

    assert!(total == 171846613143331);
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
