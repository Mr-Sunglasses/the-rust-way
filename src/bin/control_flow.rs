#![allow(dead_code, unused)]

fn main() -> () {
    // control flow
    // if expression

    let number = 10;

    if number < 5 {
        println!("True");
    } else {
        println!("False");
    }

    let is_one_or_four = 4;

    // using multiple is else is bad
    // use match

    if is_one_or_four == 1 {
        println!("One");
    } else if is_one_or_four > 1 {
        println!("this condition is true");
        // below condition is not executed as in else if if one block true it exists
        // use multiple if to do so
    } else if is_one_or_four == 4 {
        println!("Four");
    } else {
        println!("Neither one nor four");
    }

    // example with multiple if
    if is_one_or_four == 4 {
        println!("four");
    }
    if is_one_or_four > 1 {
        println!("greater than one");
    } else {
        println!("None");
    }

    // if let statemet
    let number = 4;
    let is_number_even: bool = if number % 2 == 0 { true } else { false }; // both should return same type

    // bad
    // let is_number_even = if number % 2 == 0 {
    //     true
    // } else {
    //     "No".to_string()
    // };

    println!("is number even or odd: {is_number_even}");

    let mut counter = 0;

    let result = loop {
        counter = counter + 1;

        if counter == 10 {
            counter * 2;
            continue;
        }

        if counter == 20 {
            break counter;
        }
    };

    let mut counter_one = 0;

    'counting_up: loop {}
}
