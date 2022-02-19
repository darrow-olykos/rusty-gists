pub fn as_ref_for_more_generic_fns() {
    let boxed_str: Box<str> = String::from("my_string").into_boxed_str();
    let static_str = "my_string";
    let string = String::from("my_string");

    // strlen can accept all three of these "strings" since they can all be easily converted to a reference to a str
    println!(
        "{} {} {}",
        strlen(boxed_str),
        strlen(static_str),
        strlen(string)
    ); // => 9 9 9
}

fn strlen(s: impl AsRef<str>) -> usize {
    s.as_ref().len()
}
