fn main() {
    let languages = vec![
        String::from("rust"),
        String::from("go"),
        String::from("typescript"),
    ];

    let result = next_language(&languages, "go");
    println!("Next language: {}", result);

    println!(" --------------- ");

    let last_language = last_language(&languages);
    println!("Last language: {}", last_language);

    println!(" --------------- ");

    let longest_language = longest_language("typescript", "go");
    println!("Longest language: {}", longest_language);
}

// As a reminder, &[String] allows you to put the full vector as an argument, or just a portion of it. &Vec<String> expects the full vector only.
fn next_language<'a>(languages: &'a [String], current: &str) -> &'a str {
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

fn last_language(languages: &[String]) -> &str {
    languages.last()
        .unwrap()
}

fn longest_language<'a>(lang_a: &'a str, lang_b: &'a str) -> &'a str {
    if lang_a.len() > lang_b.len() {
        lang_a
    } else {
        lang_b
    }
}