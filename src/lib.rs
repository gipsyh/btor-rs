mod bitblast;
mod parse;

use fol::{Term, TermManager};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Btor {
    pub tm: TermManager,
    pub input: Vec<Term>,
    pub latch: Vec<Term>,
    pub init: HashMap<Term, Term>,
    pub next: HashMap<Term, Term>,
    pub bad: Vec<Term>,
    pub constraint: Vec<Term>,
}
