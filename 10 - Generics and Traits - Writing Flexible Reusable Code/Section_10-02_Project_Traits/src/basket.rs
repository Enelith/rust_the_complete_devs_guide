// Note: adding the #[derive(Debug)] will make things a bit more complicated with generics

use super::container::Container;

// Basket is a Generic Struct
pub struct Basket<T> {
    item: Option<T>,
}

impl<T> Basket<T> {
    pub fn new(item: T) -> Self {
        Basket { item: Some(item) }
    }

    // Note: Because we're going to implement a trait, we can't add methods to it,
    // and since the 'new()' method wasn't part of the trait Container,
    // we need to move the methods get, put and is_empty in another block.
}

// The methods get, put and is_empty are going to be public by default when implementing a trait,
// so we can remove the 'pub' in front of each method.
impl<T> Container<T> for Basket<T> {
    fn get(&mut self) -> Option<T> {
        // .take(): Takes the value out of the option, leaving a None in its place.
        self.item.take()
    }

    fn put(&mut self, item: T) {
        self.item = Some(item);
    }

    fn is_empty(&self) -> bool {
        // .is_none(): Returns true if the option is a None value.
        self.item.is_none()
    }
}
