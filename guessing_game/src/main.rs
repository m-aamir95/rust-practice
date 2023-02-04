use std::io;

fn main() {
    println!("Please Enter A Number :");

    let mut guess : String = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Unable to read a guess");


    println!("You have entered {guess}");
}
