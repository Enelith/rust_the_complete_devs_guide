fn main() {
    let mut colors = vec![
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
    println!(" > Using 'for loops' with iterators");
    print_elements_1(&colors);

    // 08.86 Using iterator consumers
    println!(" ---------------- ");
    println!(" > Using iterator consumers");
    print_elements_2(&colors);

    // 08.87 Using iterator adapters & consumers
    println!(" ---------------- ");
    println!(" > Using iterator adapters & consumers");
    print_elements_3(&colors);

    // 08.89 Reminder on Ownership and Borrowing
    println!(" ---------------- ");
    shorten_strings(&mut colors[2..3]);
    println!("{:#?}", colors);
    /*
    Will output:
    [
        "red",
        "green",
        "b",
    ]
     */

    // 08.92 Collecting elements from an Iterator
    println!(" ---------------- ");
    let uppercased = to_uppercase(&colors);
    println!("{:#?}", uppercased);
    /*
    Will output:
    [
        "RED",
        "GREEN",
        "B",
    ]

     */

}

fn print_elements_1(elements: &Vec<String>) {
    for element in elements {
        println!("{}", element);
    }
}

fn print_elements_2(elements: &Vec<String>) {
    // Using a 'closure' inside the 'for_each' function which will be called for each element:
    // a 'closure' is an anonymous function (that has no name assigned to it)
    // To create a 'closure' in Rust, we put in 2 pipe symbols '||', and we're receiving our arguments in between those 2 pipes.
    // Right after those pipes is our function body.
    elements.iter().for_each(|element| println!("{}", element));
}

fn print_elements_3(elements: &Vec<String>) {
    elements
        .iter()
        .map(|el| format!("{} {}", el, el))
        .for_each(|element| println!("{}", element));
}

// Note that using [String] instead of Vec<String> allows you to pass a portion of a vector (Vec<String> would expect you to pass the whole vector)
fn shorten_strings(elements: &mut [String]) {
    // Solution with a bug
    elements.iter_mut()
        .for_each(|element| element.truncate(1));
    // You'll get an on 'element': Cannot borrow immutable local variable `element` as mutable
}

fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements.iter()
        .map(|el| el.to_uppercase())
        .collect()
}
