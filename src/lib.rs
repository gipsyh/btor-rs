use fol::{
    op::{self, DynOp},
    Sort, Term,
};
use num_bigint::BigInt;
use num_traits::Num;
use std::{collections::HashMap, path::Path};

#[derive(Debug)]
pub struct Btor {
    pub input: Vec<Term>,
    pub latch: Vec<Term>,
    pub init: HashMap<Term, Term>,
    pub next: HashMap<Term, Term>,
    pub bad: Vec<Term>,
    pub constraint: Vec<Term>,
}

impl Btor {
    pub fn parse(s: &str) -> Self {
        let mut parser = Parser::default();
        parser.parse(s)
    }
}

impl<T> From<T> for Btor
where
    T: AsRef<Path>,
{
    fn from(value: T) -> Self {
        let content = std::fs::read_to_string(value).unwrap();
        Btor::parse(&content)
    }
}

#[derive(Default)]
struct Parser {
    sorts: HashMap<u32, Sort>,
    nodes: HashMap<u32, Term>,
}

impl Parser {
    pub fn parse_sort<'a>(&self, mut split: impl Iterator<Item = &'a str>) -> Sort {
        match split.next().unwrap() {
            "bitvec" => {
                let w = split.next().unwrap().parse::<u32>().unwrap();
                Sort::Bv(w)
            }
            "array" => {
                let index = parse_id(&mut split);
                let element = parse_id(&mut split);
                let index = self.sorts.get(&index).unwrap();
                let element = self.sorts.get(&element).unwrap();
                Sort::Array(index.bv_len(), element.bv_len())
            }
            _ => panic!(),
        }
    }

    pub fn parse(&mut self, s: &str) -> Btor {
        let mut input = Vec::new();
        let mut latch = Vec::new();
        let mut init = HashMap::new();
        let mut next = HashMap::new();
        let mut output = Vec::new();
        let mut bad = Vec::new();
        let mut constraint = Vec::new();
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
                    let sort = self.parse_sort(split);
                    assert!(self.sorts.insert(id, sort).is_none());
                }
                "input" => {
                    let sort = *self.sorts.get(&parse_id(&mut split)).unwrap();
                    let v = Term::new_var(sort, id);
                    input.push(v.clone());
                    assert!(self.nodes.insert(id, v).is_none());
                }
                "state" => {
                    let sort = *self.sorts.get(&parse_id(&mut split)).unwrap();
                    let v = Term::new_var(sort, id);
                    latch.push(v.clone());
                    assert!(self.nodes.insert(id, v).is_none());
                }
                "init" => {
                    let _sort = self.sorts.get(&parse_id(&mut split)).unwrap();
                    let state = self.nodes.get(&parse_id(&mut split)).unwrap().clone();
                    let value = self.nodes.get(&parse_id(&mut split)).unwrap().clone();
                    init.insert(state, value);
                }
                "next" => {
                    let _sort = self.sorts.get(&parse_id(&mut split)).unwrap();
                    let state = self.nodes.get(&parse_id(&mut split)).unwrap().clone();
                    let value = self.nodes.get(&parse_id(&mut split)).unwrap().clone();
                    next.insert(state, value);
                }
                "output" => {
                    let o = self.nodes.get(&parse_id(&mut split)).unwrap().clone();
                    output.push(o);
                }
                "bad" => {
                    let b = self.nodes.get(&parse_id(&mut split)).unwrap().clone();
                    bad.push(b);
                }
                "constraint" => {
                    let c = self.nodes.get(&parse_id(&mut split)).unwrap().clone();
                    constraint.push(c);
                }
                "const" | "constd" | "consth" => {
                    let ty = ConstType::try_from(second).unwrap();
                    let sort = *self.sorts.get(&parse_id(&mut split)).unwrap();
                    let Sort::Bv(w) = sort else { todo!() };
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
                    assert!(self.nodes.insert(id, Term::bv_const(&c)).is_none());
                }
                _ => {
                    let term = self.parse_op(second, split);
                    assert!(self.nodes.insert(id, term).is_none());
                }
            }
        }
        Btor {
            input,
            latch,
            init,
            next,
            bad,
            constraint,
        }
    }

    fn parse_op<'a>(&mut self, second: &str, mut split: impl Iterator<Item = &'a str>) -> Term {
        let op = DynOp::from(second);
        let _sort = self.sorts.get(&parse_id(&mut split)).unwrap();
        let mut operand = Vec::new();
        if &op == &op::Uext {
            let opa = self.nodes.get(&parse_id(&mut split)).unwrap().clone();
            let ext_len: usize = split.next().unwrap().parse().unwrap();
            let ext_len = Term::bv_const(&vec![false; ext_len]);
            operand.push(opa);
            operand.push(ext_len);
        } else {
            for _ in 0..op.num_operand() {
                operand.push(self.nodes.get(&parse_id(&mut split)).unwrap().clone());
            }
        }
        Term::new_op_term(op, &operand)
        // if second == "slice" {
        //     let sort = self.sorts.get(&parse_id(&mut split)).unwrap();
        //     let a = self.nodes.get(&parse_id(&mut split)).unwrap();
        //     let upper: u32 = split.next().unwrap().parse().unwrap();
        //     let lower: u32 = split.next().unwrap().parse().unwrap();
        //     return a.slice(upper, lower);
        // }
        // todo!()
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
