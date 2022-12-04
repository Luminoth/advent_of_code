// TODO: there's probably a math way to do this to avoid the branching
#[inline]
fn priority(a: char) -> usize {
    if a.is_uppercase() {
        a as usize - 64 + 26
    } else {
        a as usize - 96
    }
}

fn part1(values: impl AsRef<[&'static str]>) {
    let values: Vec<(&str, &str)> = values
        .as_ref()
        .iter()
        .map(|x| {
            let len = x.len();
            assert!(len % 2 == 0);

            x.split_at(x.len() / 2)
        })
        .collect();

    let mut total = 0;
    for value in values {
        let mut bitset = 0_u64;
        for a in value.0.chars() {
            let bit = priority(a) - 1;
            bitset |= 1 << bit;
        }

        for b in value.1.chars() {
            let bit = priority(b) - 1;
            let n = 1 << bit;
            if (bitset & n) == n {
                total += bit + 1;
                break;
            }
        }
    }

    assert!(total == 7821);
    println!("Total priority: {}", total);
}

fn part2(values: impl AsRef<[&'static str]>) {
    let mut total = 0;
    for group in values.as_ref().chunks(3) {
        let mut bitset_a = 0_u64;
        for a in group[0].chars() {
            let bit = priority(a) - 1;
            bitset_a |= 1 << bit;
        }

        let mut bitset_b = 0_u64;
        for b in group[1].chars() {
            let bit = priority(b) - 1;
            bitset_b |= 1 << bit;
        }

        for c in group[2].chars() {
            let bit = priority(c) - 1;
            let n = 1 << bit;
            if (bitset_a & n) == n && (bitset_b & n) == n {
                total += bit + 1;
                break;
            }
        }
    }

    assert!(total == 2752);
    println!("Total priority: {}", total);
}

fn main() {
    let input = include_str!("../input.txt");

    let values = input
        .lines()
        .filter_map(|x| {
            let x = x.trim();
            if x.is_empty() {
                return None;
            }

            Some(x)
        })
        .collect::<Vec<_>>();

    part1(&values);
    part2(&values);
}
