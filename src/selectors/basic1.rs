// Import necessary functions from the selectors module
use crate::selectors::{default_output, get_vec};

// Define the main function that takes HTML data, a boolean flag for tty, and a width
pub fn main(data: &scraper::Html, tty: &bool, w: usize) {
    // Extract text from the HTML data based on the selector
    let x = get_vec(data, "div.HwtpBd.gsrt.PZPZlf.kTOYnf");
    
    // Check if the tty flag is true or false
    match tty {
        // If tty is true, use the default_output function to display the result
        true => default_output(x[0], w),
        
        // If tty is false, simply print the result
        false => println!("{}", x[0])
    }
}
