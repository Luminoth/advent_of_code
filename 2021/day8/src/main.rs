use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Entry {
    patterns: Vec<&'static str>,
    output: Vec<&'static str>,
}

fn part1(entries: impl AsRef<[Entry]>) {
    let count: usize = entries
        .as_ref()
        .iter()
        .map(|x| {
            x.output
                .iter()
                .filter(|y| {
                    let len = y.len();
                    len == 2 || len == 4 || len == 3 || len == 7
                })
                .count()
        })
        .sum();

    assert!(count == 272);
    println!("Total: {}", count);
}

/*
  0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg

*/

// had to take mad help from https://twitter.com/TartanLlama on this one
// https://www.twitch.tv/videos/1227489077
// https://github.com/zertosh also has an interesting solution
// that involves generating all possible permutations as a lookup table
fn part2(entries: impl AsRef<[Entry]>) {
    let mut total = 0;
    for entry in entries.as_ref() {
        let mut mapping = HashMap::new();

        // first pass to map the known unique segments
        // we need to determine the overlapping segment values
        for pattern in &entry.patterns {
            match pattern.len() {
                2 => mapping.insert('1', pattern.to_string()),
                4 => mapping.insert('4', pattern.to_string()),
                3 => mapping.insert('7', pattern.to_string()),
                _ => None,
            };
        }

        // second pass to map almost everything else
        for pattern in &entry.patterns {
            match pattern.len() {
                5 => {
                    // 3 contains 7
                    if mapping[&'7'].chars().all(|ch| pattern.contains(ch)) {
                        mapping.insert('3', pattern.to_string());
                    }
                    // cannot deduce 2 or 5 yet
                }
                6 => {
                    // 6 does not contain 1
                    if !mapping[&'1'].chars().all(|ch| pattern.contains(ch)) {
                        mapping.insert('6', pattern.to_string());
                    }
                    // 9 contains 4
                    else if mapping[&'4'].chars().all(|ch| pattern.contains(ch)) {
                        mapping.insert('9', pattern.to_string());
                    }
                    // must be a 0
                    else {
                        mapping.insert('0', pattern.to_string());
                    }
                }
                _ => (),
            }
        }

        // one last pass to differentiate 2 and 5
        for pattern in &entry.patterns {
            if pattern.len() == 5 {
                // skip 3s
                if mapping[&'7'].chars().all(|ch| pattern.contains(ch)) {
                    continue;
                }
                // 5 is a subset of 6
                else if pattern.chars().all(|ch| mapping[&'6'].contains(ch)) {
                    mapping.insert('5', pattern.to_string());
                }
                // must be a 2
                else {
                    mapping.insert('2', pattern.to_string());
                }
            }
        }

        // inverse the mappings for lookup
        let mapping: HashMap<String, char> = mapping
            .iter()
            .map(|(k, v)| {
                // sort the key for lookup
                let mut x: Vec<char> = v.chars().collect();
                x.sort();
                (x.iter().collect(), *k)
            })
            .collect();

        // remap the outputs to their values
        let mut value = String::with_capacity(4);
        for output in &entry.output {
            match output.len() {
                2 => value.push('1'),
                3 => value.push('7'),
                4 => value.push('4'),
                5 | 6 => {
                    // sort the key for lookup
                    let mut x: Vec<char> = output.chars().collect();
                    x.sort();
                    let v: String = x.iter().collect();

                    value.push(mapping[&v]);
                }
                7 => value.push('8'),
                _ => panic!("invalid output: {}", output),
            }
        }

        let value: usize = value.parse().unwrap();
        total += value;
    }

    println!("Total: {}", total);
}

fn main() {
    let input = include_str!("../input.txt");

    let entries: Vec<Entry> = input
        .lines()
        .filter_map(|x| {
            let x = x.trim();
            if x.is_empty() {
                return None;
            }

            let v = x.split_once('|').unwrap();

            let patterns: Vec<&str> =
                v.0.split(' ')
                    .filter_map(|x| {
                        let x = x.trim();
                        if x.is_empty() {
                            return None;
                        }

                        Some(x)
                    })
                    .collect();
            assert!(patterns.len() == 10);

            let output: Vec<&str> =
                v.1.split(' ')
                    .filter_map(|x| {
                        let x = x.trim();
                        if x.is_empty() {
                            return None;
                        }

                        Some(x)
                    })
                    .collect();
            assert!(output.len() == 4);

            Some(Entry { patterns, output })
        })
        .collect();

    part1(&entries);
    part2(entries);
}
