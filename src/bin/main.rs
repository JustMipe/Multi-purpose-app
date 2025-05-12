use app::ui::menu::main_menu;
use app::utils::{input, clear};
use app::calc::calculator::calculator;

fn main() {
    clear();  // Clear whole terminal for first use

    // Main loop
    loop {
        main_menu();  // Show main menu
        let choice = input();  // Receiving input from user
 
        // Matching user's input
        match choice.trim() {
            // choice 'calculator'
            "1" => calculator(),
            // choice 'quit'
            "2" => {
                clear();
                println!("PROGRAM CLOSED!");
                break;
            }  
            // non-existent choice
            _ => {
                clear();
                println!("ERROR: Wrong action!");
            }
        }
    }
}
