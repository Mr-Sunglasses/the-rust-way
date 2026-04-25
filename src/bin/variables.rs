#![allow(dead_code, unused)]
const PI: f32 = 3.1418;
const SECONDS_IN_AN_HOUR: u32 = 60 * 60; // mintues in hours = 60 and seconds in a minute 60

fn main() {
    // variables are immutable by default
    let var = "myvar".to_string();
    println!("{}", var);

    // mutable variable
    let mut mutvar = String::new();
    mutvar.push_str("This is a new mutable var");
    println!("{mutvar}");

    println!("shadowing a variable");
    // but it can be shadowed
    let var = "New String".to_string();

    {
        // here var is dropped from the memory when it is out of scope.
        let var = "Some String".to_string();
        println!("Inside brackets: {var}");
    }

    println!("{}", var);

    // shadowing vs mut = in shadowing we can change the variable type but in mut we can only change it's value.
    let mut string_m = String::from("This is a String of Len X");
    // string_m = string_m.len(); // error mismached type, ie. String != usize
    let string_m = string_m.len();
}
