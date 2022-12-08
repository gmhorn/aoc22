use crate::fs::FileSystem;
use anyhow::Result;
use aoc22::Timer;

// https://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/

fn main() -> Result<()> {
    let timer = Timer::tick();

    let filesystem: FileSystem = include_str!("../../data/day07.txt").parse()?;

    println!("{:?}", filesystem);

    timer.tock();
    Ok(())
}

mod fs {
    pub use anyhow::Result;
    use anyhow::{anyhow, Context, Error};
    use std::collections::hash_map::Entry;
    use std::collections::HashMap;
    use std::str::FromStr;

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

        pub fn root_mut(&mut self) -> CurrentDirMut {
            CurrentDirMut {
                fs: self,
                idx: NodeIdx(0),
            }
        }
    }

    impl FromStr for FileSystem {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut filesystem = Self::new();
            let mut cur_dir = filesystem.root_mut();

            let mut lines = s.lines().peekable();
            while let Some(cmd) = lines.next() {
                let tokens: Vec<_> = cmd.split_ascii_whitespace().collect();
                match &tokens[..] {
                    ["$", "ls"] => {
                        while let Some(fs_entry) = lines.next_if(|line| !line.starts_with("$")) {
                            let tokens: Vec<_> = fs_entry.split_ascii_whitespace().collect();
                            match &tokens[..] {
                                ["dir", name] => {
                                    cur_dir.add_folder(*name).context(format!("Could not add folder {}", name))?;
                                },
                                [size, name] => {
                                    let size: usize = size.parse().context(format!("invalid file size {} for file {}", size, name))?;
                                    cur_dir.add_file(*name, size).context(format!("Could not add file {}", name))?;
                                },
                                _ => return Err(anyhow!("invalid filesystem entry '{}'", fs_entry))
                            }
                        }
                    },
                    ["$", "cd", dir] => {
                        cur_dir = cur_dir.change(dir)?
                    },
                    _ => return Err(anyhow!("unknown command '{}'", cmd))
                }
            }
            Ok(filesystem)
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

    #[derive(Debug)]
    pub struct CurrentDirMut<'fs> {
        fs: &'fs mut FileSystem,
        idx: NodeIdx,
    }

    impl<'fs> CurrentDirMut<'fs> {
        fn add_child(&mut self, name: impl Into<String>, child: Node) -> Result<()> {
            let child_idx = NodeIdx(self.fs.0.len());
            let node = &mut self.fs.0[self.idx.0];
            if let Node::Folder(folder_data) = node {
                match folder_data.children.entry(name.into()) {
                    Entry::Vacant(v) => {
                        v.insert(child_idx);
                        self.fs.0.push(child);
                        Ok(())
                    },
                    Entry::Occupied(o) => Err(anyhow!(
                    "Attempted to insert duplicate child entry (name: {}, idx: {})", o.key(), self.idx.0)),
                }
            } else {
                Err(anyhow!("Cannot add children to file (idx: {})", self.idx.0))
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

        pub fn change(self, path: impl AsRef<str>) -> Result<Self> {
            let node = &mut self.fs.0[self.idx.0];
            if let Node::Folder(folder_data) = node {
                match path.as_ref() {
                    "/" => {
                        Ok(CurrentDirMut {
                            fs: self.fs,
                            idx: NodeIdx(0),
                        })
                    },
                    ".." => {
                        let parent_idx = folder_data.parent.unwrap_or(NodeIdx(0));
                        Ok(CurrentDirMut {
                            fs: self.fs,
                            idx: parent_idx,
                        })
                    },
                    child => {
                        let child_idx = folder_data.children.get(child)
                            .context(format!("No c")).copied()?;
                        Ok(CurrentDirMut {
                            fs: self.fs,
                            idx: child_idx,
                        })
                    }
                }
            } else {
                Err(anyhow!("Cannot cd into a file: {} (idx: {})", path.as_ref(), self.idx.0))
            }
        }
    }
}