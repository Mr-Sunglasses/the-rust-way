#![allow(unused)]
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    // println!("What is Your Name");
    // let mut name = String::new();
    // let greeting = "Nice to Meet You";
    // io::stdin().read_line(&mut name).expect("Didn't recieve the Input");
    // println!("Hello World {} {}", name.trim_end(), greeting);

    const ONE_MILLION: u32 = 1_000_000;
    const PI: f32 = 3.145643;
    let age = "47";
    // Shadowing
    let mut age: u32 = age.trim().parse().expect("Age Wasn't a number");
    age = age + 1;
    println!("I am {} and I want {}", age, ONE_MILLION);

    let random = rand::thread_rng().gen_range(1..101);
    println!("{}",random);

    let number = 46;
    if (number % 2 == 0){
        println!("Even");
    }else if (number == 47) {
        println!("Fourty Seven");
    }else {
        println!("Odd");
    }

    // Ternary
    let age = 45;
    let isvote: bool = if age >= 18 {
        true
    }else {
        false
    };
    println!("{}",isvote);

    let important_birthday = 82;
    match important_birthday{
        1..=18 => println!("Important Birthday Kid"),
        19..=i32::MAX => println!("Too Important Birthday"),
        _ => println!("Not an Important Birthday")
    };
    let my_age = 20;
    let voting_age = 18;
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("Not Eligible Kid"),
        Ordering::Equal => println!("Yay You got the right to Vote"),
        Ordering::Greater => println!("You can Vote"),
        _ => println!("Voted")
    }
    let array: [i32; 9] = [1,2,3,4,5,6,7,8,9];
    println!("Array at 0th {}", array[0]);
    println!("Length of Array {}", array.len());
    let mut index_zero = 0;
    loop {
        if array[index_zero] % 2 == 0{
            index_zero +=1;
        }
        if array[index_zero] == 9 {
            break
        }
        println!("Even val: {}", array[index_zero]);
        index_zero+=1;
    }
    while index_zero < array.len() {
        println!("Val: {}", array[index_zero]);
        index_zero+=1;
    }
    for val in array.iter(){
        println!("Val: {}", val);
    }

    let my_tuple: (u32, String, f32) = (47, "Kanishk".to_string(), 123.33);
    println!("{}", my_tuple.1);
    let(v1, v2, v3) = my_tuple;
    println!("{}", v2);
}

