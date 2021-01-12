//BRINING IN INPUT OUTPUT LIBRARY WHICH COMES FROM STANDARD LIBRARY STD
use std::io;
//Bringing in the rand library and then call the Rng which has traits that random number generators use this is 
use rand::Rng;



fn main() {
    println!("==Guessing Game==");

    //calls the rand::thread function, whih gives us access to the gen_range method to create a a range where a random nmber is selected 
    //It is inclusive on the lower end and exclusive on the higer band
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Enter a number");

    //Creating a new varibale which is an instance of a String 
    let mut guess =String::new();

    //calls the stdin function from the io module
    io::stdin()
    //cals the read line method to handle use input
    //& makes a refernce to guess variale storing the user input into this variable 
        .read_line(&mut guess)
        //used to handle ppotential errors if the Result type returns an ERR 
        .expect("Faied to read line");

    print!("You guessed {}", guess);

    //prints random number
    println!("The secret number is: {}", secret_number);

}
