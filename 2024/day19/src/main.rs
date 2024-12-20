use std::collections::BTreeMap;

#[derive(Debug)]
struct TrieNode {
    children: BTreeMap<char, Option<TrieNode>>,
    value: Option<String>,
    terminal: bool,
}

impl Default for TrieNode {
    fn default() -> Self {
        Self {
            children: BTreeMap::from([
                ('w', None),
                ('u', None),
                ('b', None),
                ('r', None),
                ('g', None),
            ]),
            value: None,
            terminal: false,
        }
    }
}

#[derive(Debug, Default)]
struct Trie {
    root: TrieNode,
}

impl Trie {
    fn insert(&mut self, value: String) {
        let mut c = &mut self.root;
        for i in value.chars() {
            if c.children[&i].is_none() {
                *c.children.get_mut(&i).unwrap() = Some(TrieNode::default());
            }
            c = c.children.get_mut(&i).unwrap().as_mut().unwrap();
        }

        c.value = Some(value);
        c.terminal = true;
    }

    // TODO: this is the basic search
    // but we probably want to length limit and
    // allow wrapping back to the root?
    /*fn search(&self, key: &str) -> bool {
        let mut c = &self.root;
        for i in key.chars() {
            if c.children[&i].is_none() {
                return false;
            }
            c = c.children.get(&i).unwrap().as_ref().unwrap();
        }
        c.terminal
    }*/
}

fn part1(patterns: &Trie, designs: &[&str]) {
    println!("{:?}", patterns);
    println!("{:?}", designs);

    for _design in designs {
        // TODO: do we traverse the Trie up len(design)
        // allowing it to wrap if we terminal but aren't at the length?
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let (patterns, designs) = input.split_once("\n\n").unwrap();

    let mut patterns_trie = Trie::default();
    patterns
        .split(", ")
        .for_each(|pattern| patterns_trie.insert(pattern.to_owned()));

    let designs = designs.lines().collect::<Vec<_>>();

    part1(&patterns_trie, &designs);
}
