/*
    Verify that the arguments are valid
    and return a map of the arguments
 */

use std::collections::HashMap;
use std::io::Write;
use std::path::Path;

fn args_invalid(message: &str) {
    println!("Usage: ./rusty-compiler <input file> <output file>");
    println!("{}", message);
}

fn get_args() -> Vec<String> {
    let args: Vec<String> = std::env::args().collect();
    args
}

fn ask_if_overwrite_file() {
    print!("Output file already exists. Overwrite? (y/n) ");
    std::io::stdout().flush().unwrap();
    let mut response = String::new();
    std::io::stdin().read_line(&mut response).unwrap();
    if response.trim() == "n" {
        std::process::exit(0);
    }
}

fn path_is_video(path: String) -> bool {
    match Path::new(&path).extension() {
        Some(extension) => {
            if extension == "mp4" {
                true
            } else {
                false
            }
        },
        None => {
            false
        }
    }
}

fn verify_args(args: Vec<String>) -> HashMap<String, String> {
    let mut arg_map = HashMap::new();

    if args.len() < 1 {
        args_invalid("Not enough arguments");
    }

    if args.len() > 3 {
        args_invalid("Too many arguments");
    }

    let mut arg_iter = args.iter();

    arg_iter.next(); // Skip the program name

    let input_path = arg_iter.next().unwrap();
    let output_path = arg_iter.next().unwrap();

    arg_map.insert("input_path".to_string(), input_path.to_string());
    arg_map.insert("output_path".to_string(), output_path.to_string());
    arg_map.insert("is_video".to_string(), path_is_video(input_path.to_string()).to_string());

    if !Path::new(&arg_map["input_path"]).exists() {
        panic!("Input file does not exist");
    }
    if Path::new(&arg_map["output_path"]).exists() {
        ask_if_overwrite_file();
    }

    arg_map
}

pub(crate) fn parse_args() -> HashMap<String, String> {
    let args = get_args();
    let arg_map = verify_args(args);
    arg_map
}

