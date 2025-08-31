use crate::Btor;
use giputils::hash::GHashMap;
use logicrs::fol::{
    BvConst, Sort, Term,
    op::{self, DynOp},
};
use num_bigint::{BigInt, Sign};
use num_traits::Num;

#[derive(Default)]
pub struct Parser {
    sorts: GHashMap<usize, Sort>,
    nodes: GHashMap<usize, Term>,
}

impl Parser {
    #[inline]
    fn get_node(&self, nid: isize) -> Term {
        let abs: usize = nid.unsigned_abs();
        let mut res = self.nodes.get(&abs).unwrap().clone();
        if nid < 0 {
            res = !res;
        }
        res
    }

    pub fn parse_sort<'a>(&self, mut split: impl Iterator<Item = &'a str>) -> Sort {
        match split.next().unwrap() {
            "bitvec" => {
                let w = split.next().unwrap().parse::<usize>().unwrap();
                Sort::Bv(w)
            }
            "array" => {
                let index = parse_id(&mut split);
                let element = parse_id(&mut split);
                let index = self.sorts.get(&index).unwrap();
                let element = self.sorts.get(&element).unwrap();
                if element.is_array() {
                    panic!("currently arrays of arrays are not supported");
                }
                Sort::Array(index.bv(), element.bv())
            }
            _ => panic!(),
        }
    }

    pub fn parse(&mut self, s: &str) -> Btor {
        let mut input = Vec::new();
        let mut latch = Vec::new();
        let mut init = GHashMap::new();
        let mut next = GHashMap::new();
        let mut output = Vec::new();
        let mut bad = Vec::new();
        let mut constraint = Vec::new();
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
            match second {
                "sort" => {
                    let sort = self.parse_sort(split);
                    assert!(self.sorts.insert(id, sort).is_none());
                }
                "input" => {
                    let sort = *self.sorts.get(&parse_id(&mut split)).unwrap();
                    let v = Term::new_var(sort);
                    input.push(v.clone());
                    assert!(self.nodes.insert(id, v).is_none());
                }
                "state" => {
                    let sort = *self.sorts.get(&parse_id(&mut split)).unwrap();
                    let v = Term::new_var(sort);
                    latch.push(v.clone());
                    assert!(self.nodes.insert(id, v).is_none());
                }
                "init" => {
                    let _sort = self.sorts.get(&parse_id(&mut split)).unwrap();
                    let state = self.get_node(parse_signed_id(&mut split));
                    let value = self.get_node(parse_signed_id(&mut split));
                    init.insert(state, value);
                }
                "next" => {
                    let sort = self.sorts.get(&parse_id(&mut split)).unwrap();
                    let state = self.get_node(parse_signed_id(&mut split));
                    let value = self.get_node(parse_signed_id(&mut split));
                    assert!(state.sort().eq(sort));
                    assert!(value.sort().eq(sort));
                    next.insert(state, value);
                }
                "output" => {
                    let o = self.get_node(parse_signed_id(&mut split));
                    output.push(o);
                }
                "bad" => {
                    let b = self.get_node(parse_signed_id(&mut split));
                    bad.push(b);
                }
                "constraint" => {
                    let c = self.get_node(parse_signed_id(&mut split));
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
                    let (s, c) = c.to_radix_le(2);
                    let mut c: Vec<bool> = c.into_iter().map(|x| x == 1).collect();
                    while c.len() < w {
                        c.push(false);
                    }
                    if let Sign::Minus = s {
                        c = c.into_iter().map(|x| !x).collect();
                        let mut carry = true;
                        for i in c.iter_mut() {
                            if !carry {
                                break;
                            }
                            let ni = *i ^ carry;
                            carry = *i && carry;
                            *i = ni;
                        }
                    }
                    assert!(
                        self.nodes
                            .insert(id, Term::bv_const(BvConst::new(&c)))
                            .is_none()
                    );
                }
                "zero" => {
                    let sort = *self.sorts.get(&parse_id(&mut split)).unwrap();
                    assert!(
                        self.nodes
                            .insert(id, Term::bv_const_zero(sort.bv()))
                            .is_none()
                    );
                }
                "one" => {
                    let sort = *self.sorts.get(&parse_id(&mut split)).unwrap();
                    assert!(
                        self.nodes
                            .insert(id, Term::bv_const_one(sort.bv()))
                            .is_none()
                    );
                }
                "ones" => {
                    let sort = *self.sorts.get(&parse_id(&mut split)).unwrap();
                    assert!(
                        self.nodes
                            .insert(id, Term::bv_const_ones(sort.bv()))
                            .is_none()
                    );
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
        let sort = self.sorts.get(&parse_id(&mut split)).unwrap();
        let mut operand = Vec::new();
        if op == op::Uext || op == op::Sext {
            let opa = self.get_node(parse_signed_id(&mut split));
            let ext_len: usize = split.next().unwrap().parse().unwrap();
            let ext_len = Term::bv_const_zero(ext_len);
            operand.push(opa);
            operand.push(ext_len);
        } else if op == op::Slice {
            let opa = self.get_node(parse_signed_id(&mut split));
            let high: usize = split.next().unwrap().parse().unwrap();
            let low: usize = split.next().unwrap().parse().unwrap();
            operand.push(opa);
            operand.push(Term::bv_const_zero(high));
            operand.push(Term::bv_const_zero(low));
        } else {
            for _ in 0..op.num_operand() {
                operand.push(self.get_node(parse_signed_id(&mut split)));
            }
        }
        let res = Term::new_op(op, &operand);
        assert!(res.sort().eq(sort));
        res
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
fn parse_id<'a>(split: &mut impl Iterator<Item = &'a str>) -> usize {
    split.next().unwrap().parse().unwrap()
}

#[inline]
fn parse_signed_id<'a>(split: &mut impl Iterator<Item = &'a str>) -> isize {
    split.next().unwrap().parse().unwrap()
}
