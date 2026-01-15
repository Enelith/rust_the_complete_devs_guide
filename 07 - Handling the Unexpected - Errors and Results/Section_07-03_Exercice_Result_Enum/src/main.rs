use std::io::Error;

// Todo: add in a return type
fn validate_ingredients(ingredients: &Vec<String>) -> Result<(), Error> {
    if ingredients.len() > 3 {
        // Todo: make it clear that this is an error!
        Err(Error::other("Too many ingredients"))
    } else {
        // Todo: make it clear that the ingredidents passed
        // validation. Note that we don't really have any value
        // to include in the 'Ok' here...
        Ok(())
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
    let validation = validate_ingredients(&ingredients);

    match validation {
        Ok(..) => println!("Ingredients are ok"),
        Err(error) => println!("Error: {}", error),
    }
}







