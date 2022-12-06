use itertools::Itertools;

// NOTE: all_unique() builds a HashSet under the hood so this solution is not allocation-free

fn part1(input: impl AsRef<str>) {
    let mut n = 0;
    for x in input.as_ref().as_bytes().windows(4) {
        if x.iter().all_unique() {
            break;
        }
        n += 1;
    }
    n += 4;

    assert!(n == 1912);
    println!("Packet start: {}", n);
}

fn part2(input: impl AsRef<str>) {
    let mut n = 0;
    for x in input.as_ref().as_bytes().windows(14) {
        if x.iter().all_unique() {
            break;
        }
        n += 1;
    }
    n += 14;

    assert!(n == 2122);
    println!("Message start: {}", n);
}

fn main() {
    let input = include_str!("../input.txt");

    let input = input.trim();

    part1(input);
    part2(input);
}
