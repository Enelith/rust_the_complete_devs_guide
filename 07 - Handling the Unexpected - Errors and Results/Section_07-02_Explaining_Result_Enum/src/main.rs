use std::io::Error;

fn main() {
    // Will return the result: 1.666667
    match divide(5.0, 3.0) {
        Ok(result_of_division) => {
            println!("{}", result_of_division);
        }
        Err(what_went_wrong) => {
            println!("{}", what_went_wrong);
        }
    }

    // Will return an error: Can't divide by 0
    match divide(5.0, 0.0) {
        Ok(result_of_division) => {
            println!("{}", result_of_division);
        }
        Err(what_went_wrong) => {
            println!("{}", what_went_wrong);
        }
    }

    match validate_email(String::from("test@test.com")) {
        Ok(..) => println!("Email is valid"),
        Err(reason_validation_failed) => {
            println!("{}", reason_validation_failed);
        }
    }

    match validate_email(String::from("testtest.com")) {
        Ok(..) => println!("Email is valid"),
        Err(reason_validation_failed) => {
            println!("{}", reason_validation_failed);
        }
    }
}

fn divide(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        Err(Error::other("Can't divide by 0"))
    } else {
        Ok(a / b)
    }
}

fn validate_email(email: String) -> Result<(), Error> {
    if email.contains("@") {
        // Success!
        Ok(())
    } else {
        Err(Error::other("Invalid email"))
    }
}
