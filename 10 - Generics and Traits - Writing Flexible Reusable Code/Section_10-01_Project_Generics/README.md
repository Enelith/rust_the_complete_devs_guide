# Section_10-01_Project_Generics

In this project, we want to try to make the Pythagorean theorem as a function.

As a reminder, the Pythagorean theorem allows us to solve for some unknown value or unknown length inside a right-angled triangle.

```
The Pythagorean theorem is a fundamental principle in geometry that states the relationship between the three sides of a right-angled triangle.

The equation is:
    a² + b² = c²
Where:
• a and b are the lengths of the two shorter sides of the triangle (the "legs").
• c is the length of the longest side, which is opposite the right angle (the "hypotenuse").

In simple terms, if you square the lengths of the two shorter sides and add those squares together, the result will be equal to the square of the length of the longest side.
```

## Issues with Number Types

Note that in Rust, we have two kinds of floats: f32 and f64.
<br/>
Whenever you write a float, Rust is going to assume it's going to be a f64.

Suppose we have the following code:
```
fn solve(a: f64, b: f64) -> f64 {
    (a.powi(2) + b.powi(2)).sqrt()
}

fn main() {
    let a: f32 = 3.0;
    let b = 4.0;

    println!("{}", solve(a, b)); // <= Error on 'a'
}
```
Because we declared `a` as a f32, calling `solve(a, b)` shows an error.
```
Type mismatch [E0308]

Expected: f64
Found: f32
```
While the reason looks simple, there's an underlying issue here.

It turns out, 
1. Rust is not going to automatically convert different types of numbers into other types.
2. In addition, you can't do arithmetic between different types of numbers (i32 and i64 for example, or f32 and f64)

```
fn main() {
    let a: f32 = 3.0;
    let b: f64 = 4.0;

    a + b; // <= Error on '+', and error on 'b'
}
```
*Error on `+`*: 
```
Cannot add `f64` to `f32` [E0369]
```
*Error on `b`*: 
```
Mismatched types [E0308]

Expected: f32
Found: f64
```

So, to fix this, one thing we could do is manually convert `a` before we pass it into the function. We can convert it specifically into float 64.
<br/>
There are several ways to do it, but here are two ways to do it right away.

1. Declaring a new variable `as f64`
```
fn solve(a: f64, b: f64) -> f64 {
    (a.powi(2) + b.powi(2)).sqrt()
}

fn main() {
    let a: f32 = 3.0;
    let b = 4.0;
    
    let a_f64 = a as f64;

    println!("{}", solve(a_f64, b));
}
```

2. Using the `num-traits` crates
```
use num_traits::ToPrimitive;

fn solve(a: f64, b: f64) -> f64 {
    (a.powi(2) + b.powi(2)).sqrt()
}

fn main() {
    let a: f32 = 3.0;
    let b: f64 = 4.0;
 
    // Note that a.to_f64() returns an Option<f64> (which you'll need to unwrap in order to get its value)
    let a_f64 = a.to_f64().unwrap();

    println!("{}", solve(a_f64, b));
}
```

### Requirements

Install `num-traits`:
```
cargo add num-traits
```