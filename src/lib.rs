mod bitblast;
mod parse;

use fol::{Term, TermManager};
use giputils::hash::GHashMap;
use parse::Parser;
use std::path::Path;

#[derive(Debug)]
pub struct Btor {
    pub tm: TermManager,
    pub input: Vec<Term>,
    pub latch: Vec<Term>,
    pub init: GHashMap<Term, Term>,
    pub next: GHashMap<Term, Term>,
    pub bad: Vec<Term>,
    pub constraint: Vec<Term>,
}

impl Btor {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let tm = TermManager::new();
        let content = std::fs::read_to_string(path).unwrap();
        let mut parser = Parser::new(&tm);
        parser.parse(&content)
    }
}
