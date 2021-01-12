//BRINING IN INPUT OUTPUT LIBRARY WHICH COMES FROM STANDARD LIBRARY STD
use std::io;


fn main() {
    println!("==Guessing Game==");

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

}
