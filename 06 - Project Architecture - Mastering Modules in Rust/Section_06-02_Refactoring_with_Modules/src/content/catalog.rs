// 'super' is a reference to the parent module (aka 'content')
use super::media::Media;

// When you see the crate keyword at the beginning of a path,
// it means you are specifying an absolute path starting from the root of your own crate.
//use crate::content::media::Media;

#[derive(Debug)]
pub struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    pub fn new() -> Self {
        // Catalog { items: Vec::new() }
        Catalog { items: vec![] }
    }

    pub fn add(&mut self, media: Media) {
        self.items.push(media);
    }

    pub fn get_by_index(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            // Good case
            Some(&self.items[index])
        } else {
            // Bad case
            None
        }

        /*
        The Vec::get() method is designed for this exact purpose:
        it returns an Option<&T>, giving you Some(&value) if the index is valid
        and None if it's out of bounds.
         */
        // self.items.get(index)
    }
}