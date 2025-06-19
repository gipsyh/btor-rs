mod deparse;
mod parse;

use deparse::Deparser;
use giputils::hash::GHashMap;
use logicrs::fol::{Term, TermManager};
use parse::Parser;
use std::path::Path;

#[derive(Debug, Clone)]
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
    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        let tm = TermManager::new();
        let content = std::fs::read_to_string(path).unwrap();
        let mut parser = Parser::new(&tm);
        parser.parse(&content)
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) {
        let mut deparser = Deparser::new();
        let c = deparser.deparse(self);
        std::fs::write(path, c).unwrap();
    }
}
