#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
}

fn print_media(media: Media) {
    println!("{:?}", media);
}

fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("audiobook")
    };

    print_media(audiobook);
}
