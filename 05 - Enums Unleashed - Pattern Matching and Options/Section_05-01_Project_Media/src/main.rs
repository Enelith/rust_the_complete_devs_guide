#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(u32),
    Placeholder,
}

impl Media {
    fn description_1(&self) -> String {
        // This is a very basic type check
        if let Media::Book { title, author } = self {
            // if we have a book
            format!("Book: {} {}", title, author)
        } else if let Media::Movie { title, director } = self {
            // if we have a movie
            format!("Movie: {} {}", title, director)
        } else if let Media::Audiobook { title } = self {
            // if we have an audiobook
            format!("Audiobook: {}", title)
        } else {
            String::from("Media description")
        }
    }

    fn description_2(&self) -> String {
        match self {
            Media::Book { title, author } => {
                format!("Book: {} {}", title, author)
            }
            Media::Movie { title, director } => {
                format!("Movie: {} {}", title, director)
            }
            Media::Audiobook { title } => {
                format!("Audiobook: {}", title)
            }
            Media::Podcast(episode_number) => {
                format!("Podcast: {}", episode_number)
            }
            Media::Placeholder => "Placeholder".to_string(), //            _ => String::from("Media description"),
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        // Catalog { items: Vec::new() }
        Catalog { items: vec![] }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);
    }

    // This implementation will crash if index > catalog.items.len() (length)
    fn get_by_index_faulty(&self, index: usize) -> &Media {
        //self.items[index] // Warning: this will return the OWNERSHIP, which is not something we want to do
        &self.items[index]
    }

    // Fixed version, which will use an emulation of the Option Enum to simulate the null/nil/undefined values
    fn get_by_index_with_custom_enum(&self, index: usize) -> MightHaveValue {
        if self.items.len() > index {
            // Good case
            MightHaveValue::HasValueAvailable(&self.items[index])
        } else {
            // Bad case
            MightHaveValue::NoValueAvailable
        }
    }

    // Fixed version using the built-in Option enum
    fn get_by_index_with_option(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            // Good case
            Some(&self.items[index])
        } else {
            // Bad case
            None
        }
    }
}

#[derive(Debug)]
enum MightHaveValue<'a >{
    // HasValueAvailable(&Media), // <= Missing lifetime specifier [E0106]
    HasValueAvailable(&'a Media), // for now, we're adding the lifetime specifier "' a" and we'll explain it in later lesson
    NoValueAvailable,
}

fn print_media(media: Media) {
    println!("{:?}", media);
}

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

    // Calling common method on each Media type
    println!("{{description_1}} {}", audiobook.description_1());
    println!("{{description_1}} {}", good_movie.description_1());
    println!("{{description_1}} {}", bad_book.description_1());

    println!("{{description_2}} {}", audiobook.description_2());
    println!("{{description_2}} {}", good_movie.description_2());
    println!("{{description_2}} {}", bad_book.description_2());

    /*
    print_media(audiobook);
    print_media(good_movie);
    print_media(bad_book);
     */

    let podcast = Media::Podcast(10);
    let placeholder = Media::Placeholder;

    let mut catalog = Catalog::new();
    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    println!("{{catalog}} {:#?}", catalog);

    // #53 Option Enum
    // Will return "Some(...)" or "None"
    println!("{:#?}", catalog.items.get(0));

    match catalog.items.get(0) {
        Option::Some(value) => {
            println!("Item: {:#?}", value);
        }
        Option::None => {
            println!("Nothing at that index");
        }
    }

    match catalog.items.get(100) {
        Option::Some(value) => {
            println!("Item: {:#?}", value);
        }
        Option::None => {
            println!("Nothing at that index");
        }
    }

    // #54 Option from another perspective
    let item_at_0 = catalog.get_by_index_faulty(0);
    println!("{:#?}", item_at_0);
    let item_at_4 = catalog.get_by_index_faulty(4);
    println!("{:#?}", item_at_4);
    // Will crash
    /*
    thread 'main' (8512) panicked at src\main.rs:66:20:
        index out of bounds: the len is 5 but the index is 40
    stack backtrace:
    0: std::panicking::panic_handler
     */
    //let item_at_40 = catalog.get_by_index_faulty(40);
    //println!("{:#?}", item_at_40);

    let item_with_custom_enum = catalog.get_by_index_with_custom_enum(40);
    println!("{:#?}", item_with_custom_enum);
    // Match Statement
    match item_with_custom_enum {
        MightHaveValue::HasValueAvailable(value) => {
            println!("(Matching Statement > MightHaveValue::HasValueAvailable) Item: {:#?}", value);
        }
        MightHaveValue::NoValueAvailable => {
            println!("(Matching Statement > MightHaveValue::NoValueAvailable) Nothing at that index");
        }
    }

    // Pattern Matching
    if let MightHaveValue::HasValueAvailable(value) = item_with_custom_enum {
        println!("(Pattern Matching > if let MightHaveValue::HasValueAvailable) Item: {:#?}", value);
    } else if let MightHaveValue::NoValueAvailable = item_with_custom_enum {
        println!("(Pattern Matching > if let MightHaveValue::NoValueAvailable) Nothing at that index");
    }

    // #55 Replacing Custom Enum with Option
    let item_with_option = catalog.get_by_index_with_option(40);
    println!("{:#?}", item_with_option);
    // Match Statement
    match item_with_option {
        Some(value) => {
            println!("(Matching Statement > Option::Some) Item: {:#?}", value);
        }
        None => {
            println!("(Matching Statement > Option::Some) Nothing at that index");
        }
    }

    // Pattern Matching
    if let Some(value) = item_with_option {
        println!("(Pattern Matching > if let Option::Some) Item: {:#?}", value);
    } else if let None = item_with_option {
        println!("(Pattern Matching > if let Option::None) Nothing at that index");
    }
}
