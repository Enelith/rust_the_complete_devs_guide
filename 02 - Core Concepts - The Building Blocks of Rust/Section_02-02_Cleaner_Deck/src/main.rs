#[derive(Debug)] // Is required in order to display our deck using the debug formatter "{:?}"
struct Deck {
    cards: Vec<String>,
}

// Let's create an "inherent implementation"
impl Deck {
    // Creating a function called "new" that will return a Deck
    // (and because we're returning a Deck inside an "impl Deck", we'll use "Self"
    // ~ "Self" is a keyword which references to whatever type was mentioned in the parent implementation block).
    // fn new() -> Deck {
    fn new() -> Self {
        // List of 'suits' - "Hearts", "Spades", ...
        // List of 'values' - "ace", "two", ...
        // Both will be arrays because they probably won't change in size (otherwise, use a vec! for Vector)
        let suits = ["Hearts", "Spades", "Diamonds"];
        let values = ["Ace", "Two", "Three"];

        let mut cards = vec![];

        // Double nested for loop
        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        // let deck = Deck { cards: cards}; // Because we have a field named with an identical value we're to assign to it, it can be shortened as the following line of code
        let deck = Deck { cards };
        deck // is EQUALS to "return deck;" ; we could even just write "Deck { cards }" instead of creating a binding "deck"
    }

    fn shuffle(&self) {

    }
}

// In Rust, the "main" function will always be called when running the program
fn main() {
    let deck = Deck::new();

    deck.shuffle();

    println!("Here's your deck: {:#?}", deck); // Adding a "#" to the debug formatter will render it much better / easier to read.
}
