/* rust is statically typed. we must know all of the types are compile time.

Compiler can usually infer what type we want to use based on the value and how we use it.

*/

use std::mem;

fn main() {
    // if multiple types are possible, need a type annotation.
    let guess: u32 = "42".parse::<u32>().expect("Not a number");
    println!("{guess}");

    // Scalar types
    /* Integer types!
        - signed types start with i (i8, i16, i32, ..., isize) -2^{n-1} to 2^{n-1} - 1
        - unsigned types start with u (u8, u16, u32, ..., usize), 0 to 2^n - 1*/ 

    /* Messing around with integer overflows

    let k = 10;
    let n = 1;
    let x: i8 = n << k;
    println!("{x}");
    */

    /* Floating point:
        f32, f64
        IEEE-754: Sign bit(0 positive, 1 negative) , Exponent (scale), Mantissa (Precision)
        Val = (-1)^sign * (1 + mantissa) * 2^{exponent-B}
     */

    // let x = 2.0;
    // let y: f32 = 3.0;

    let quotient = 56.7/32.2;
    println!("{quotient}");

    /* Compound types */

    // similar to structs: we have padding to avoid fragmentation.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let size_of_var = mem::size_of_val(&tup);
    println!("{size_of_var}");

    let a = [1, 2, 3, 4, 5];

}

