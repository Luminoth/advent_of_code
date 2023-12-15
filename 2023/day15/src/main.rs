fn hash(v: &str) -> usize {
    let mut h = 0;
    for ch in v.chars() {
        h += ch as usize;
        h *= 17;
        h %= 256;
    }
    h
}

fn part1(sequence: &[&str]) {
    let mut total = 0;

    for step in sequence {
        let h = hash(step);
        //println!("{}", h);
        total += h;
    }

    assert!(total == 510801);
    println!("Total: {}", total);
}

fn part2(sequence: &[&str]) {
    let mut boxes = Vec::with_capacity(256);
    for _ in 0..boxes.capacity() {
        boxes.push(vec![]);
    }

    for step in sequence {
        if step.ends_with('-') {
            let label = &step[0..step.len() - 1];
            let r#box = hash(label);

            //println!("{} is in box {}", label, r#box);

            let r#box = &mut boxes[r#box];
            if let Some(idx) = r#box.iter().position(|(x, _)| *x == label) {
                // can't remove_swap since we need to maintain ordering
                r#box.remove(idx);
            }
        } else {
            let (label, focal_len) = step.split_once('=').unwrap();
            let focal_len = focal_len.parse::<usize>().unwrap();
            let r#box = hash(label);

            //println!("{} ({}) is in box {}", label, focal_len, r#box);

            let r#box = &mut boxes[r#box];
            if let Some(idx) = r#box.iter().position(|(x, _)| *x == label) {
                r#box.get_mut(idx).unwrap().1 = focal_len;
            } else {
                r#box.push((label, focal_len));
            }
        }
    }

    let mut total = 0;

    for (r#box, lenses) in boxes.iter().enumerate() {
        //println!("{} = {:?}", r#box, lenses);

        for (idx, (_label, focal_len)) in lenses.iter().enumerate() {
            let power = (1 + r#box) * (idx + 1) * focal_len;
            /*println!(
                "{} ({}) is in box {} with power {}",
                _label, focal_len, r#box, power
            );*/
            total += power;
        }
    }

    assert!(total == 212763);
    println!("Total: {}", total);
}

fn main() {
    let mut input = include_str!("../input.txt").to_string();
    input.retain(|c| !c.is_ascii_whitespace());

    let sequence = input.split(',').collect::<Vec<_>>();

    part1(&sequence);
    part2(&sequence);
}
