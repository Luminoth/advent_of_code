fn most_common_value(values: impl AsRef<[String]>, idx: usize) -> usize {
    let v: isize = values
        .as_ref()
        .iter()
        .map(|x| {
            if x.chars().nth(idx).unwrap() == '0' {
                -1
            } else {
                1
            }
        })
        .sum();

    if v < 0 {
        0
    } else {
        1
    }
}

fn part1(values: impl AsRef<[String]>) {
    // assume all the lines are the same length
    let bitcount = values.as_ref()[0].len();

    let mut v = 0;
    for i in 0..bitcount {
        let c = most_common_value(&values, i);

        let mask = c << (bitcount - 1 - i);
        v |= mask;
    }
    assert!(v == 779);

    let mut nv = v;
    for i in 0..bitcount {
        let mask = 1 << i;
        nv ^= mask;
    }
    assert!(nv == 3316);

    println!("{} * {} = {}", v, nv, v * nv);
}

fn part2(values: &[String]) {
    let mut oxygen = values.to_owned();
    let mut co2 = values.to_owned();

    let mut i = 0;
    loop {
        if oxygen.len() == 1 && co2.len() == 1 {
            break;
        }

        if oxygen.len() > 1 {
            let c = most_common_value(&oxygen, i);
            oxygen.retain(|x| {
                let ch = x.chars().nth(i).unwrap();
                if c == 0 {
                    ch == '0'
                } else {
                    ch == '1'
                }
            });
        }

        if co2.len() > 1 {
            let c = most_common_value(&co2, i);
            co2.retain(|x| {
                let ch = x.chars().nth(i).unwrap();
                if c == 0 {
                    ch == '1'
                } else {
                    ch == '0'
                }
            });
        }

        i += 1;
    }

    let oxygen = usize::from_str_radix(&oxygen[0], 2).unwrap();
    assert!(oxygen == 825);

    let co2 = usize::from_str_radix(&co2[0], 2).unwrap();
    assert!(co2 == 3375);

    println!("{} * {} = {}", oxygen, co2, oxygen * co2);
}

fn main() {
    let input = include_str!("../input.txt");

    let values: Vec<String> = input
        .lines()
        .filter_map(|x| {
            let x = x.trim();
            if x.is_empty() {
                return None;
            }

            Some(x.to_string())
        })
        .collect();

    part1(&values);
    part2(&values);
}
