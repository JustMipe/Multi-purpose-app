use crate::utils::clear;

/// Calling calculator
pub fn calculator() {
    loop {
        clear();
        println!("Welcome in Calculator!");
        println!("\n1. quit");
        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                clear();
                break;
            }
            _ => break,
        }
    }
}