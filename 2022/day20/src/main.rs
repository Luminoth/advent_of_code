use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

// had to grab help from https://www.reddit.com/r/adventofcode/
// on this one to realize the number has to be removed from the list
// before finding the destination location

const DECRYPTION_KEY: i64 = 811589153;

#[derive(Debug)]
struct Number {
    n: i64,

    left: Option<NumberHandle>,
    right: Option<NumberHandle>,
}

type NumberHandle = Rc<RefCell<Number>>;

impl From<i64> for Number {
    fn from(n: i64) -> Self {
        Self {
            n,
            left: None,
            right: None,
        }
    }
}

// this here is some janky linked list garbage
// using indexes here rather than handles
// would probably be a decent speedup
// (no need to traverse the list anywhere)

fn move_left(n: NumberHandle, len: i64) {
    // first, remove the number from it's current position
    // so we don't count it as "in the list" when moving
    let right = n.borrow_mut().right.take().unwrap();
    let left = n.borrow_mut().left.take().unwrap();
    right.borrow_mut().left = Some(left.clone());
    left.borrow_mut().right = Some(right);

    let move_by = (n.borrow().n % (len - 1)).abs();
    let dst = {
        let mut c = left;
        for _ in 1..move_by {
            c = {
                let v = c.borrow();
                v.left.clone().unwrap()
            };
        }
        c
    };

    #[cfg(feature = "debugvis")]
    println!(
        "{} moves left between {} and {}:",
        n.borrow().n,
        dst.borrow().left.as_ref().unwrap().borrow().n,
        dst.borrow().n,
    );

    // insert the number in its new position
    n.borrow_mut().left = dst.borrow_mut().left.take();
    n.borrow().left.as_ref().unwrap().borrow_mut().right = Some(n.clone());
    dst.borrow_mut().left = Some(n.clone());
    n.borrow_mut().right = Some(dst);
}

fn move_right(n: NumberHandle, len: i64) {
    // first, remove the number from it's current position
    // so we don't count it as "in the list" when moving
    let left = n.borrow_mut().left.take().unwrap();
    let right = n.borrow_mut().right.take().unwrap();
    left.borrow_mut().right = Some(right.clone());
    right.borrow_mut().left = Some(left);

    let move_by = n.borrow().n % (len - 1);
    let dst = {
        let mut c = right;
        for _ in 1..move_by {
            c = {
                let v = c.borrow();
                v.right.clone().unwrap()
            };
        }
        c
    };

    #[cfg(feature = "debugvis")]
    println!(
        "{} moves right between {} and {}:",
        n.borrow().n,
        dst.borrow().n,
        dst.borrow().right.as_ref().unwrap().borrow().n,
    );

    // insert the number in its new position
    n.borrow_mut().right = dst.borrow_mut().right.take();
    n.borrow().right.as_ref().unwrap().borrow_mut().left = Some(n.clone());
    dst.borrow_mut().right = Some(n.clone());
    n.borrow_mut().left = Some(dst);
}

fn r#move(n: NumberHandle, len: usize) {
    let move_by = n.borrow().n;
    match move_by.cmp(&0) {
        Ordering::Greater => move_right(n, len as i64),
        Ordering::Less => move_left(n, len as i64),
        Ordering::Equal => {
            #[cfg(feature = "debugvis")]
            println!("{} does not move:", n.borrow().n);
        }
    }
}

#[cfg(feature = "debugvis")]
fn print_values(values: impl AsRef<[NumberHandle]>) {
    let mut i = 0;
    let mut c = values.as_ref().first().unwrap().clone();
    loop {
        if i >= values.as_ref().len() {
            break;
        }

        assert!(c.borrow().left.is_some());

        c = {
            let v = c.borrow();

            #[cfg(feature = "debugvis")]
            print!("{}, ", v.n);

            v.right.clone().unwrap()
        };

        i += 1;
    }
    println!();
}

fn decrypt(values: impl AsRef<[i64]>, rounds: usize) -> (usize, i64, i64, i64) {
    let values = values
        .as_ref()
        .iter()
        .map(|x| Rc::new(RefCell::new(Number::from(*x))))
        .collect::<Vec<_>>();

    // create the internal list
    for value in values.windows(2) {
        value[0].borrow_mut().right = Some(value[1].clone());
        value[1].borrow_mut().left = Some(value[0].clone());
    }
    values.first().unwrap().borrow_mut().left = Some(values.last().unwrap().clone());
    values.last().unwrap().borrow_mut().right = Some(values.first().unwrap().clone());

    #[cfg(feature = "debugvis")]
    {
        println!("Initial arrangement ({}):", values.len());
        print_values(&values);
    }

    for _ in 0..rounds {
        for value in &values {
            #[cfg(feature = "debugvis")]
            println!();

            r#move(value.clone(), values.len());

            #[cfg(feature = "debugvis")]
            print_values(&values);
        }
    }

    let zidx = {
        let mut n = 0;
        for (i, v) in values.iter().enumerate() {
            if v.borrow().n == 0 {
                n = i;
                break;
            }
        }
        n
    };

    let aidx = 1000 % values.len();
    let a = {
        let mut n = values.get(zidx).unwrap().clone();
        let mut v = None;
        for _ in 0..aidx {
            n = {
                let v = n.borrow();
                v.right.clone().unwrap()
            };
            v = Some(n.borrow().n);
        }
        v.unwrap()
    };

    let bidx = 2000 % values.len();
    let b = {
        let mut n = values.get(zidx).unwrap().clone();
        let mut v = None;
        for _ in 0..bidx {
            n = {
                let v = n.borrow();
                v.right.clone().unwrap()
            };
            v = Some(n.borrow().n);
        }
        v.unwrap()
    };

    let cidx = 3000 % values.len();
    let c = {
        let mut n = values.get(zidx).unwrap().clone();
        let mut v = None;
        for _ in 0..cidx {
            n = {
                let v = n.borrow();
                v.right.clone().unwrap()
            };
            v = Some(n.borrow().n);
        }
        v.unwrap()
    };

    #[cfg(feature = "debugvis")]
    println!();

    (zidx, a, b, c)
}

fn part1(values: impl AsRef<[i64]>) {
    let (zidx, a, b, c) = decrypt(values, 1);

    let total = a + b + c;
    assert!(total == 5962);
    println!(
        "Grove coordinates from {} ({}, {}, {}): {}",
        zidx, a, b, c, total
    );
}

fn part2(values: impl AsRef<[i64]>) {
    let values = values
        .as_ref()
        .iter()
        .map(|x| x * DECRYPTION_KEY)
        .collect::<Vec<_>>();

    let (zidx, a, b, c) = decrypt(values, 10);

    let total = a + b + c;
    assert!(total == 9862431387256);
    println!(
        "Grove coordinates from {} ({}, {}, {}): {}",
        zidx, a, b, c, total
    );
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

            Some(x.parse().unwrap())
        })
        .collect::<Vec<_>>();

    part1(&values);
    part2(values);
}
