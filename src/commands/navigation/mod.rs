use std::error::Error;
use std::env;
use std::fs;

pub fn change_dir(args: &[&str]) -> Result<(), Box<dyn Error>> {
    if args.len() < 2 {
        println!("Usage: cd <destination>");
        return Ok(());
    }
    env::set_current_dir(args[1])?;
    println!("Changed directory to {}", args[1]);
    Ok(())
}

pub fn go_back() -> Result<(), Box<dyn Error>> {
    env::set_current_dir("..")?;
    println!("Went back one directory.");
    Ok(())
}

pub fn list_directory() -> Result<(), Box<dyn Error>> {
    let current_dir = env::current_dir()?;
    println!("Listing for {}:", current_dir.display());
    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            println!("[DIR]  {}", entry.file_name().to_string_lossy());
        } else {
            println!("       {}", entry.file_name().to_string_lossy());
        }
    }
    Ok(())
}