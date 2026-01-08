#[derive(Debug)] // Is required in order to display our deck using the debug formatter "{:?}"
struct Deck {
    cards: Vec<String>,
}

// In Rust, the "main" function will always be called when running the program
fn main() {
    // "println!" is called a macro
    println!("Hello, world!");

    // Let's declare a new variable (in Rust, it's called a "binding" ~ so let's declare a new binding)
    let deck = Deck { cards: vec![] }; // This is writing using the macro
    // let deck = Deck { cards: Vec::new() }; // This is equivalent to the above line of code (this is written using the function)

    // Display your deck
    println!("Here's your deck: {:?}", deck); // ":?" is a (debug) formatter
    //println!("Here's your deck: {}", deck);
    //println!("Here's your deck: {deck}");
}
