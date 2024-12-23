mod bitblast;
mod parse;

use fol::{Term, TermManager};
use parse::Parser;
use std::{collections::HashMap, path::Path};

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

impl Btor {
    pub fn new<P: AsRef<Path>>(tm: &TermManager, path: P) -> Self {
        let content = std::fs::read_to_string(path).unwrap();
        let mut parser = Parser::new(tm);
        parser.parse(&content)
    }
}
