use std::io;
use std::cmp::Ordering;
use rand::{self, Rng};

fn main() {
    

    let mut num_tries : u32 = 1;

    //Infinite loop until a break is triggered from inside the loop body
    loop {

        println!("Please Enter A Number between 1-10:");

        //Declare a mutable string
        //It needs to be mutable as it will be passed to the stdin().read_line()
        //Which expects a mutable string
        let mut guess : String = String::new();
        
        //Generating a random number between a certain range using the rand crate
        let random_num = rand::thread_rng().gen_range(1..=10);


        //Read a number from the terminal
        //It will be a string
        io::stdin().read_line(&mut guess).expect("Unable to read a guess");
        


        //Parse to int
        //We are also handling the invalid input scenerio
        //In the case of a user entering an invalid number
        //The program will not carry on and continue to the beginning of loop
        let guess : u32 = match guess.trim().parse(){

            Ok(num) => num, //Return the parsed number as `num`
            Err(_) => continue //Jump to the start of loop

        };

        //Using the cmp function block to match different cases
        match guess.cmp(&random_num){

            Ordering::Less => println!("Your guess is low"),
            Ordering::Greater => println!("Your guess is high"),
            Ordering::Equal => {
                println!("\n\n>>> Congratulations you have guessed the number <<<");
                break;
            }

        }

        num_tries += 1;
    }

    println!("It took you `{num_tries}` tries to guess the number");
}
