# Section_10-02_Project_Traits

We're going to have two separate goals in this application.

- The first goal is to make a `Basket` struct.
    - We want this struct to be able to hold any kind of data (for example, a String, a number, a Vec<String>, a
      Vec<Boolean>, or even other structs).
    - Along with this struct, we want to have a couple of methods attached to it.

| Name       | Args           | Return    | Description                                                                                                                               |
|:-----------|:---------------|:----------|:------------------------------------------------------------------------------------------------------------------------------------------|
| `get`      | -              | Option<_> | Returns the value contained by the basket wrapped in an Option (None if the basket had nothing)                                           | 
| `put`      | Value to store | -         | Stores a value, replacing whatever the basket stores. <br/><br/>**If the basket is storing a number, add the new value to the existing.** | 
| `is_empty` | -              | bool      | True if the basket is empty                                                                                                               | 

- The second goal is to make a `Stack` struct that holds as much data as needed
    - This struct should also be able to hold any type of data
    - It's going to have the same methods as the `Basket` struct

| Name       | Args           | Return    | Description                                                                    |
|:-----------|:---------------|:----------|:-------------------------------------------------------------------------------|
| `get`      | -              | Option<_> | Returns the value most recently added to the stack, None if the stack is empty | 
| `put`      | Value to store | -         | Stores a value                                                                 | 
| `is_empty` | -              | bool      | True if the stack is empty                                                     | 

Therefore, 
- `Basket` and `Stack` methods work differently but have the same signature
- We can define those methods in a trait, then have each struct implement that trait
- Benefit: throughout our app, we can work with a `Basket` or `Stack` by using generics and **trait bounds**

Here's an example of how it could look like: 
```
// Generic function, T can be anything that implement the 'Container' trait
fn add_string_to_container<T: Container>(container: T, s: String) {
    container.put(s);
}

fn main() {
    let basket = Basket::new();
    add_string_to_container(
        basket,
        String::from("hi")
    );
    
    let stack = Stack::new();
    add_string_to_container(
        stack,
        String::from("hi too")
    );
}
```