# Section_08-01_Project_Iter

We're going to work on a very kind of simple example here

```
let colors = vec![
    String::from("red"),
    String::from("green"),
    String::from("blue"),
];
```

| Name                | Description                                          |
|:--------------------|:-----------------------------------------------------|
| `shorten_string()`  | Shortens each string in the vector to 1 character    | 
| `move_elements()`   | Moves elements from the given vector to a new vector | 
| `print_elements()`  | Prints each element in the vector one by one         | 
| `to_uppercase()`    | Return a new vector with each element capitalized    | 
| `explode()`         | Turns a Vec<String> into a Vec<Vec<String>>          | 
| `find_color_or()`   | Finds a matching element or returns a fallback       | 

### Shorten_strings
- `shorten_strings()` should modify the strings in the vector
- We do *NOT* want to create a new vector

Strings in Rust have a method tied to them called `truncate`.

- **truncate()** : Modifies a string in place
```
fn main() {
    let mut color = String::from("blue");
    
    color.truncate(1); // color becomes just "b"
}
```