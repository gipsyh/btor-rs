use crate::Btor;
use giputils::hash::GHashMap;
use logicrs::fol::{op, Sort, Term, TermType};
use std::ops::Deref;

pub struct Deparser {
    sorts: GHashMap<Sort, usize>,
    terms: GHashMap<Term, usize>,
    content: Vec<String>,
}

impl Deparser {
    pub fn new() -> Self {
        Self {
            sorts: Default::default(),
            terms: Default::default(),
            content: Vec::new(),
        }
    }

    fn get_sort_id(&mut self, sort: Sort) -> usize {
        if let Some(id) = self.sorts.get(&sort) {
            return *id;
        }
        let line = match sort {
            Sort::Bv(w) => format!("sort bitvec {w}"),
            Sort::Array(i, e) => {
                let i = self.get_sort_id(Sort::Bv(i));
                let e = self.get_sort_id(Sort::Bv(e));
                format!("sort array {i} {e}")
            }
        };
        self.content.push(line);
        self.sorts.insert(sort, self.content.len());
        self.content.len()
    }

    fn get_term_id(&mut self, term: &Term) -> usize {
        if let Some(id) = self.terms.get(term) {
            return *id;
        }
        let sid = self.get_sort_id(term.sort());
        let line = match term.deref() {
            TermType::Const(c) => {
                let mut line = format!("const {sid} ");
                line.extend(c.iter().map(|b| if *b { '1' } else { '0' }).rev());
                line
            }
            TermType::Op(op) => {
                assert!(op.op.is_core());
                let args: Vec<_> = if op.op == op::Sext || op.op == op::Uext {
                    vec![self.get_term_id(&op.terms[0]), op.terms[1].bv_len()]
                } else if op.op == op::Slice {
                    let arg = self.get_term_id(&op.terms[0]);
                    let h = op.terms[1].bv_len();
                    let l = op.terms[2].bv_len();
                    vec![arg, h, l]
                } else {
                    op.terms.iter().map(|arg| self.get_term_id(arg)).collect()
                };
                format!(
                    "{} {sid} {}",
                    op.op.name().to_lowercase(),
                    args.iter()
                        .map(|id| id.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                )
            }
            TermType::Var(_) => panic!(),
        };
        self.content.push(line);
        self.terms.insert(term.clone(), self.content.len());
        self.content.len()
    }

    pub fn deparse(&mut self, btor: &Btor) -> String {
        for i in btor.input.iter() {
            let line = format!("input {}", self.get_sort_id(i.sort()),);
            self.content.push(line);
            self.terms.insert(i.clone(), self.content.len());
        }
        for i in btor.init.values() {
            self.get_sort_id(i.sort());
            self.get_term_id(i);
        }
        for l in btor.latch.iter() {
            let line = format!("state {}", self.get_sort_id(l.sort()),);
            self.content.push(line);
            self.terms.insert(l.clone(), self.content.len());
        }
        for (l, i) in btor.init.iter() {
            let line = format!(
                "init {} {} {}",
                self.get_sort_id(l.sort()),
                self.get_term_id(l),
                self.get_term_id(i)
            );
            self.content.push(line);
        }
        for (l, i) in btor.next.iter() {
            let line = format!(
                "next {} {} {}",
                self.get_sort_id(l.sort()),
                self.get_term_id(l),
                self.get_term_id(i)
            );
            self.content.push(line);
        }
        for b in btor.bad.iter() {
            let line = format!("bad {}", self.get_term_id(b));
            self.content.push(line);
        }
        for c in btor.constraint.iter() {
            let line = format!("constraint {}", self.get_term_id(c));
            self.content.push(line);
        }
        for (i, l) in self.content.iter_mut().enumerate() {
            *l = format!("{} {}", i + 1, l);
        }
        format!("{}\n", self.content.join("\n"))
    }
}
