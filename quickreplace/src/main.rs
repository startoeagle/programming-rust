use std::process::{exit};

use regex::Regex;
use text_colorizer::*;

fn print_usage() {
    eprintln!(
        "Usage: {} <string to find> <string to replace with> <input filename> <output filename>",
        "quickreplace".green()
    );
}

#[derive(Debug)]
struct Arguments<'a> {
    find: &'a String,
    replace: &'a String,
    input_filename: &'a String,
    output_filename: &'a String,
}

fn replace(target: &String, replacement: &String, input: &String) -> Result<String, regex::Error> {
    let regex = Regex::new(target.as_str())?;
    Ok(regex.replace_all(input, replacement).to_string())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 4 {
        print_usage();
        exit(1);
    }

    let arguments = Arguments {
        find: &args[0],
        replace: &args[1],
        input_filename: &args[2],
        output_filename: &args[3],
    };

    let mut data = match std::fs::read_to_string(&arguments.input_filename) {
        Err(e) => {
            eprintln!(
                "Failed to read {}, {}: {:?}",
                arguments.input_filename,
                "Error".red().bold(),
                e.to_string()
            );
            exit(1);
        }
        Ok(input) => input,
    };

    data = match replace(arguments.find, arguments.replace, &data) {
        Err(e) => {
            eprintln!(
                "could not replace contents. {}: {:?}",
                "Error".red().bold(),
                e
            );
            exit(2);
        }
        Ok(s) => s,
    };

    match std::fs::write(arguments.output_filename, &data) {
        Err(e) => {
            eprintln!(
                "Failed to read {}, {}: {:?}",
                arguments.input_filename,
                "Error".red().bold(),
                e.to_string()
            );
            exit(1);
        }
        Ok(_) => (),
    };
}
