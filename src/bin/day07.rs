use crate::fs::FileSystem;
use anyhow::Result;
use aoc22::Timer;

// https://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/

fn main() -> Result<()> {
    let timer = Timer::tick();
    let mut _lines = include_str!("../../data/day07.txt").lines();

    let mut filesystem = FileSystem::new();
    let mut cur = filesystem.root_mut();
    cur.add_file("file1.txt", 1234).expect("can add");
    cur.add_file("file2.txt", 5678).expect("can add");
    cur.add_folder("sub1").expect("can add");

    cur.cd("sub1").expect("can cd");
    println!("{:?}", filesystem);

    timer.tock();
    Ok(())
}

mod fs {
    pub use anyhow::Result;
    use anyhow::{anyhow, Context};
    use std::collections::hash_map::Entry;
    use std::collections::HashMap;

    #[derive(Debug, Clone, Copy)]
    pub struct NodeIdx(usize);

    #[derive(Debug)]
    pub struct FileSystem(Vec<Node>);

    impl FileSystem {
        pub fn new() -> Self {
            let root = FolderData {
                parent: None,
                children: HashMap::new(),
            };
            Self(vec![Node::Folder(root)])
        }

        pub fn root(&self) -> CurrentDir {
            CurrentDir {
                fs: self,
                node: NodeIdx(0),
            }
        }

        pub fn root_mut(&mut self) -> CurrentDirMut {
            CurrentDirMut {
                fs: self,
                idx: NodeIdx(0),
            }
        }
    }

    #[derive(Debug)]
    pub enum Node {
        File(usize),
        Folder(FolderData),
    }

    #[derive(Debug)]
    pub struct FolderData {
        parent: Option<NodeIdx>,
        children: HashMap<String, NodeIdx>,
    }

    pub struct CurrentDir<'fs> {
        fs: &'fs FileSystem,
        node: NodeIdx,
    }

    pub struct CurrentDirMut<'fs> {
        fs: &'fs mut FileSystem,
        idx: NodeIdx,
    }

    impl<'fs> CurrentDirMut<'fs> {
        fn add_child(&mut self, name: impl Into<String>, child: Node) -> Result<()> {
            let child_idx = NodeIdx(self.fs.0.len());
            let parent_node = self.fs.0.get_mut(self.idx.0)
                .context(format!("Could not find current dir in file system (idx: {})", self.idx.0))?;
            if let Node::Folder(parent_data) = parent_node {
                match parent_data.children.entry(name.into()) {
                    Entry::Vacant(v) => {
                        v.insert(child_idx);
                        self.fs.0.push(child);
                        Ok(())
                    },
                    Entry::Occupied(o) => Err(anyhow!(
                        "Attempted to insert duplicate child entry {}",
                        o.key()
                    )),
                }
            } else {
                Err(anyhow!(
                    "Current dir is somehow not a folder (impossible??) (idx: {})",
                    self.idx.0
                ))
            }
        }

        pub fn add_file(&mut self, name: impl Into<String>, size: usize) -> Result<()> {
            self.add_child(name, Node::File(size))
        }

        pub fn add_folder(&mut self, name: impl Into<String>) -> Result<()> {
            let child = Node::Folder(FolderData {
                parent: Some(self.idx),
                children: HashMap::new(),
            });
            self.add_child(name, child)
        }

        pub fn cd(&'fs mut self, path: impl AsRef<str>) -> Result<()> {
            let node = &mut self.fs.0[self.idx.0];
            if let Node::Folder(folder_data) = node {
                match path.as_ref() {
                    "/" => {
                        *self = self.fs.root_mut();
                    },
                    ".." => {
                        let parent_idx = folder_data.parent.unwrap_or(NodeIdx(0));
                        self.idx = parent_idx;
                    },
                    child => {
                        let child_idx = folder_data.children.get(child)
                            .context(format!("no child")).copied()?;
                        self.idx = child_idx;
                    }
                }
                Ok(())
            } else {
                Err(anyhow!("Cannot cd into a file: {} (idx: {})", path.as_ref(), self.idx.0))
            }
        }
    }
}


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
