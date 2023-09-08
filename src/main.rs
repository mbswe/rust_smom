use std::{env, process::exit};

fn main() {
    // Get arguments
    let args: Vec<String> = env::args().collect();

    // Check if enough arguments
    if args.len() <= 2 {
        print!("Usage: smom <price_incl_vat> <vat_percentage>");
        exit(1);
    }

    // Get arguments as variables
    let price_incl_vat = args[1].clone();
    let vat_percentage = args[2].clone();
    
    // Start with no error
    let mut has_error = false;

    // Convert to float and check if error
    let price_incl_vat: f64 = match price_incl_vat.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            has_error = true;
            println!("Error: price_incl_vat is not a number.");
            0.0f64
        }
    };

    // Convert to float and check if error
    let vat_percentage: f64 = match vat_percentage.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            has_error = true;
            println!("Error: vat_percentage is not a number.");
            0.0f64
        }
    };

    // Exit if error
    if has_error {
        exit(1);
    }

    // Calculate
    let price_excl_vat: f64 = price_incl_vat / (1.0 + vat_percentage / 100.0);
    let vat: f64 = price_incl_vat - price_excl_vat;

    // Print
    println!("\nPrice excl vat: {:.2}", price_excl_vat);
    println!("Vat: {:.2}\n", vat);
}
