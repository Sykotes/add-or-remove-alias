use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

fn get_prefix_before_quote(string: &str) -> &str {
    let mut iter = string.split('"');
    if let Some(prefix) = iter.next() {
        return prefix;
    }
    string
}

fn line_exists_in_file(_alias_line: &str, _alias_name: &str) -> io::Result<bool> {
    let home_dir = env::var("HOME").expect("Failed to retrieve home directory.");
    let file = File::open(format!("{}/.zshrc", home_dir))?;
    // let file = File::open("test.txt")?;
    let reader = BufReader::new(file);
    let alias_prefix = get_prefix_before_quote(_alias_line);
    for line in reader.lines() {
        if let Ok(line) = line {
            if line.starts_with(alias_prefix) {
                println!("Alias \"{}\" already exists", _alias_name);
                return Ok(true);
            }
        }
    }

    Ok(false)
}

fn write_alias(_alias_line: &str) -> io::Result<()> {
    let home_dir = env::var("HOME").expect("Failed to retrieve home directory.");
    let mut file = OpenOptions::new()
        .append(true)
        .open(format!("{}/.zshrc", home_dir))?;
    // .open(format!("test.txt"))?;

    file.write_all(_alias_line.as_bytes())?;
    file.write_all(b"\n")?;

    Ok(())
}

fn add_alias() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("\nExample: addalias test \"cd test\"\nthis adds to your .zshrc in your home dir the line: alias test=\"cd test\"");
        return Ok(());
    }
    let alias_name: &String = &args[2];

    if alias_name.to_lowercase() != "--help" || alias_name.to_lowercase() != "help" {
        let alias_command: &String = &args[3];
        println!(
            "Alias name: {}\nAlias command: {}",
            alias_name, alias_command
        );
        let alias_line = "alias ".to_string() + alias_name + "=\"" + alias_command + "\"";
        let line_exists = line_exists_in_file(&alias_line, &alias_name)?;
        if !line_exists {
            write_alias(&alias_line)?;
            println!("Alias added");
        }
    } else {
        println!("\nExample: addalias test \"cd test\"\nthis adds to your .zshrc in your home dir the line: alias test=\"cd test\"");
    }
    Ok(())
}

fn remove_line(arg: &str) -> Result<(), std::io::Error> {
    let home_dir = env::var("HOME").expect("Failed to retrieve home directory.");
    let filename = format!("{}/.zshrc", home_dir);
    // let filename = format!("test.txt");
    // Open the input file for reading
    let file = File::open(filename.clone())?;
    let reader = BufReader::new(file);

    // Create a temporary file for writing
    let temp_filename = format!("{}_temp", filename);
    let temp_file = File::create(&temp_filename)?;

    // Open the temporary file for writing
    let mut writer = std::io::BufWriter::new(temp_file);

    // Iterate over each line in the input file
    for line in reader.lines() {
        let line = line?;

        // Check if the line starts with "your alias"
        if !line.starts_with(arg) {
            // Write the line to the temporary file
            writeln!(writer, "{}", line)?;
        }
    }

    // Flush and sync the writer to ensure all data is written to the file
    writer.flush()?;
    writer.get_mut().sync_all()?;

    // Replace the input file with the temporary file
    std::fs::rename(&temp_filename, filename)?;

    Ok(())
}

fn rm_alias() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 || args.len() > 3 {
        println!("FAILED:\nExample use: rmalias myalias");
        return Ok(());
    }
    let arg: &String = &args[2];
    let alias_from_arg = "alias ".to_string() + arg + "=";
    if let Err(err) = remove_line(&alias_from_arg) {
        eprintln!("Error: {}", err);
    } else {
        println!("Alias \"{}\" has been removed", arg);
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg: &String = &args[1];
    if arg.to_lowercase() == "rm" {
        _ = rm_alias();
    } else if arg.to_lowercase() == "add" {
        _ = add_alias();
    } else {
        println!("no");
    }
}
