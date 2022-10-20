use std::io;

fn main(){
    println!("Guess The Number Game");

    println!("Please Enter the Input You want to Guess");

    let mut guess = String::new();
    println!("The Value of Guess is now {guess}");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to Guess the input");
    


    println!("You have Guess the input: {guess}");
    

    
}

