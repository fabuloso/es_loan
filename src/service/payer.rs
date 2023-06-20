pub trait Payer {
    fn charge(&self);
}

pub struct CardPayer {}

impl Payer for CardPayer {
    fn charge(&self) {
        println!("Living on a Payer!")
    }
}
