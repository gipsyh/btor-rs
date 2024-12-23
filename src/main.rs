use btor::Btor;
use fol::TermManager;

fn main() {
    let btor2 = Btor::new("counter.btor");
    dbg!(btor2);
}
