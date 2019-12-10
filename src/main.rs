use std::env;
use std::process;

fn main() {
    if env::args().len() == 1{
        println!("Usage: armstrong [integers...]")
    }
    
    for test_armstrong in env::args().skip(1){
        
        let num_digits = test_armstrong.chars().count() as u32;
        let test_armstrong_number = test_armstrong.parse::<u32>().expect("Not a number.");
        
        //sum of digits to the power of number of digits
        let armstrong_check: u32 = test_armstrong.chars().map(
            |c| c.to_digit(10).unwrap().pow(num_digits)
        ).sum();
        
        if test_armstrong_number==armstrong_check {
            println!("The number {} is an armstrong number.", test_armstrong)
        }else{
            println!("The number {} is not an armstrong number.", test_armstrong)
        }
    }
    process::exit(0);
}