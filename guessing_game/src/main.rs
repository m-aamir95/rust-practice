use std::io;
use std::cmp::Ordering;
use rand::{self, Rng};

fn main() {
    println!("Please Enter A Number between 1-10:");

    let mut guess : String = String::new();
    let random_num = rand::thread_rng().gen_range(1..=10);

    io::stdin().read_line(&mut guess)
        .expect("Unable to read a guess");
    


    //Parse to int
    let guess : u32 = guess.trim().parse().expect("Please enter a number");

    match guess.cmp(&random_num){
        Ordering::Less => println!("Your guess is low"),
        Ordering::Greater => println!("Your guess is high"),
        Ordering::Equal => println!(">>> Congratulations you have guessed the number <<<")
    }
}
