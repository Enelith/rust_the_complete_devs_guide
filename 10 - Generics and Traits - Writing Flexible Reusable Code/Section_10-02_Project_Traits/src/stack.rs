pub struct Stack<T> {
    items: Vec<T>
}

impl<T> Stack<T> {
    pub fn new(items: Vec<T>) -> Self {
        Stack {
            items
        }
    }

    pub fn get(&mut self) -> Option<T> {
        // .pop(): Removes the last element from a vector and returns it, or None if it is empty.
        // If you'd like to pop the first element, consider using VecDeque::pop_front instead.
        self.items.pop()
    }

    pub fn put(&mut self, item: T) {
        // .push(): Appends an element to the back of a collection.
        self.items.push(item);
    }

    pub fn is_empty(&self) -> bool {
        // .is_empty(): Returns true if the vector contains no elements.
        self.items.is_empty()
    }
}