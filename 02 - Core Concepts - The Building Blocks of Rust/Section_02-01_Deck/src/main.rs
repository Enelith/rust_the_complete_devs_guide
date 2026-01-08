#[derive(Debug)] // Is required in order to display our deck using the debug formatter "{:?}"
struct Deck {
    cards: Vec<String>,
}

// In Rust, the "main" function will always be called when running the program
fn main() {
    // "println!" is called a macro
    println!("Hello, world!");

    // List of 'suits' - "Hearts", "Spades", ...
    // List of 'values' - "ace", "two", ...
    // Both will be arrays because they probably won't change in size (otherwise, use a vec! for Vector)
    let suits = ["Hearts", "Spades", "Diamonds"];
    let values = ["Ace", "Two", "Three"];

    // Double nested for loop
    let cards = vec![];
    for suit in suits {
        for value in values {
            let card = format!("{} of {}", value, suit);
            cards.push(card);
        }
    }

    // Let's declare a new variable (in Rust, it's called a "binding" ~ so let's declare a new binding)
    let deck = Deck { cards: vec![] }; // This is writing using the macro
    // let deck = Deck { cards: Vec::new() }; // This is equivalent to the above line of code (this is written using the function)

    // Display your deck
    println!("Here's your deck: {:?}", deck); // ":?" is a (debug) formatter
    //println!("Here's your deck: {}", deck);
    //println!("Here's your deck: {deck}");
}
