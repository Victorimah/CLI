mod interface;
mod errors;
mod commands;

fn main() {
    if let Err(e) = interface::run_cli() { 
        println!("Error: {}", e);
    }
}