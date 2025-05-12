use std::io::{self, Write};
use std::process::Command;


/// Clear terminal like 'cls'
pub fn clear() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}

/// Let user make input
pub fn input() -> String {
    print!("\nYour action -> ");
    io::stdout().flush().unwrap();
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}
