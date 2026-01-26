mod basket;
mod stack;
mod container;

use basket::Basket;
use stack::Stack;
use container::Container;

// The method will only work with Container that uses String (so in this example, b1 and s1)
fn add_string<T: Container<String>>(c: &mut T, s: String) {
    c.put(s);
}

fn main() {
    let mut b1 = Basket::new(String::from("Hi there"));
    let b2 = Basket::new(10);
    let b3 = Basket::new(true);

    let mut s1 = Stack::new(vec![String::from("hi")]);
    let s2 = Stack::new(vec![1, 2, 3]);

    add_string(
        &mut b1,
        String::from("Adding to Basket")
    );

    add_string(
        &mut s1,
        String::from("Adding to Stack")
    );

    println!("{:#?}", b1);
    println!("{:#?}", s1);
}
