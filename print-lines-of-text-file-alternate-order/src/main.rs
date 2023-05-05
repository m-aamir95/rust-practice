//Importing creates
use std::fs;



fn main() {
    
    let filename = "./some_data_file.txt";

    // Doing the task the Rusty way
    println!("\n\n>>>> Doing things the Rusty way <<<< \n\n");
    match fs::read_to_string(filename){
        Ok(contents) => {

           //filtering based upon odd index, only print odd lines
           println!("------ Printing the filtered read file data ------");
           contents
                 .trim()
                .lines()
                .enumerate()
                .filter(|(index, _)| index % 2 == 0)
                .for_each(|(_, line)| println!("{}", line));

            println!("------ Now printing the original read file data ------");
            println!("{}", contents);
        },
        Err(e) => println!("An Error Occurrd -> {}", e)
    };


    //Doing the things the more imperative way, more verbose code
    println!("\n\n>>>> Doing things the NonRusty way <<<< \n\n");
    let file_contents = fs::read_to_string(filename);

    if let Ok(contents) = file_contents{

        let trimmed = contents.trim();
        let parts = contents.split("\n");
        for (index, p) in parts.enumerate(){
            if index % 2 == 0{
                println!("{}", p);
            }
        }

        println!("------ Now printing the original read file data ------");
        println!("{}", contents);

    }else if let Err(e) = file_contents{
        println!("An Error Occurrd -> {}", e);

    }else{
        println!("I dont't know man something weired happened");
    }

}
