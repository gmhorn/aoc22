use anyhow::{anyhow, Result};
use aoc22::Timer;
use std::{
    collections::{hash_map::Entry, HashMap},
    rc::Rc,
};

// https://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/

fn main() -> Result<()> {
    let timer = Timer::tick();
    let mut _lines = include_str!("../../data/day07.txt").lines();

    let mut root = FolderData::root();
    root.add("foo", Node::File(1234));
    root.add("bar", Node::File(6));

    Rc::new(value)
    let baz = Node::Folder(FolderData::new(&root));
    root.add("baz", baz);

    println!("{}", root.size());
    timer.tock();
    Ok(())
}

pub type FSIndex = usize;

pub struct FileSystem {
    nodes: Vec<Node>,
}

pub enum Node {
    File(usize),
    Folder(Rc<FolderData>),
}

impl Node {
    pub fn size(&self) -> usize {
        match self {
            Node::File(size) => *size,
            Node::Folder(data) => data.size(),
        }
    }
}

pub struct FolderData {
    parent: Option<Rc<FolderData>>,
    children: HashMap<String, Node>,
}

impl FolderData {
    pub fn root() -> Self {
        Self {
            parent: None,
            children: HashMap::new(),
        }
    }

    pub fn new(parent: &Rc<FolderData>) -> Self {
        Self {
            parent: Some(Rc::clone(parent)),
            children: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: impl Into<String>, child: Node) -> Result<()> {
        match self.children.entry(name.into()) {
            Entry::Occupied(o) => {
                Err(anyhow!("duplicate entry '{}", o.key()))
            },
            Entry::Vacant(v) => {
                v.insert(child);
                Ok(())
            }
        }
    }

    pub fn size(&self) -> usize {
        self.children
            .values()
            .fold(0, |acc, child| acc + child.size())
    }
}

// struct FolderData {
//     name: String,
//     children: Vec<Node>,
// }

// impl FolderData {
//     pub fn new(name: impl Into<String>) -> Self {
//         Self {
//             name: name.into(),
//             children: vec![],
//         }
//     }

//     pub fn add(&mut self, child: Node) {
//         self.children.push(child)
//     }
// }

// enum Node {
//     File(FileData),
//     Folder(FolderData),
// }

// impl FromStr for Node {
//     type Err = Error;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let (prefix, name) = s
//             .split_once(' ')
//             .context(format!("'{}' not a valid filesystem node", s))?;

//         if prefix == "dir" {
//             Ok(Node::Folder(FolderData::new(name)))
//         } else {
//             let size: usize = prefix
//                 .parse()
//                 .context(format!("unknown prefix '{}'", prefix))?;
//             Ok(Node::File(FileData::new(name, size)))
//         }
//     }
// }
