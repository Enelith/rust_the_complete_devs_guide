// Note: adding the #[derive(Debug)] will make things a bit more complicated with generics

use super::container::Container;

// Stack is a Generic Struct
#[derive(Debug)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new(items: Vec<T>) -> Self {
        Stack { items }
    }

    // Note: Because we're going to implement a trait, we can't add methods to it,
    // and since the 'new()' method wasn't part of the trait Container,
    // we need to move the methods get, put and is_empty in another block.
}

// The methods get, put and is_empty are going to be public by default when implementing a trait,
// so we can remove the 'pub' in front of each method.
impl<T> Container<T> for Stack<T> {
    fn get(&mut self) -> Option<T> {
        // .pop(): Removes the last element from a vector and returns it, or None if it is empty.
        // If you'd like to pop the first element, consider using VecDeque::pop_front instead.
        self.items.pop()
    }

    fn put(&mut self, item: T) {
        // .push(): Appends an element to the back of a collection.
        self.items.push(item);
    }

    fn is_empty(&self) -> bool {
        // .is_empty(): Returns true if the vector contains no elements.
        self.items.is_empty()
    }
}
