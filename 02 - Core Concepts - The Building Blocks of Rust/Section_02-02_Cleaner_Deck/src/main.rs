// use rand::{thread_rng, seq::SliceRandom}
// This is equivalent to defining an alias in order to use the external crate methods in our code:
// use rand::thread_rng
// use rand::seq::SliceRandom
// Also, note that rand::thread_rng is deprecated since 0.9.0, and renamed 'rng'
use rand::{rng, seq::SliceRandom};

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

    // When calling shuffle, we're going to have a REFERENCE to our deck instance, and we're going to expect in this method to
    // change the data inside that deck instance in some way (we want to randomize a list of cards).
    // So, in addition to mark, in the 'main' function, the binding 'deck' with 'mut',
    // we ALSO need to mark '&self' as '&mut self' (that's going to indicate that when we receive this REFERENCE
    // to the deck we're going to change, we're going to get a MUTABLE / CHANGEABLE version of it).
    // fn shuffle(&self) {...} -> fn shuffle(&mut self) {...}
    fn shuffle(&mut self) {
        // Creating a new random number generator by calling the crate 'rand'
        // Equivalent to: let mut rng = rand::rng();
        let mut rng = rng(); // ('mut' is also added here, bcs the data inside the 'rng' will also change in some way)

        self.cards.shuffle(&mut rng)
    }
}

// In Rust, the "main" function will always be called when running the program
fn main() {
    // Because we're going to shuffle our deck, the data inside 'deck' will change, and by default, 'deck' is immutable.
    // Ergo, we need to qualify 'deck' with the keyword 'mut'
    let mut deck = Deck::new();

    deck.shuffle();

    println!("Here's your deck: {:#?}", deck); // Adding a "#" to the debug formatter will render it much better / easier to read.
}
