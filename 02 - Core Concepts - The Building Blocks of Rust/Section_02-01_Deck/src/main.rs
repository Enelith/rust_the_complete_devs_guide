#[derive(Debug)] // Is required in order to display our deck using the debug formatter "{:?}"
struct Deck {
    cards: Vec<String>,
}

// In Rust, the "main" function will always be called when running the program
fn main() {
    // "println!" is called a macro
    println!("Hello, world!");
    println!(" ==================================== ");

    // Let's declare a new variable (in Rust, it's called a "binding" ~ so let's declare a new binding)
    let empty_deck = Deck { cards: vec![] }; // This is writing using the macro
    // let deck = Deck { cards: Vec::new() }; // This is equivalent to the above line of code (this is written using the function)

    // Display your deck
    println!("Here's your empty deck: {:?}", empty_deck); // ":?" is a (debug) formatter
    //println!("Here's your deck: {}", deck);
    //println!("Here's your deck: {deck}");
    println!(" ==================================== ");

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
    println!("Here's your deck: {:#?}", deck); // Adding a "#" to the debug formatter will render it much better / easier to read.
    println!(" ==================================== ");
}
