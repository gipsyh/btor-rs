use logic_form::fol::{BiOpType, Sort, Term, UniOpType};
use num_bigint::BigInt;
use num_traits::Num;
use std::{collections::HashMap, path::Path};

#[derive(Debug)]
pub struct Btor2 {
    pub input: Vec<Term>,
    pub latch: Vec<Term>,
    pub init: HashMap<Term, Term>,
    pub next: HashMap<Term, Term>,
    pub bad: Term,
}

impl Btor2 {
    pub fn parse(s: &str) -> Self {
        let mut parser = Parser::default();
        parser.parse(s)
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

#[derive(Default)]
struct Parser {
    sorts: HashMap<u32, Sort>,
    nodes: HashMap<u32, Term>,
}

impl Parser {
    pub fn parse(&mut self, s: &str) -> Btor2 {
        let mut input = Vec::new();
        let mut latch = Vec::new();
        let mut init = HashMap::new();
        let mut next = HashMap::new();
        let mut bad = Term::bool_const(true);
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
            match second {
                "sort" => {
                    let sort = parse_sort(split);
                    assert!(self.sorts.insert(id, sort).is_none());
                }
                "state" => {
                    let sort = *self.sorts.get(&parse_id(&mut split)).unwrap();
                    let v = Term::new_var(sort);
                    latch.push(v.clone());
                    self.nodes.insert(id, v);
                }
                "init" => {
                    let sort = self.sorts.get(&parse_id(&mut split)).unwrap();
                    let state = self.nodes.get(&parse_id(&mut split)).unwrap().clone();
                    let value = self.nodes.get(&parse_id(&mut split)).unwrap().clone();
                    init.insert(state, value);
                }
                "next" => {
                    let sort = self.sorts.get(&parse_id(&mut split)).unwrap();
                    let state = self.nodes.get(&parse_id(&mut split)).unwrap().clone();
                    let value = self.nodes.get(&parse_id(&mut split)).unwrap().clone();
                    next.insert(state, value);
                }
                "bad" => {
                    let b = self.nodes.get(&parse_id(&mut split)).unwrap().clone();
                    bad = bad.and(&b);
                }
                _ => {
                    let term = self.parse_term(second, split);
                    assert!(self.nodes.insert(id, term).is_none());
                }
            }
        }
        Btor2 {
            input,
            latch,
            init,
            next,
            bad,
        }
    }

    fn parse_term<'a>(&mut self, second: &str, mut split: impl Iterator<Item = &'a str>) -> Term {
        if let Ok(ty) = ConstType::try_from(second) {
            let sort = *self.sorts.get(&parse_id(&mut split)).unwrap();
            let Sort::BV(w) = sort else { todo!() };
            let c = split.next().unwrap();
            let radix = match ty {
                ConstType::Const => 2,
                ConstType::Constd => 10,
                ConstType::Consth => 16,
            };
            let c = BigInt::from_str_radix(c, radix).unwrap();
            let (_, c) = c.to_radix_le(2);
            let mut c: Vec<bool> = c.into_iter().map(|x| x == 1).collect();
            while (c.len() as u32) < w {
                c.push(false);
            }
            dbg!(&c);
            return Term::bv_const(&c);
        }

        if let Ok(ty) = UniOpType::try_from(second) {
            let sort = self.sorts.get(&parse_id(&mut split)).unwrap();
            let a = self.nodes.get(&parse_id(&mut split)).unwrap();
            return a.uniop(ty);
        }

        if let Ok(ty) = BiOpType::try_from(second) {
            let sort = self.sorts.get(&parse_id(&mut split)).unwrap();
            let a = self.nodes.get(&parse_id(&mut split)).unwrap().clone();
            let b = self.nodes.get(&parse_id(&mut split)).unwrap();
            return a.biop(b, ty);
        }
        todo!()
    }
}

#[derive(Debug, Clone, strum::EnumString, strum::Display)]
#[strum(serialize_all = "lowercase")]
pub enum ConstType {
    Const = 2,
    Constd = 10,
    Consth = 16,
}

#[inline]
fn parse_id<'a>(split: &mut impl Iterator<Item = &'a str>) -> u32 {
    split.next().unwrap().parse().unwrap()
}

fn parse_sort<'a>(mut split: impl Iterator<Item = &'a str>) -> Sort {
    match split.next().unwrap() {
        "bitvec" => {
            let w = split.next().unwrap().parse::<u32>().unwrap();
            Sort::BV(w)
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
