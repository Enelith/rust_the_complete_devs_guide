fn main() {
    let languages = vec![
        String::from("rust"),
        String::from("go"),
        String::from("typescript"),
    ];


    let result = next_language(&languages, "go");
    println!("Next language: {:#?}", result);

    println!(" --------------- ");
}

// As a reminder, &[String] allows you to put the full vector as argument, or just a portion of it. &Vec<String> expects the full vector only.
fn next_language(languages: &[String], current: &str) -> &str {
    let mut found = false;

    for lang in languages {
        if found {
            return lang;
        }
        if lang == current {
            found = true;
        }
    }

    // Returning the very last element of the vector by default.
    // For simplicity, we're going to assume 'languages.last()' will ALWAYS return an Option.Some variant, and not an Option.None.
    languages.last()
        .unwrap()
}