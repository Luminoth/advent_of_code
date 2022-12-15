fn most_common_values(values: impl AsRef<[String]>) -> Vec<usize> {
    // assume all the values are the same length
    let bitcount = values.as_ref()[0].len();

    let mut x = vec![0_isize; bitcount];
    for value in values.as_ref() {
        for (i, c) in value.chars().enumerate() {
            x[i] += if c == '0' { -1 } else { 1 };
        }
    }

    // convert the result to 0 / 1
    x.iter().map(|&v| usize::from(v >= 0)).collect()
}

fn part1(values: impl AsRef<[String]>) {
    let m = most_common_values(values);

    let mut v = 0;
    for (i, x) in m.iter().enumerate() {
        let mask = x << (m.len() - 1 - i);
        v |= mask;
    }
    assert!(v == 779);

    let mut nv = v;
    for i in 0..m.len() {
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
            let m = most_common_values(&oxygen);
            oxygen.retain(|x| {
                let ch = x.chars().nth(i).unwrap();
                if m[i] == 0 {
                    ch == '0'
                } else {
                    ch == '1'
                }
            });
        }

        if co2.len() > 1 {
            let m = most_common_values(&co2);
            co2.retain(|x| {
                let ch = x.chars().nth(i).unwrap();
                if m[i] == 0 {
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
