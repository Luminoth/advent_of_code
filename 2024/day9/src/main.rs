fn checksum_disk(disk: &[Option<usize>]) -> usize {
    let mut checksum = 0;
    for (idx, v) in disk.iter().enumerate() {
        match v {
            Some(id) => {
                //print!("{}", id);
                checksum += id * idx;
            }
            None => {
                //print!(".");
            }
        }
    }
    //println!();

    checksum
}

fn checksum_diskmap(_diskmap: &[usize]) -> usize {
    0
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

    let total = checksum_disk(&disk);
    assert!(total == 6331212425418);
    println!("Part 1: {}", total);
}

fn part2(diskmap: Vec<usize>) {
    // TODO: we should be able to use the diskmap here to find the free slots?
    // it will have to be updated tho as we move things around
    // and the checksum I think can be calculated from that

    let total = checksum_diskmap(&diskmap);
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
    checksum_diskmap(&diskmap);

    let mut disk = Vec::with_capacity(diskmap.iter().sum());
    for (id, v) in diskmap.chunks(2).enumerate() {
        disk.extend(vec![Some(id); v[0]]);
        if v.len() > 1 {
            disk.extend(vec![None; v[1]]);
        }
    }
    checksum_disk(&disk);

    part1(disk);
    part2(diskmap);
}
