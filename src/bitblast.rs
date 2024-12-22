use crate::Btor;
use aig::Aig;
use std::collections::HashMap;

impl Btor {
    pub fn bitblast(&self) -> Aig {
        let mut aig = Aig::new();
        let mut map = HashMap::new();
        for input in self.input.iter().chain(self.latch.iter()) {
            let aig_input = aig.new_input();
            map.insert(input.clone(), aig_input);
        }

        todo!()
    }
}
