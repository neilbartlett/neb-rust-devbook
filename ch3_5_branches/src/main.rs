fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // no automatic type conversion to boolean
    // the following would give an error
    // if number {
    //     println!("number was {number}");
    // }


    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    } 

    // we can use if in a let statement to return an expression based on a condition
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // both arms of the if expression must return the same type
    // the following would give an error
    // let number = if condition { 5 } else { "six" };
    // println!("The value of number is: {number}");

    // Rust has three kinds of loops: loop, while, and for
    // loop is the most flexible note to break out of a loop we can use break
    // we can also return a value from a loop using break
    // the following loop will return 20
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // loops can be nested. A break only works on the innermost loop
    // however we can label loops and break to a label

    'counting_up: loop {
        println!("counting up");

        'counting_down: loop {
            println!("counting down");

            break 'counting_up;
        }
    }

    // while loops are similar to other languages
    // note no need to put parentheses around the condition
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    // for loops can use iterators
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // we can also use for loops to iterate over a range
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

}