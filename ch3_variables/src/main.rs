fn main() {


    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    println!("THREE_HOURS_IN_SECONDS: {THREE_HOURS_IN_SECONDS}");


    // Shadowing

    let y = 5;

    println!("The value of y is: {y}");

    let y = y + 1;

    println!("The value of y is: {y}");

    {
        let y = y + 1;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");


    let spaces = "   ";
    println!("spaces=[{spaces}]");
    let spaces = spaces.len();
    println!("spaces={spaces}");


    // compile error
    // mut variable cannot change type
    // let mut spaces = "   ";
    // spaces = spaces.len();

}