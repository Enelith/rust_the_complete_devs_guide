# Section_07-02_Explaining_Return_Enum_1

### The function definition
```
fn divide(a: f64, b: f64) -> Result<f64, Error>
```
- Here, we're saying we're to return something of type `Result`
- `Result` is an enum, which is a built-in type in Rust (there's no need to import or anything like that ~ we can directly use it the same way we would use the `Option` enum)
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
- So whenever we're making use of this `Result` enum, we're going to add in those angle brackets, and this is like *calling a function with some values*.
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