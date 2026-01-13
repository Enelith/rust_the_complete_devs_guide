#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
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
            _ => String::from("Media description"),
        }
    }
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

    print_media(audiobook);
    print_media(good_movie);
    print_media(bad_book);
}
