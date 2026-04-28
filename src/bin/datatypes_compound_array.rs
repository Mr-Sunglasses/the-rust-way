#![allow(dead_code, unused)]

use std::io;

fn main() -> () {
    // Compund data Arrays
    // Arrays: fixed size (fixed length) storage box with same data type
    // Unlike tuples, they have same data type

    let a = [1, 2, 3, 4, 5, 6];
    let b: [&str; 2] = ["Hello", "World"];

    // array is fixed size and it is stored in stack, to store data in heap use vector
    // also array is useful when we know how much data we need to store and of what type

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // initialize array with the same value syntax: let varname = [value; times]
    let best_friend = ["ishu"; 5];

    println!("{:?}", best_friend);

    // accessing the elements of an array
    let my_array = [1, 2, 3, 4, 5];

    let first = my_array[0];

    let second = my_array[1];

    println!("Print Third: {:?}", my_array[2]);

    // Invalid Array Element Access
    // when you try to access element out of bound lenght it stops the program and does not exectute final println!
    // this ensures memory safety

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = my_array[index];

    println!("The value of the element at index {index} is: {element}");
}
