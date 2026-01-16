# Section_07-02_Explaining_Return_Enum

### The function definition

```
fn divide(a: f64, b: f64) -> Result<f64, Error>
```

- Here, we're saying we're to return something of type `Result`
- `Result` is an enum, which is a built-in type in Rust (there's no need to import or anything like that ~ we can
  directly use it the same way we would use the `Option` enum)
- The angle brackets `<...>` indicates this is a **generic enum**
    - A generic means we can use different kind of types in this enum
    - The actual definition of the `Result` enum (in the Rust source code) is exactly as follow:

````
enum Result<T, E> {
    Ok(T),
    Err(E)
}
````

- These angle brackets `<T, E>` are very similar in nature to function **arguments**, but for **types** instead
- So whenever we're making use of this `Result` enum, we're going to add in those angle brackets, and this is like
  *calling a function with some values*.
    - Instead, we're kind of redefining the `Result` type with some customs types put into it

As a result, we can understand in our code that from `-> Result<f64, Error>`,

- `f64` will show up as the letter `T`
- and `Error` will show up as the letter `E`

The end result to summarize this, is we end with:

```
enum Result {
    Ok(f64),
    Err(Error)
}
```

- The `Ok()` variant will contain a float64 `f64`
- The `Err()` variant will contain some value of type `Error`

---

Remember, enums are *kind of* like a shortcut for defining multiple structs.

```
fn divide(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        Err(Error::other("Oops..."))
    } 
    
    Ok(1.33)
}
```

is similar in essence to:

```
// This code won't work, but it gives an idea

struct OkResult {
    value_that_was_calculated: f64
}

struct ErrResult {
    the_error_that_occured: Error
} 

fn divide(a: f64, b: f64) -> OkResult or ErrResult {
    if b == 0.0 {
        ErrResult {
            the_error_that_occured: Error::other("Oops...")
        }
    } 
    
    OkResult {
        value_that_was_calculated: 1.33
    }
}
```

### Types of Errors

Imports a struct defined in the std lib. Used to represent an error:

```
use std::io::Error;
```

Creates an instance of the Error struct:

```
Error::other("Can't divide by 0") // <=> Similar to the Bank::new() from before
```

Many modules in the std lib have their own custom error types.

```
use std::str::Utf8Error;
```

```
use std::string::FromUtf8Error;
```

```
use std::num::ParseIntError;
use std::num::ParseFloatError;
use std::num::TryFromIntError;
```

```
use std::thread::JoinError;
```

```
use std::io::Error;
```

You can also create your own custom error types.

There isn't really a general-purpose catch-all type of error (Javascript has 'Error', Python has 'Exception', ...)

---

### Empty OK Variants

The value we put in the `Ok()` is the result of the successful operation.

```
fn divide(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        Err(Error::other("Can't divide by 0"))
    } else {
        Ok(a / b)
    }
}
```

But what would happen / what should we put there if we have a successful operation that doesn't really give us any
value?

Examples:

- Writing data to a file
- Removing a directory
- Permission check
- Validating a String

Those operations either fail (in which case they'll probably return an error), or they succeed, but they don't really
have a value to return
to describe the success in any way.

So what do we do in those cases ?

Let's create the basic following method:

```
fn validate_email(email: String) -> Result<???, Error> {
  if email.contains("@") {
    // Success!
    Ok() // <= 'Ok()' with empty parenthesis will have an error (because an argument is required)
  } else {
    Err(Error::other("Invalid email"))
  }
}
```

'Ok()' with empty parenthesis will have an error (because an argument is required).

The convention is to put an empty tuple `()` as argument, which gives us: `Ok(())`, and `-> Result<(), Error>`.

Likewise, in the Match Statement, by convention, since we don't care about the value we're going to receive from this `Ok()` variant and we don't want to assign a variable (since we don't ever want to use it),
we just put `..`.

```
match validate_email(String::from("test@test.com")) {
    Ok(..) => println!("Email is valid"),
    Err(reason_validation_failed) => {
        println!("{}", reason_validation_failed);
    }
}
```

### Side example on Tuples

Suppose we make a tiny app to represent colors.

One way to represent the RGB values is to make a `Rgb` Struct, which has `red`, `blue`, and `green` fields.
<br/>
We'd also have a function `make_rgb()` which returns us an instance of `Rgb`. We can use it in our `main()` function however we want.
```
struct Rgb {
  red: u8,
  green: u8,
  blue: u8,
}

fn make_rgb() -> Rgb {
  Rgb {
    red: 0,
    green: 255,
    blue: 128,
  }
}

fn main() {
  let rgb = make_rgb();
  let red = rgb.red;
}
```

---

We can also represent RGB values using a slightly different data structure: a tuple.

A tuple can be thought of as an *array* of values that has a fixed length, where every element has a very specific meaning.

The only real difference here between a `struct` and a `tuple` is that we don't really have **labels** on these individuals positions.
- In a `struct`, we have labels on each field (we clearly see each field meaning)
- In a `tuple`, we still have as many values (3 in this example), but those values are not explicitely labeled
  - Instead, as developers, we have to remember what each position (or each index inside this array) is meant to represent
    - Therefore, in this example, we might decide that the first element is to represent *red*, the second *green* and the third *blue*
```
// Aka ('this element represents red', 'this one represents green', 'this one represents blue')
type Rgb = (u8, u8, u8);

fn make_rgb() -> Rgb {
  Rgb (0, 128, 255)
}

fn main() {
  let rgb = make_rgb();
  let red = rgb.0;
}
```

## Ways to Handle a Result<>
### A) Using Match Statements
It can very quickly be messy.
```
    // fs::read_to_string return a Result
    match fs::read_to_string("logs.txt") {
        Ok(text_that_was_read) => {
            // 'extract_errors_fix()' definition in Section_07-01_Logs
            let errors_string_slice = extract_errors_fix(text_that_was_read.as_str());

            // fs::write return a Result
            match fs::write("errors.txt", errors_string_slice.join("\n")) {
                Ok(..) => println!("Wrote errors.txt"),
                Err(writing_to_file_failed) => {
                    println!("Failed to write to file: {}", writing_to_file_failed);
                }
            }
        }
        Err(why_this_failed) => {
            println!("Failed to read file: {}", why_this_failed);
        }
    }
```

### B) Using Result Enum Methods
Many of the Option Enum methods works with the Return Enum (check the Section_05-02_Project_Media_Cleanup's README.md file)
#### 1. `.unwrap()`
``` 
text.unwrap()
```
- If `text` is an `Ok`, returns the value in the `Ok`.
- If `text` is an `Err`, panics!
  - **Use for quick debugging or examples**

---
#### 2. `.expect(...)`
``` 
text.expect("Couldn't open the file")
```
- If `text` is an `Ok`, returns the value in the `Ok`.
- If `text` is an `Err`, prints the provided debug message and panics!
  - Use when we **want** to crash if something goes wrong

---
#### 3. `.unwrap_or(&...)`
``` 
text.unwrap_or(
  String::from("Backup text")
)
```
- If `text` is an `Ok`, returns the value in the `Ok`.
- If `text` is an `Err`, returns the provided default value
  - Use when it makes sense to provide a fallback value

