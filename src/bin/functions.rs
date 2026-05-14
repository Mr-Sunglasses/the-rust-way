#![allow(dead_code, unused)]

// rust use snake case in function

fn main() -> () {
    print_labeled_measurements(12, 'g');
}

fn print_labeled_measurements(measurements: i32, unit_lable: char) -> () {
    println!("The measurements is: {}{}", measurements, unit_lable);
}

// return by expression
fn add_expression(a: i32, b: i32) -> i32 {
    a + b
}

fn add_by_return_statement(a: u8, b: u8) -> u8 {
    let sum: u8 = a + b;

    return sum;
}
