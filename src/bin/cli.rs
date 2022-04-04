use roman_numerals::RomanNumber;
use std::io::{stdout, Write};

fn main() {
    let mut s = String::new();
    println!("Welcome to Roman-Arabic number convertor:\n");
    print!("enter number to be converted:\n> ");

    let _ = stdout().flush();
    if let Err(_) = std::io::stdin().read_line(&mut s) {
        eprintln!("Failed to read user input.");
        return;
    };

    s.retain(|c| !c.is_whitespace());

    match convert(&s) {
        Ok(r) => {
            println!("Converted number: {}", r);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

fn convert(s: &str) -> Result<String, String> {
    if let Ok(n) = s.parse::<u32>() {
        return Ok(RomanNumber::from_arab(n)?.to_string());
    }
    if let Ok(_) = s.parse::<f32>() {
        return Err("Decimal numbers are not supported.".into());
    }
    return Ok(RomanNumber::from_string(s)?.to_arab()?.to_string());
}
