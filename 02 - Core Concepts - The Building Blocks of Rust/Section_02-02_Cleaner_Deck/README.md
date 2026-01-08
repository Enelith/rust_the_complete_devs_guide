# Rust Deck of Cards

A simple command-line application written in Rust that simulates a deck of playing cards. This project demonstrates core Rust concepts such as structs, inherent implementations, vectors, ownership, and external crate usage.

## Features

- **Create Deck**: Generates a deck of cards combining suits (Hearts, Spades, Diamonds) and values (Ace, Two, Three).
- **Shuffle**: Randomizes the order of cards using the `rand` crate.
- **Deal**: Removes a specified number of cards from the deck and returns them as a hand.

## Prerequisites

- Rust and Cargo installed.

## Dependencies

This project requires the `rand` crate. Ensure your `Cargo.toml` contains the dependency:

```toml
[dependencies]
rand = "0.9.0" # Or the version matching the API usage in main.rs
```

## Usage

To run the program, execute the following command in the project root:

```bash
cargo run
```

## Code Structure

The main logic resides in `src/main.rs`:

- **`struct Deck`**: A wrapper around a `Vec<String>` to hold the cards.
- **`impl Deck`**:
  - `new()`: Constructor that initializes the deck with combinations of suits and values.
  - `shuffle()`: Mutates the deck in-place using a random number generator.
  - `deal(num_cards)`: Splits the vector to return a specific number of cards (the hand) and keeps the rest in the deck.

## Example Output

```text
Here's your original deck: Deck { cards: ["Ace of Hearts", "Two of Hearts", ...] }
Here's your shuffled deck: Deck { cards: ["Two of Spades", "Ace of Diamonds", ...] }
Here's your hand: ["Three of Hearts", "Ace of Spades"]
Here's what's left of the shuffled deck: Deck { ... }
```