use std::io;
fn main() {

    // saturating_* does not panic on overflow, instead it returns the maximum value of the type
    assert_eq!(i32::MAX.saturating_add_unsigned(1), i32::MAX);

    // overflowing_* returns a tuple of the value of wrapped value if an overflow and a boolean indicating if an overflow occurred
    // NOTE i<num> types wrap to the minimum value not to zero.
    // cf u<num> types wrape to the minimum value (which happens to be zero for unsigned types)
    assert_eq!(i32::MAX.overflowing_add(1), (i32::MIN,true));

    // checked_* returns an Option<T> with the result if no overflow occurred or None if an overflow occurred
    assert_eq!((i32::MAX - 2).checked_add(1), Some(i32::MAX - 1));
    assert_eq!((i32::MAX - 2).checked_add(3), None);
    // cf that the following will not compile as it is not wrapped in an Option<T>
    // assert_eq!((i32::MAX - 2).checked_add(1), i32::MAX - 1);



    // tuples can use dot notation to access their fields

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("The value of five_hundred is: {}", five_hundred);
    println!("The value of six_point_four is: {}", six_point_four);
    println!("The value of one is: {}", one);


    // arrays are fixed length
    // arrays are allocated on the stack
    // note [T; N] but [T; N + 1] is a different type
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a has {} elements", a.len());
    println!("a[0] = {}", a[0]); 

    // shorthand for a repeated value
    let _a: [i32; 5] = [3; 5];

    // note default formatter does not display arrays
    // this is a compiler error
    //    print!("a = {}", a); 

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");



}
