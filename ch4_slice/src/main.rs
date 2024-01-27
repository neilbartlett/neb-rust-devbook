fn main() {

    let s = String::from("hello world");

    // need this is we want to s.clear() below
    // let mut s = String::from("hello world");

    let word = first_word(&s);

    // error!
    // s.clear(); // error!

    println!("the first word is: {}", word);


    let my_string = String::from("hello world");
    println!("the string is: {}", my_string);

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    println!("the first word is: {}", word);
    let word = first_word(&my_string[..]);
    println!("the first word is: {}", word);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);
    println!("the first word is: {}", word);

    let my_string_literal = "hello world";
    println!("the string is: {}", my_string);

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    println!("the first word is: {}", word);
    let word = first_word(&my_string_literal[..]);
    println!("the first word is: {}", word);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("the first word is: {}", word);

}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}