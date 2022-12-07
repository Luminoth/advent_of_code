use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::{all_consuming, map},
    sequence::{preceded, separated_pair},
    Finish, IResult,
};

#[derive(Debug, PartialEq, Eq)]
enum NodeType {
    Directory,
    File,
}

// Node handle idea taken from https://fasterthanli.me/series/advent-of-code-2022/part-7
type NodeHandle = Rc<RefCell<Node>>;

#[derive(Debug)]
struct Node {
    r#type: NodeType,

    #[allow(unused)]
    name: String,

    // the root doesn't have a parent
    parent: Option<NodeHandle>,

    // only files have a size
    size: Option<u64>,

    // only directories have children
    children: Option<HashMap<String, NodeHandle>>,
}

impl Node {
    fn new_directory(name: impl Into<String>, parent: Option<NodeHandle>) -> NodeHandle {
        Rc::new(RefCell::new(Self {
            r#type: NodeType::Directory,
            name: name.into(),
            parent,
            size: None,
            children: Some(HashMap::new()),
        }))
    }

    fn new_file(name: impl Into<String>, size: u64, parent: NodeHandle) -> NodeHandle {
        Rc::new(RefCell::new(Self {
            r#type: NodeType::File,
            name: name.into(),
            parent: Some(parent),
            size: Some(size),
            children: None,
        }))
    }

    // TODO: we should cache this for directories
    fn size(&self) -> u64 {
        match self.r#type {
            NodeType::Directory => {
                let mut size = 0;
                for (_, child) in self.children.clone().unwrap() {
                    size += child.borrow().size();
                }
                size
            }
            NodeType::File => self.size.unwrap(),
        }
    }
}

// NOTE: nom code taken from https://fasterthanli.me/series/advent-of-code-2022/part-7
// this is my first experience using nom and it is really, really nice!

fn parse_path(input: &str) -> IResult<&str, String> {
    map(
        take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
        Into::into,
    )(input)
}

fn parse_ls(input: &str) -> IResult<&str, &str> {
    tag("ls")(input)
}

#[derive(Debug)]
struct Cd(String);

fn parse_cd(input: &str) -> IResult<&str, Cd> {
    map(preceded(tag("cd "), parse_path), Cd)(input)
}

#[derive(Debug)]
enum Command {
    Ls,
    Cd(String),
}

impl From<Cd> for Command {
    fn from(cd: Cd) -> Self {
        Command::Cd(cd.0)
    }
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("$ ")(input)?;
    alt((map(parse_ls, |_| Command::Ls), map(parse_cd, Into::into)))(input)
}

#[derive(Debug)]
enum Entry {
    Directory(String),
    File(u64, String),
}

fn parse_entry(input: &str) -> IResult<&str, Entry> {
    let parse_file = map(
        separated_pair(nom::character::complete::u64, tag(" "), parse_path),
        |(size, path)| Entry::File(size, path),
    );
    let parse_dir = map(preceded(tag("dir "), parse_path), Entry::Directory);

    alt((parse_file, parse_dir))(input)
}

#[derive(Debug)]
enum Line {
    Command(Command),
    Entry(Entry),
}

fn parse_line(i: &str) -> IResult<&str, Line> {
    alt((
        map(parse_command, Line::Command),
        map(parse_entry, Line::Entry),
    ))(i)
}

fn get_dir_sizes(node: NodeHandle, total: &mut u64, max_size: u64) {
    for child in node.borrow().children.as_ref().unwrap().values() {
        let node = child.borrow();
        if node.r#type == NodeType::Directory {
            let size = node.size();
            if size <= max_size {
                *total += size;
            }

            get_dir_sizes(child.clone(), total, max_size);
        }
    }
}

fn part1(root: NodeHandle) {
    let mut total = 0;
    get_dir_sizes(root, &mut total, 100000);

    assert!(total == 1232307);
    println!("Total size: {}", total);
}

fn get_smallest_dir_size(node: NodeHandle, current_size: &mut u64, min_size: u64) {
    for child in node.borrow().children.as_ref().unwrap().values() {
        let node = child.borrow();
        if node.r#type == NodeType::Directory {
            let size = node.size();
            if size >= min_size && size < *current_size {
                *current_size = size;
            }

            get_smallest_dir_size(child.clone(), current_size, min_size);
        }
    }
}

fn part2(root: NodeHandle) {
    let mut current_size = root.borrow().size();
    let required_space = 30000000 - (70000000 - current_size);
    println!("Need to free {} space", required_space);

    get_smallest_dir_size(root, &mut current_size, required_space);

    assert!(current_size == 7268994);
    println!("Min dir size: {}", current_size);
}

fn main() {
    let input = include_str!("../input.txt");

    let values = input
        .lines()
        .map(|x| all_consuming(parse_line)(x).finish().unwrap().1);

    let root = Node::new_directory("/", None);
    let mut pwd: Option<NodeHandle> = None;

    for value in values {
        match value {
            Line::Command(command) => match command {
                Command::Ls => (),
                Command::Cd(dir) => {
                    let node = pwd.clone();
                    if dir == ".." {
                        pwd = node.unwrap().borrow().parent.clone();
                    } else {
                        match node {
                            Some(node) => {
                                let node = node.borrow();

                                let children = node.children.as_ref().unwrap();
                                pwd = Some(children.get(&dir).unwrap().clone());
                            }
                            None => {
                                assert!(dir == "/");
                                pwd = Some(root.clone());
                            }
                        }
                    }
                }
            },
            Line::Entry(entry) => match entry {
                Entry::Directory(name) => {
                    let node = pwd.clone().unwrap();
                    let mut node = node.borrow_mut();

                    let children = node.children.as_mut().unwrap();
                    if !children.contains_key(&name) {
                        children
                            .insert(name.clone(), Node::new_directory(name.clone(), pwd.clone()));
                    }
                }
                Entry::File(size, name) => {
                    let node = pwd.clone().unwrap();
                    let mut node = node.borrow_mut();

                    let children = node.children.as_mut().unwrap();
                    if !children.contains_key(&name) {
                        children.insert(
                            name.clone(),
                            Node::new_file(name.clone(), size, pwd.clone().unwrap()),
                        );
                    }
                }
            },
        }
    }

    part1(root.clone());
    part2(root);
}
