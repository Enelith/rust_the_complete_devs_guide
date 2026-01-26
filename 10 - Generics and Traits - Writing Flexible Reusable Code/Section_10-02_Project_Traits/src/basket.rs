// Note: adding the #[derive(Debug)] will make things a bit more complicated with generics

pub struct Basket {
    item: Option<String>,
}

impl Basket {
    pub fn new(item: String) -> Self {
        Basket {
            item: Some(item),
        }
    }

    pub fn get(&mut self) -> Option<String> {
        // .take(): Takes the value out of the option, leaving a None in its place.
        self.item.take()
    }

    pub fn put(&mut self, item: String) {
        self.item = Some(item);
    }

    pub fn is_empty(&self) -> bool {
        // .is_none(): Returns true if the option is a None value.
        self.item.is_none()
    }
}