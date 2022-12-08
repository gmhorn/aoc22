use anyhow::{anyhow, Context, Error, Result};
use aoc22::Timer;
use std::collections::HashSet;
use std::str::FromStr;

fn main() -> Result<()> {
    let timer = Timer::tick();

    let forest: Forest = include_str!("../../data/day08.txt").parse()?;

    let mut visible: HashSet<Tree> = HashSet::new();
    for row in 0..forest.height() {
        visible.extend(Tree::visible(&mut forest.iter_row(row)));
        visible.extend(Tree::visible(&mut forest.iter_row(row).rev()));
    }

    for col in 0..forest.width() {
        visible.extend(Tree::visible(&mut forest.iter_col(col)));
        visible.extend(Tree::visible(&mut forest.iter_col(col).rev()));
    }
    println!("{}", visible.len());

    let most_scenic = forest
        .iter_trees()
        .map(|tree| forest.scenic_score(&tree))
        .max()
        .context("could not calculate most scenic tree")?;
    println!("{}", most_scenic);

    timer.tock();
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tree {
    pub x: usize,
    pub y: usize,
    pub height: u8,
}

impl Tree {
    pub fn new(x: usize, y: usize, height: u8) -> Self {
        Self { x, y, height }
    }

    pub fn visible(trees: &mut impl Iterator<Item = Self>) -> Vec<Tree> {
        let mut answer = vec![];

        if let Some(first) = trees.next() {
            let mut max = first.height;
            answer.push(first);

            for tree in trees {
                if tree.height > max {
                    max = tree.height;
                    answer.push(tree);
                }
            }
        }

        answer
    }

    pub fn count_visible(&self, trees: &mut impl Iterator<Item = Self>) -> usize {
        // Advance the iterator until we hit ourself
        trees.take_while(|other| self.ne(other)).for_each(|_| {});

        // Start counting!
        let mut count = 0;
        for tree in trees {
            count += 1;
            if self.height <= tree.height {
                break;
            }
        }
        count
    }
}

#[derive(Debug)]
pub struct Forest {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

// Alias to make signatures a bit easier
pub trait TreeIter<'a>:
    DoubleEndedIterator<Item = Tree> + ExactSizeIterator<Item = Tree> + 'a
{
}
impl<'a, T> TreeIter<'a> for T where
    T: DoubleEndedIterator<Item = Tree> + ExactSizeIterator<Item = Tree> + 'a
{
}

impl Forest {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn iter_trees(&self) -> impl TreeIter {
        self.data.iter().enumerate().map(|(idx, &height)| {
            let x = idx % self.width;
            let y = idx / self.width;
            Tree::new(x, y, height)
        })
    }

    pub fn iter_row(&self, row: usize) -> impl TreeIter {
        self.iter_trees().skip(row * self.width).take(self.width())
    }

    pub fn iter_col(&self, col: usize) -> impl TreeIter {
        self.iter_trees().skip(col).step_by(self.width)
    }

    pub fn scenic_score(&self, tree: &Tree) -> usize {
        let right = tree.count_visible(&mut self.iter_row(tree.y));
        let left = tree.count_visible(&mut self.iter_row(tree.y).rev());
        let down = tree.count_visible(&mut self.iter_col(tree.x));
        let up = tree.count_visible(&mut self.iter_col(tree.x).rev());

        right * left * down * up
    }
}

impl FromStr for Forest {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = vec![];
        let zero = '0' as u32;

        let mut row_lens = HashSet::new();
        let mut height = 0;

        for row in s.lines() {
            height += 1;
            row_lens.insert(row.len());

            data.extend(row.chars().map(|c| ((c as u32) - zero) as u8).into_iter());
        }

        if row_lens.len() == 1 {
            Ok(Self {
                width: row_lens.into_iter().next().unwrap(),
                height,
                data,
            })
        } else {
            Err(anyhow!("Forest must be square!"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scenic_score() {
        let forest: Forest = "30373\n25512\n65332\n33549\n35390".parse().unwrap();

        assert_eq!(4, forest.scenic_score(&Tree::new(2, 1, 5)));
        assert_eq!(8, forest.scenic_score(&Tree::new(2, 3, 5)));
        assert_eq!(0, forest.scenic_score(&Tree::new(0, 0, 3)));
    }
}
