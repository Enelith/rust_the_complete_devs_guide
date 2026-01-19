fn main() {
    let colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];
    // 08.84 Let's create an iterator
    let mut colors_iter = colors.iter();

    println!("{:#?}", colors_iter.next());
    println!("{:#?}", colors_iter.next());
    println!("{:#?}", colors_iter.next());
    println!("{:#?}", colors_iter.next());
    /*
    This will print:
    Some(
        "red",
    )
    Some(
        "green",
    )
    Some(
        "blue",
    )
    None
     */

    // 08.85 Using For Loops with Iterators
    println!(" ---------------- ");
    print_elements(&colors);


    println!(" ---------------- ");
}

fn print_elements(elements: &Vec<String>) {
    for element in elements {
        println!("{}", element);
    }
}