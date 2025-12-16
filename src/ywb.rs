use crate::Btor;
use giputils::hash::GHashMap;
use logicrs::fol::Term;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
struct Clock {
    path: Vec<String>,
    offset: u32,
    edge: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SignalPart {
    pub path: Vec<String>,
    pub width: u32,
    pub offset: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct BtorWitnessMap {
    version: String,
    generator: String,
    clocks: Vec<Clock>,
    inputs: Vec<Vec<SignalPart>>,
    states: Vec<Vec<SignalPart>>,
    asserts: Vec<Vec<String>>,
    assumes: Vec<Vec<String>>,
}

impl Btor {
    pub fn witness_map(&self, s: &str) -> GHashMap<Term, Vec<SignalPart>> {
        let ywb: BtorWitnessMap = serde_json::from_str(s).unwrap();
        assert!(self.input.len() == ywb.inputs.len());
        assert!(self.latch.len() == ywb.states.len());
        let mut map: GHashMap<Term, Vec<SignalPart>> = GHashMap::new();
        for (i, s) in self.input.iter().zip(ywb.inputs.iter()) {
            map.insert(i.clone(), s.clone());
        }
        for (l, s) in self.latch.iter().zip(ywb.states.iter()) {
            map.insert(l.clone(), s.clone());
        }
        map
    }
}
