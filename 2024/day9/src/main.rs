fn checksum(disk: &Vec<Option<usize>>) -> usize {
    let mut checksum = 0;
    for (idx, v) in disk.iter().enumerate() {
        match v {
            Some(id) => {
                //print!("{}", id);
                checksum += id * idx;
            }
            None => {
                //print!(".");
                ()
            }
        }
    }
    //println!();

    checksum
}

fn part1(mut disk: Vec<Option<usize>>) {
    // head starts at the first empty block
    let mut head = 0;
    while disk[head].is_some() {
        head += 1;
    }

    // tail starts at the last non-empty block
    let mut tail = disk.len() - 1;
    while disk[tail].is_none() {
        tail -= 1;
    }

    loop {
        if head >= tail {
            break;
        }

        // move the tail to the head
        disk[head] = disk[tail];
        disk[tail] = None;

        // update the head / tail
        while disk[head].is_some() {
            head += 1;
        }
        while disk[tail].is_none() {
            tail -= 1;
        }
    }

    let total = checksum(&disk);
    assert!(total == 6331212425418);
    println!("Part 1: {}", total);
}

fn part2(disk: Vec<Option<usize>>) {
    // TODO:

    let total = checksum(&disk);
    //assert!(total == ???);
    println!("Part 2: {}", total);
}

fn main() {
    let input = include_str!("../input.txt");

    let diskmap = input
        .trim_ascii()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();

    let mut disk = Vec::with_capacity(diskmap.iter().sum());

    let mut id = 0;
    for v in diskmap.chunks(2) {
        disk.extend(vec![Some(id); v[0]]);
        if v.len() > 1 {
            disk.extend(vec![None; v[1]]);
        }

        id += 1;
    }
    checksum(&disk);

    part1(disk.clone());
    part2(disk);
}
