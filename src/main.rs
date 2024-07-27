#[derive(Debug)] // derive attribute for Deck to implement Debug trait.
struct Deck {
    cards: Vec<String>,
}
fn main() {
    // let deck = Deck { cards: Vec::new()}; // another way.
    let deck = Deck { cards: vec![]};

    println!("Here's your deck:\n {:#?}", deck);
}
