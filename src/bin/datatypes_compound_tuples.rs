#![allow(dead_code, unused)]

fn main() -> () {
    // compound data types: tuples and list

    //tuples: fixed size storage with multiple values of different data types

    let datatypes_tuple: (i8, u8, f32, bool, char, (i16, bool)) =
        (-12, 12, 12.4, false, 'f', (-113, false));

    // you can assign values to a variable using tuples
    let (a, b, c, d, e, f) = datatypes_tuple;

    // or using variable.index
    let first = datatypes_tuple.0;

    let last = datatypes_tuple.5;

    println!("{:?}", last);

    // unit tuple -> tuple without any value
    let unit_tuple: () = ();

    println!("{:?}", unit_tuple);

    // it was written in rust book: Expressions implicitly return the unit value if they don’t return any other value

    // expression
    // let x = 5; // this is an expression and it retrun its return value which is i32 ( also this is not technically a expression)
    let x = {
        let y = 5;
        y
    };
    println!("x: {x}");

    // statement
    let y = {
        // semicolon kills the return value
        let a = 22; // here it is a statement and it does not have a final expression, ; ended the scope and it return nothing so
        // : Expressions implicitly return the unit value if they don’t return any other value
    };

    println!("Expression: {}", x);
    println!("Statement: {:?}", y);

    let return_a_value = {
        let a = 44;
        a // <- no semicolon = expression
    };

    println!("Return a value: {return_a_value}");
}
