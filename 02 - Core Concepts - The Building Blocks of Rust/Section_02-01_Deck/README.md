# Rust Deck Project

This project is a simple Rust application that demonstrates core language concepts—such as structs, vectors, arrays, and loops—by simulating the creation of a deck of cards.

## Project Structure

The main logic is located in `src/main.rs`. It illustrates the following:

- **Structs**: Defining a custom `Deck` data type.
- **Vectors (`Vec`)**: Storing a dynamic list of cards.
- **Arrays**: Using fixed-size lists for Suits and Values.
- **Macros**: Using `println!`, `vec!`, and `format!`.
- **Debug Formatting**: Using `#[derive(Debug)]` to print struct internals to the console.

## How to Run

Ensure you have Rust and Cargo installed. Navigate to the project directory and run:

```bash
cargo run
```

## Code Overview

### The Deck Struct
The project defines a `Deck` struct that holds a vector of strings. The `#[derive(Debug)]` attribute allows us to inspect the deck using the `{:?}` formatter.

```rust
#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}
```

### Generating the Deck
The application iterates through arrays of **Suits** and **Values** to generate card strings (e.g., "Ace of Hearts") and pushes them into the deck.

```rust
let suits = ["Hearts", "Spades", "Diamonds"];
let values = ["Ace", "Two", "Three"];

for suit in suits {
    for value in values {
        let card = format!("{} of {}", value, suit);
        cards.push(card);
    }
}
```