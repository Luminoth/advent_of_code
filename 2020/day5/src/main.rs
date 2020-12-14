fn narrow(input: char, min: i64, max: i64) -> (i64, i64) {
    let mid = (max - min) / 2;

    match input {
        'F' | 'L' => (min, min + mid),
        'B' | 'R' => (min + mid + 1, max),
        _ => panic!("Invalid boarding pass!"),
    }
}

fn seatid(boarding_pass: impl AsRef<str>) -> i64 {
    let mut chars = boarding_pass.as_ref().chars();

    // narrow the rows
    let mut rowmin = 0;
    let mut rowmax = 127;
    for _ in 0..7 {
        let c = chars.next().unwrap();
        let (s, t) = narrow(c, rowmin, rowmax);
        rowmin = s;
        rowmax = t;
    }
    assert_eq!(rowmin, rowmax, "row");
    let row = rowmin;

    // narrow the seats
    let mut seatmin = 0;
    let mut seatmax = 7;
    for _ in 0..3 {
        let c = chars.next().unwrap();
        let (s, t) = narrow(c, seatmin, seatmax);
        seatmin = s;
        seatmax = t;
    }
    assert_eq!(seatmin, seatmax, "seat");
    let seat = seatmin;

    row * 8 + seat
}

// assumes seatids is sorted
fn find_missing(seatids: impl AsRef<[i64]>) -> i64 {
    let seatids = seatids.as_ref();

    println!("checking {} seatids", seatids.len());

    let mut last_seat_id = seatids.get(0).unwrap() - 1;
    for seatid in seatids {
        let expected = last_seat_id + 1;
        if *seatid != expected {
            return expected;
        }
        last_seat_id = *seatid;
    }

    panic!("No seat id!");
}

fn main() {
    let input = include_str!("../input.txt");

    let lines: Vec<&str> = input.lines().filter(|x| !x.is_empty()).collect();

    let mut seatids = lines.iter().map(seatid).collect::<Vec<i64>>();
    seatids.sort_unstable();

    println!("highest seat id: {}", seatids.iter().last().unwrap());

    let seatid = find_missing(&seatids);
    println!("your seat id: {}", seatid);
}
