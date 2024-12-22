use crate::Btor;
use fol::{bitblast::bitblast_terms, Term, TermManager};
use std::collections::HashMap;

impl Btor {
    pub fn bitblast(&self) -> Btor {
        let mut tm = TermManager::new();
        let mut map = HashMap::new();
        let input: Vec<Term> = bitblast_terms(self.input.iter(), &mut tm, &mut map)
            .flatten()
            .collect();
        let latch: Vec<Term> = bitblast_terms(self.latch.iter(), &mut tm, &mut map)
            .flatten()
            .collect();
        let mut init = HashMap::new();
        for (l, i) in self.init.iter() {
            let l = l.bitblast(&mut tm, &mut map);
            let i = i.bitblast(&mut tm, &mut map);
            for (l, i) in l.iter().zip(i.iter()) {
                init.insert(l.clone(), i.clone());
            }
        }
        let mut next = HashMap::new();
        for (l, n) in self.next.iter() {
            let l = l.bitblast(&mut tm, &mut map);
            let n = n.bitblast(&mut tm, &mut map);
            for (l, n) in l.iter().zip(n.iter()) {
                next.insert(l.clone(), n.clone());
            }
        }
        let bad: Vec<Term> = bitblast_terms(self.bad.iter(), &mut tm, &mut map)
            .flatten()
            .collect();
        let constraint: Vec<Term> = bitblast_terms(self.constraint.iter(), &mut tm, &mut map)
            .flatten()
            .collect();
        Self {
            tm,
            input,
            latch,
            init,
            next,
            bad,
            constraint,
        }
    }
}
