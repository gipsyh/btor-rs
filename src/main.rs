use btor::Btor;
use fol::TermManager;

fn main() {
    let tm = TermManager::new();
    let btor2 = Btor::new(&tm, "counter.btor");
    dbg!(btor2);
}
