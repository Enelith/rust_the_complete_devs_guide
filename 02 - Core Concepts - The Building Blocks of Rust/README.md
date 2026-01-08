## Attributes
```
#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}
```
- Provides extra instructions to the compiler
- `derive` is an *attribute*. Tells the compiler to add additional code to the struct.
- `Debug` is a *trait*. Has functions included that aid in debugging (like printing a struct).
---

## Struct
```
struct Deck {
    cards: Vec<String>,
}
```
- Defines a collection of fields (data) that are related in some way.
- Can be used to tie together data + functionality if we add an `impl` block.
---

## Vectors and Arrays

This creates an **array** of strings.
<br/>
Arrays are fixed in size (can't grow, or shrink)
```
let suits = ["Diamonds", "Clubs"];
let values = ["2", "3", "4", "5"];
```
---
This creates a **Vector** of strings
<br/>
Vectors are dynamic (they can grow or shrink)
```
let suits = vec!["Diamonds", "Clubs"];
let values = vec!["2", "3", "4", "5"];
```

**Are the suits / values going to change over time ? If not, maybe use an array !**

---

## Mutable vs Immutable Bindings

Bindings are **immutable** by default (can't be changed).


### Immutable
```
let numbers = vec![];

// Error ! Can't CHANGE the value
numbers.push(1);

// Error ! Can't REASSIGN the value either !
numbers = vec![];
```

### Mutable
```
let mut numbers = vec![];

// Ok!
numbers.push(1);

// Ok!
numbers = vec![];
```

---

## Inherent Implementation
- Defines methods + associated functions tied to a Struct
- First argument determines whether we are making a method or an associated function.

Fancy term for `add a function to a struct`.
<br/>
Used to define **methods** and **associated functions** (aka class methods).

```
// Impl definition. Same name as struct.
impl Deck {
    // This is an "associated function", tied to the struct definition.
    fn new() {
        // ...
    }
    
    // This a "method", it operates on a specific instance of the struct.
    fn shuffle(&self) {
        // ...
    }
}

fn main() {
    // 'associated functions' are called using the '::' syntax  
    let deck = Deck::new();

    deck.shuffle();
}
```

To define a **method**, the only thing you have to do, is define a function and the first argument is going to be *&self*.
<br/>
That's it, that's what differenciate a **method** from an **associated function**.

### Associated Functions
```
impl Deck {
    fn new() -> Self {
        // ...
    }
}

fn main() {
    Deck::new();
}
```

- Use when you have functionality not tied to a specific instance
  - Example: *full_deck()*, *with_n_cards(10)*, *empty_deck()*, ...

### Methods
```
impl Deck {
    fn shuffle(&self) {
        // ...
    }
}

fn main() {
    deck.shuffle();
}
```
- Use when you need to read of change fields on a specific instance
  - Example: shuffling cards, adding a card, removing a card, checking if a card exists, ...

---

## Implicit Return
```
fn is_even(num: i32) -> bool {
    return num % 2 == 0;
}
```
can also be written as (and it's totally equivalent to):
```
fn is_even(num: i32) -> bool {
    num % 2 == 0 
}
```
Note: *Don't forget to drop the semicolon*

Rush is going to automatically return the last executed expression inside your function 
as long as it doesn't have a semicolon *;* at the end of the line.

---

## Crate == Package

Rust Standard Library: 
- Included with every project without any additional install
- Docs at **doc.rust-lang.org/std**

External Crates:
- Have to be installed into our project with *cargo add rand*
- Crate listing at **cartes.io**
- Docs also at **docs.rs**

Note that we can directly access **external** crates, but to use **internal** modules, we need to use the `mod` keyword.

---

## Types of Numbers

| Types   | Description                   | Range                           |
|:--------|:------------------------------|:--------------------------------|
| `i8`    | Positive or negative integers | -128 to 127                     |
| `i16`   | Positive or negative integers | -32.768 to 32.767               |
| `i32`   | Positive or negative integers | -2.147.483.648 to 2.147.483.647 |                 |
| `i64`   | Positive or negative integers | -9.2E18 to 9.2E18               |                          |
| `i128`  | Positive or negative integers | -1.7E38 to 1.7E38               |
| `isize` | Positive or negative integers | -9.2E18 to 9.2E18               |
| `u8`    | Unsigned integers             | 0 to 255                        |
| `u16`   | Unsigned integers             | 0 to 65.535                     |
| `u32`   | Unsigned integers             | 0 to 4.29E9                     |
| `u64`   | Unsigned integers             | 0 to 1.84E19                    |
| `u128`  | Unsigned integers             | 0 to 3.4E38                     |
| `usize` | Unsigned integers             | 0 to 1.84E19                    |
| `f32`   | Decimal values                | -3.4E38 to 3.4E38               |
| `f64`   | Decimal values                | -1.7E308 to 1.7E308             |
