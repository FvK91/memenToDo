#![allow(non_snake_case)]

const DATA_PATH: &str = "./dat/";

use std::env::args;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write};

use glob::glob;

fn make_file_path(file_name: &str) -> String {
    String::from(DATA_PATH.to_owned() + file_name)
}

fn open_file(file_path: &str) -> Option<File> {
    let file = File::open(file_path);
    match file {
        Ok(f) => Some(f),
        Err(e) => {
            println!("Error opening memento: {}", e);
            None
        }
    }
}

struct Memento;

impl Memento {
    fn create(file_name: &str, memento: &str) -> io::Result<()> {
        // Create 'data' directory if not present
        std::fs::create_dir_all(DATA_PATH)?;

        // Create file on disk and write memento
        let mut new_memento = File::create(make_file_path(file_name))?;
        new_memento.write_all(memento.as_bytes())?;

        // Print confirmation message
        println!("Memento has been created: {}", file_name);

        Ok(())
    }

    fn show(file_name: &str) -> io::Result<()> {
        // If empty show list of mementos
        if file_name == "all" {
            println!("{}", "Mementos:");

            let pattern = make_file_path("*");
            for memento in glob(pattern.as_str()).expect("Failed to read glob pattern") {
                match memento {
                    Ok(path) => println!("{:?}", path.file_name().unwrap()),
                    Err(e) => println!("{:?}", e),
                }
            }
            return Ok(());
        }

        // Get file on disk
        let file = open_file(&make_file_path(file_name));
        match file {
            Some(f) => {
                // Read and print memento
                println!("Contents of memento {}:", file_name);
                let buffered = BufReader::new(f);
                for line in buffered.lines() {
                    println!("{}", line?);
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn delete(file_name: &str) -> io::Result<()> {
        // Remove memento after confirmation
        println!("Are you sure you want to delete Memento: {}", file_name);
        println!("Y/N?");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "Y" => {
                std::fs::remove_file(make_file_path(file_name))?;
                println!("Memento deleted: {}", file_name);
            }
            _ => {
                println!("{}", input);
            } // No delete
        }
        Ok(())
    }
}

fn handle_command(command: &str, arg: &str, memento: &str) -> io::Result<()> {
    match command {
        "create" => Memento::create(arg, memento)?,
        "show" => Memento::show(arg)?,
        "delete" => Memento::delete(arg)?,
        _ => println!("{}", "please provide a valid command"), // show
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() > 2 {
        let command = &args[1];
        let arg = &args[2];

        let mut memento = String::new();
        if args.len() > 3 {
            memento = args[3].to_owned();
        }

        match handle_command(command, arg, &memento) {
            Ok(_) => {}
            Err(..) => panic!("{}", "oops"),
        }
    } else {
        println!("please provide a command");
    }
}
