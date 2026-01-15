use std::io::Error;

// Todo: add in a return type
fn validate_ingredients(ingredients: &Vec<String>) {
    if ingredients.len() > 3 {
        // Todo: make it clear that this is an error!
    } else {
        // Todo: make it clear that the ingredidents passed
        // validation. Note that we don't really have any value
        // to include in the 'Ok' here...
    }
}

fn main() {
    let ingredients = vec![
        String::from("Cheese"),
        String::from("Tomatoes"),
        String::from("Peppers"),
        String::from("Olives"),
    ];

    // Todo: validation is an operation that might succeed or fail
    // Print out a success or fail message based on whether
    // it passes validation
    validate_ingredients(&ingredients);
}







