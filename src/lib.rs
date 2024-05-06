mod node;
mod op;

use logic_form::fol::Sort;
pub use node::*;
pub use op::*;
use std::{collections::BTreeMap, path::Path};

#[derive(Default, Debug)]
pub struct Btor2 {
    pub nodes: BTreeMap<u32, Node>,
}

impl Btor2 {
    pub fn parse(s: &str) -> Self {
        let mut sorts = BTreeMap::new();
        let mut nodes = BTreeMap::new();

        for line in s.lines() {
            dbg!(line);
            if line.starts_with(';') {
                continue;
            }
            let mut split = line.split_whitespace();
            let Some(id) = split.next() else {
                continue;
            };
            let id = id.parse().unwrap();
            let second = split.next().unwrap();
            if second == "sort" {
                let sort = parse_sort(split);
                assert!(sorts.insert(id, sort).is_none());
            }
        }

        for line in s.lines() {
            if line.starts_with(';') {
                continue;
            }
            let mut split = line.split_whitespace();
            let Some(id) = split.next() else {
                continue;
            };
            let id = id.parse().unwrap();
            let second = split.next().unwrap();
            if second == "sort" {
                continue;
            }
            if let Some(node) = Node::parse(second, split, &sorts) {
                assert!(nodes.insert(id, node).is_none());
            }
        }

        Self { nodes }
    }
}

impl<T> From<T> for Btor2
where
    T: AsRef<Path>,
{
    fn from(value: T) -> Self {
        let content = std::fs::read_to_string(value).unwrap();
        Btor2::parse(&content)
    }
}

#[inline]
fn parse_id<'a>(split: &mut impl Iterator<Item = &'a str>) -> u32 {
    split.next().unwrap().parse().unwrap()
}

fn parse_sort<'a>(mut split: impl Iterator<Item = &'a str>) -> Sort {
    match split.next().unwrap() {
        "bitvec" => {
            let length = split.next().unwrap().parse::<u32>().unwrap();
            Sort::BV(length)
        }
        "array" => {
            // let index = parse_sid(&mut split)?;
            // let element = parse_sid(&mut split)?;
            // Ok(Sort::Array(Array { index, element }))
            todo!()
        }
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let btor2 = Btor2::from("/root/wIC3/counter.btor2");
        dbg!(btor2);
    }
}
