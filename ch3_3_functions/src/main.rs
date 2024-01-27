fn main() {
    println!("Hello, world!");

    // note another_function is in scope because it is in the same module as main
    another_function();
 
    print_labeled_measurement(5, 'h');


    // statements vs exrepssions


    // statements do not return values
    // the following would give an error
    // let x = (let y = 6);

    // expressions return values
    let y = {
        let x = 3;

        // note the lack of a semicolon
        // if there was a semicolon, it would be a statement
        x + 1
    };

    // the following would give an error as x is not in scope    
    // println!("The value of x is: {x}");

    println!("The value of y is: {y}");

    println!("The value of five() is: {}", five());

    let x = plus_one(5);

    println!("The value of x is: {x}");
}


// RUST uses snake_case for function names
fn another_function() {
    println!("Another function.");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
 
    println!("The measurement is: {value}{unit_label}");
}

// functions with return values must specify the type of the return value
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
