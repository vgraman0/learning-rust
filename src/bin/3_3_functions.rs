fn main() {
    println!("hello world!");

    another_function();

    let x = 5;
    print_x(x);

    //RAII, x is freed outside of the y scope
    let y = {
        let x = 3;
        x + 1
    };
    // this should print 4
    println!("the value of y is {y}"); 
}

fn another_function() {
    println!("another function");
}

// must declare the types for function definitions 
// but that means they are not needed elsewhere.
fn print_x(x: i8) {
    println!("The value of x is {x}");
}

fn five() -> i32 {
    5
}