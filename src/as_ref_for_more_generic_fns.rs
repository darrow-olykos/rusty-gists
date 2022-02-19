pub fn as_ref_for_more_generic_fns() {
    let pointer_to_str_on_heap: Box<str> = String::from("my_string").into_boxed_str();
    let pointer_to_static_string = "my_string";
    let pointer_to_str_on_heap_with_capacity_and_resizability = String::from("my_string");

    // strlen can accept all three of these "strings" since they can all be easily converted to a reference to a str
    println!(
        "{} {} {}",
        strlen(pointer_to_str_on_heap),
        strlen(pointer_to_static_string),
        strlen(pointer_to_str_on_heap_with_capacity_and_resizability)
    ); // => 9 9 9
}

fn strlen(s: impl AsRef<str>) -> usize {
    s.as_ref().len()
}
