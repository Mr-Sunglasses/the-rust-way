#![allow(dead_code, unused)]

use std::io::{self, Read};

fn main() {
    // scalar types - represent only single values
    // - integer, floating-point-number, booleans and characters

    // integer type -> default i32
    // prefix 'i' - include +ve and -ve values, ex i8 = -2^n-1 to 2^n-1 - 1  = -2^8-1 to (2^8-1) - 1 = -128 to 127
    // prefix 'u' - include +ve values, ex u8 = 0 to 2^n - 1 = 0 to 2^8-1 = 0 to 255
    // bit starts from 8 to 128 both signed and unsigned

    let i_small_number: i8 = -127;
    let u_small_number: u8 = 0;

    // i/u[size] size of the integer depends on the os, 32 or 64 bit, used when we rely on us to set the max size and you need to index some sort of collection
    let any_os_number: usize = 1123;

    // ways to write integer literals
    let suffix_denomination = 33u8;

    let decimal_denomiation = 10_00_000;

    let hex_denomination = 0xff;

    let octal_denomination = 0o77;

    let binary_denomination = 0b1111_00_11_00;

    let byte_denomination = b'A'; // u8 only

    // integer overflow
    // When integer goes to max or min limit for example at runtime u8 go out of 255 or 0 range
    // in debug mode -> panic and in release mode = 256 -> 0 , 257 -> 1 etc.
    let mut my_addition_number: u8 = 12;

    let mut guess = String::new();

    println!("Please write a number to be added to: {guess}");

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read the input");

    let guess: u8 = guess.trim().parse().expect("failed to parse NaN");

    my_addition_number = my_addition_number + guess;

    println!("The total of 12 + {guess} = {my_addition_number}");
}
