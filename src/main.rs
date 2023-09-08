use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <price_incl_vat> <vat_percentage>", args.get(0).unwrap_or(&"smom".to_string()));
        exit(1);
    }

    let price_incl_vat = parse_arg(&args[1], "price_incl_vat").unwrap_or_else(|e| exit_with_error(e));
    let vat_percentage = parse_arg(&args[2], "vat_percentage").unwrap_or_else(|e| exit_with_error(e));

    if vat_percentage < 0.0 || vat_percentage > 100.0 {
        eprintln!("Error: vat_percentage should be between 0 and 100.");
        exit(1);
    }

    let price_excl_vat = price_incl_vat / (1.0 + vat_percentage / 100.0);
    let vat = price_incl_vat - price_excl_vat;

    println!("\nPrice excl vat: {:.2}", price_excl_vat);
    println!("VAT: {:.2}\n", vat);
}

fn parse_arg(arg: &str, arg_name: &str) -> Result<f64, String> {
    arg.trim().parse().map_err(|_| format!("Error: {} is not a valid number.", arg_name))
}

fn exit_with_error(error: String) -> ! {
    eprintln!("{}", error);
    exit(1);
}
