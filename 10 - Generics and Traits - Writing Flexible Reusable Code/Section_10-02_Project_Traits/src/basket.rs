// Note: adding the #[derive(Debug)] will make things a bit more complicated with generics

pub struct Basket<T> {
    item: Option<T>,
}

impl<T> Basket<T> {
    pub fn new(item: T) -> Self {
        Basket {
            item: Some(item),
        }
    }

    pub fn get(&mut self) -> Option<T> {
        // .take(): Takes the value out of the option, leaving a None in its place.
        self.item.take()
    }

    pub fn put(&mut self, item: T) {
        self.item = Some(item);
    }

    pub fn is_empty(&self) -> bool {
        // .is_none(): Returns true if the option is a None value.
        self.item.is_none()
    }
}