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
Note that in Rust, we have two kinds of floats: f32 and f64.
<br/>
Whenever you write a float, Rust is going to assume it's going to be a f64.

### Requirements

```
cargo add num-traits
```