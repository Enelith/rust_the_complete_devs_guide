mod content;

use content::catalog::Catalog;
use content::media::Media;

/*
fn print_media(media: Media) {
    println!("{:?}", media);
}
 */

fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("audiobook"),
    };
    let good_movie = Media::Movie {
        title: String::from("Good Movie"),
        director: String::from("Good Director"),
    };
    let bad_book = Media::Book {
        title: String::from("Bad Book"),
        author: String::from("Bad Author"),
    };

    let podcast = Media::Podcast(10);
    let placeholder = Media::Placeholder;

    let mut catalog = Catalog::new();
    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    let item_unwrap = catalog.get_by_index(40);
    let item_expect = catalog.get_by_index(40);
    let item_unwrap_or  = catalog.get_by_index(40);
    let placeholder = Media::Placeholder;

    /*
    Will generate:
    thread 'main' (16332) panicked at src\main.rs:89:35:
    called `Option::unwrap()` on a `None` value
    stack backtrace: ...
     */
    println!("{:#?}", item_unwrap.unwrap()); // Will panic

    /*
    Will generate:
    thread 'main' (8696) panicked at src\main.rs:96:35:
    There should be a value here
    stack backtrace: ...
     */
    println!("{:#?}", item_expect.expect("There should be a value here")); // Will panic but display "There should be a value here as the error message"

    /*
    Will generate:
    Placeholder
     */
    println!("{:#?}", item_unwrap_or.unwrap_or(&placeholder)); // Will return Placeholder
}
