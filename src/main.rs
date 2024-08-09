use rand::{thread_rng, seq::SliceRandom};

#[derive(Debug)] // derive attribute for Deck to implement Debug trait.
struct Deck {
    cards: Vec<String>,
}

// adding inherent implementation
impl Deck {
    // fn new() -> Deck
    // new() is a an Associated function - tied to the struct definition.
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

        // return  Deck { cards }; // classic way to return

        Deck { cards} // implicit return of the Deck
    }

    // fn shuffle(&self) - this is a method - operates on a specific instance of struct.
    fn shuffle(&mut self) {
        let mut rng = thread_rng(); // random number generator
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}
fn main() {
    let mut deck = Deck::new();

    deck.shuffle();

    let cards = deck.deal(3);

    println!("Here's your deck:\n {:#?}", deck);
    println!("Here's your hand:\n {:#?}", cards)
}
