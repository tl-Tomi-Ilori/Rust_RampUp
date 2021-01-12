//BRINING IN INPUT OUTPUT LIBRARY WHICH COMES FROM STANDARD LIBRARY STD
use std::io;
//Bringing in the rand library and then call the Rng which has traits that random number generators use this is 
use rand::Rng;
//Bringin oin Orderinng type 
use std::cmp::Ordering;


fn main() {
    println!("==Guessing Game==");

    //calls the rand::thread function, whih gives us access to the gen_range method to create a a range where a random nmber is selected 
    //It is inclusive on the lower end and exclusive on the higer band
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop{
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

    //Here we can see Rust allows us to shadow variables let guess:
    //.trim is called in guess removing any whitespace in the input
    //Parse turns a staring into a number but we must speify what kind of number
    //expect is used incase errors are made
    //Expect got changed to match for better error handling
    
    let guess: u32 = match guess.trim().parse() {
        //if parse can convert to a number it returns the num
        Ok(num) => num,
        //if not a number it continues with the loop
        //The underscore is a catchall which which matchjes all ERR values
        Err(_) => continue,
    };

    println!("You guessed {}", guess);
    //prints random number
    println!("The secret number is: {}", secret_number);
    //Match expression
    // call te cmp method on the guess variable and reference it to the secret_number variable 
    match guess.cmp(&secret_number) {
        //Ordering is an enum with 3 values
        //each => is an arm which details what to do if one of these patternes are met
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        //Enda loop if the correct answer is guessed
        Ordering::Equal =>  {
            println!("You win!");
            //ends loop
            break;
        }
    }



}

    }
