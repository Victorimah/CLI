use std ::error::Error; // For error handling
use std::path::Path; // For handling file paths
use std::io;
use std::fs; // For file system operations

use clap::builder::EnumValueParser; // For input/output operations

pub fn rename_file(args: &[&str]) -> Result<(), Box<dyn Error>> {
    if args.len() < 3 {
        println!("Usage: write <old_name> <new_name>");
        return Ok(());
    }
    fs::rename(args[1], args[2])?; 
    println!("Renamed {} to {}", args[1], args[2]);
    Ok(())
}

pub fn move_file(args: &[&str]) -> Result<(), Box<dyn Error>> { 
    if args.len() < 3 {
        println!("Usage: move <source> <destination>"); 
        return Ok(());
    }
    fs::rename(args[1], args[2])?;
    println!("Moved {} to {}", args[1], args[2]);
    Ok(())
}

pub fn locate_file(args: &[&str]) -> Result<(), Box<dyn Error>> { 
    if args.len() < 2 {
        println!("Usage: locate <file_name>");
        return Ok(());
    }
    let file_name: &str = args[1];
    let current_dir: std::path::PathBuf = std::env::current_dir()?;
    let mut found: bool = false; 
    for entry in fs::read_dir(&current_dir)? {
        let entry: fs::DirEntry = entry?;
        if entry.file_name() == file_name {
            println!("Found: {}", entry.path().display());
            found = true;
        }
    }
    if !found {
        println!("File '{}' not found in {:?}", file_name, current_dir);
    }
    println!("Looking for file '{}' in {:?}", file_name, current_dir);
    Ok(())
}

pub fn remove_file(args: &[&str]) -> Result<(), Box<dyn Error>> {
    if args.len() < 2 {
        println!("Usage: remove <file_name>");
        return Ok(());
    }
    fs::remove_file(args[1])?; // Remove the specified file
    println!("Removed file: {}", args[1]);
    Ok(())
}

pub fn file_info(args: &[&str]) -> Result<(), Box<dyn Error>> {
    if args.len() < 2 {
        println!("Usage: info <file_name>");
        return Ok(());
    }

    let meta = fs::metadata(args[1])?;
    println!("File: {}", args[1]);
    println!("Size: {} bytes", meta.len());
    println!("Readonly: {}", meta.permissions().readonly());
    println!("Created: {:?}", meta.created()?);
    println!("Modified: {:?}", meta.modified()?);
    println!("Accessed: {:?}", meta.accessed()?);
    Ok(())
}

pub fn copy_file(args: &[&str], clipboard: &mut Option<String>) -> Result<(), Box<dyn Error>> {
    if args.len() < 2 {
        println!("Usage: copy <file_name>");
        return Ok(());
    }
    let file_name = args[1];
    if !Path::new(file_name).exists() {
        println!("File '{}' does not exist.", file_name);
        return Ok(());
    }
    *clipboard = Some(file_name.to_string());
    println!("Copied '{}' to clipboard.", file_name);
    Ok(())
}

pub fn paste_file(args: &[&str], clipboard: &Option<String>) -> Result<(), Box<dyn Error>> {
    if args.len() < 2 {
        println!("Usage: paste <destination>");
        return Ok(());
    }
    let destination = args[1];
    if let Some(file_name) = clipboard {
        let dest_path = Path::new(destination).join(Path::new(file_name).file_name().unwrap());
        fs::copy(file_name, &dest_path)?;
        println!("Pasted '{}' to '{}'.", file_name, dest_path.display());
    } else {
        println!("Clipboard is empty. Use 'copy <file_name>' first.");
    }
    Ok(())
}

pub fn new_file(args: &[&str]) -> Result<(), Box<dyn Error>> {
    if args.len() < 2 {
        println!("Usage: new <file_name>");
        return Ok(());
    }
    let file_name = args[1];
    fs::File::create(file_name)?;
    println!("Created new file: {}", file_name);
    Ok(())
}

pub fn new_directory(args: &[&str]) -> Result<(), Box<dyn Error>> {
    if args.len() < 2 {
        println!("Usage: newd <directory_name>");
        return Ok(());
    }
    let dir_name = args[1];
    fs::create_dir(dir_name)?;
    println!("Created new directory: {}", dir_name);
    Ok(())
}