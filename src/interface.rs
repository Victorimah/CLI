use std::error::Error;
use std::io::{self, Write};
use crate::commands::{file_ops, navigation};

pub fn run_cli() -> Result<(), Box<dyn Error>> {
    println!("Welcome to Ima_CLI! Type 'Help' for a list of commands.");
    let mut clipboard: Option<String> = None;

    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() { continue; }

        match parts[0].to_lowercase().as_str() {
            "write" => file_ops::rename_file(&parts)?,
            "move" => file_ops::move_file(&parts)?,
            "locate" => file_ops::locate_file(&parts)?,
            "cd" => navigation::change_dir(&parts)?,
            "cd.." => navigation::go_back()?,
            "ima" => println!("App updated (simulated)."),
            "remove" => file_ops::remove_file(parts.as_slice())?,
            "help" => print_help(),
            "info" => file_ops::file_info(&parts)?,
            "copy" => file_ops::copy_file(&parts, &mut clipboard)?,
            "paste" => file_ops::paste_file(&parts, &clipboard)?,
            "new" => file_ops::new_file(&parts)?,
            "newd" => file_ops::new_directory(&parts)?,
            "ls" => navigation::list_directory()?,
            "exit" => break,
            _ => println!("Invalid command. Type 'help' for a list of commands."),
        }
    }
    Ok(())
}

fn print_help() {
    println!("Available commands:");
    println!(" write <old_name> <new_name> - Rename a file");
    println!(" move <source> <destination> - Move a file to a new directory"); 
    println!(" locate <file_name> - Find a file in the current path");
    println!(" cd <destination> - Change directory to the specified path");
    println!(" cd.. - Go back to the previous directory");
    println!(" ima - Update the application (simulated)");
    println!(" remove <file_name> - Remove a specified file");
    println!(" info <file_name> - Display information about a file");
    println!(" copy <file_name> - Copy a file (stores in clipboard)");
    println!(" paste <destination> - Paste the copied file to a new location");
    println!(" new <file_name> - Create a new file");
    println!(" newd <directory_name> - Create a new directory");
    println!(" ls - List files and directories in the current directory");
    println!(" help - Show this help message");
    println!(" exit - Exit the CLI");
}