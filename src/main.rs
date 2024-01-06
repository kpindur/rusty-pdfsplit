use std::{env, error::Error, process::Command, io, fs};

use rusty_pdfsplit::*;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        return Err("Missing arguments!".into());
    }

    let (file_path, parsed_input) = parse_input(&("./".to_owned() + &args[1]));
    
    println!("Split file: {}, into:", file_path);
    for (i, input) in parsed_input.iter().enumerate() {
        println!("{}: {:?}", i, input);
    }
    // Create temporary working folder
    ct_tmp();

    let tmp_file_path = "tmp-".to_owned() + &file_path;
    Command::new("pdftocairo")
        .arg("-pdf")
        .arg(&file_path)
        .arg(&tmp_file_path)
        .output()
        .expect("Failed to remove hyperlinks!");

    for input in parsed_input {
        match magic(&tmp_file_path, input) {
            Ok(_)   => println!("Spell succeded!"),
            Err(_)  => println!("Spell failed!"),
        };
    }

    println!("Finished processing...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Delete temporary working folder
    fs::remove_file(tmp_file_path)?;
    rm_tmp();

    Ok(())
}
