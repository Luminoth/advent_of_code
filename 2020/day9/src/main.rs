use std::collections::HashSet;

const PREAMBLE_LEN: usize = 25;

fn is_valid(preamble: impl AsRef<[usize]>, value: usize) -> bool {
    let preamble = preamble.as_ref();
    if preamble.len() != PREAMBLE_LEN {
        panic!("invalid preamble");
    }

    let mut sums = HashSet::new();
    for x in 0..PREAMBLE_LEN {
        let xv = preamble.get(x).unwrap();
        for y in x + 1..PREAMBLE_LEN {
            let yv = preamble.get(y).unwrap();
            sums.insert(xv + yv);
        }
    }

    sums.contains(&value)
}

fn main() {
    let input = include_str!("../input.txt");

    let lines: Vec<&str> = input.lines().filter(|x| !x.is_empty()).collect();

    let values: Vec<usize> = lines.iter().map(|x| x.parse().unwrap()).collect();

    let mut invalid = None;
    for x in PREAMBLE_LEN..values.len() {
        let slice_start = x - PREAMBLE_LEN;
        let value = values.get(x).unwrap();
        if !is_valid(&values[slice_start..slice_start + PREAMBLE_LEN], *value) {
            invalid = Some(value);
            break;
        }
    }

    if invalid.is_none() {
        panic!("failed to find the invalid number");
    }

    let invalid = *(invalid.unwrap());
    println!("{} is not valid", invalid);

    for x in 0..values.len() {
        let xv = values.get(x).unwrap();

        let mut sum = *xv;
        for y in x + 1..values.len() {
            let yv = values.get(y).unwrap();

            sum += yv;
            match sum.cmp(&invalid) {
                std::cmp::Ordering::Equal => {
                    let v = &values[x..y + 1];
                    let min = v.iter().min().unwrap();
                    let max = v.iter().max().unwrap();

                    println!("weakness: {} + {} = {}", min, max, min + max);
                    return;
                }
                std::cmp::Ordering::Greater => break,
                _ => continue,
            }
        }
    }

    panic!("didn't find the weakness!");
}
