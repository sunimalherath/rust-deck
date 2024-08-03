#[derive(Debug)] // derive attribute for Deck to implement Debug trait.
struct Deck {
    cards: Vec<String>,
}

// adding inherent implementation
impl Deck {
    // fn new() -> Deck
    fn new() -> Self {
        let suits = ["Hearts", "Spades", "Clubs", "Diamonds"];
        let values = [
            "Two", "Three", "Four", "Five","Six", "Seven", "Eight",
            "Nine", "Ten", "Jack", "Queen", "King", "Ace"
        ];

        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        // let deck = Deck { cards: Vec::new()}; // another way.
        // let deck = Deck { cards: cards}; // This can be simplified as follows
        let deck = Deck { cards };

        return deck;
    }
}
fn main() {
    let deck = Deck::new();

    println!("Here's your deck:\n {:#?}", deck);
}
