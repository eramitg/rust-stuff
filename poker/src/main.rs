extern crate rand;

mod card;
mod hand;
mod deck;


use card::{Card};
use deck::{Deck};


pub fn main() {
    let deck: Vec<Card> = Deck::new();
    println!("{:?}", deck);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
