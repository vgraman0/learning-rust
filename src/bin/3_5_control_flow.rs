fn main() {
    let number = 3;

    if number != 0 {
        println!("number was not zero");
    }

    let mut number = if true {5} else {6};
    println!("{number}");

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    let a = [1, 2, 3, 4, 5];
    let mut index = 0;

    for e in a {
        println!("{e}");
    }

    let a = 1..4;
}