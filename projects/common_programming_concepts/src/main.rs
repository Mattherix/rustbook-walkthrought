use std::f64::consts::TAU;

fn main() {
    // Variable and Mutability
    let a = 5;
    let mut b = 6;
    // I can b
    b *= 6;
    println!("The value of b is {} and a {}", b, a);
    // I can't change a but I can shadow it
    let a = 7;
    println!("The value of a is {}", a);
    // I can create constant but I *must* annotate them
    const QUADRUPLE_PI: f64 = 2.0 * TAU;
    // Btw happy PI day ! (look at the commit date)

    // Data types
    // We must annotate a variable if it can have multiple type
    // i.e. with the parse function. Else the compiler will deduce it
    let value: u32 = "42".parse().expect("Can't parse this value");
    // Scalar type (single value): integer, float, bool or char
    // 8-bit	i8	u8
    // 16-bit	i16	u16
    // 32-bit	i32	u32
    // 64-bit	i64	u64
    // 128-bit	i128	u128
    // arch	isize	usize
    // Can use _ in number, octal, char, ...
    // Integer overflow: in debug -> panic in release wrap around to minimum
    // Floating point, pretty usual
    // /!\ if you divided 2 integers it will output only an integer, no f32/f64
    // Just a bool: true or false
    // Char, use utf8, can use emoji

    // Compound type (multiple value in one): tuple or array
    // Tuple, static different type.  can access with tuple.index
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x={}, y={}, z ={} or {} {} {}", x, y, z, tup.0, tup.1, tup.2);

    // Array, static multiple type
    let list = [1, 2, 3, 4, 5];
    let months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    // Use using a for loop
    for num in list {
        println!("{num}");
    }
    // Or with the index
    println!("{}", months[3]);
    // Or use iterator (chapter 13) most common in the rust world

    // Functions
    fn sum(a: i32, b: i32) -> i32 {
        // Not a + b; else it became a statement from expression
        a + b
    }
    println!("{a} + {b} = {}", sum(a, b));
    // Comments
    //\! Documentation comment (read chapter 14)
    // The are check at compile time,
    // it won't compile if they are misplaced

    // Controls flow
    // If else
    let number = 3;
    if number > 5 {
        println!("number is over 5");
    } else {
        println!("number is under 5");
    }

    let n = if number < 6 { 7 } else { 3 };
    println!("n is {n}");

    // Return a value from an infinite loop (for operation that might panic!)
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    // While loop
    while counter != 0 {
        counter -= 1;
    }

    // For loop
    for countdown in (1..4).rev() {
        println!("{}", countdown);
    }
    println!("Launch !!");
}
