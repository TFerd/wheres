use std::{
    collections::{HashSet, VecDeque},
    env::{args, home_dir},
    fs::File,
    path::PathBuf,
};

const CORRECT_FORMAT: &'static str = "wheres [options]... <filename>";

fn main() {
    let mut output: Option<String> = None;
    let mut recurse = true;
    let mut verbose = false;

    let mut args = args().skip(1);
    let mut inputs = VecDeque::<String>::new();

    while let Some(arg) = args.next() {
        match &arg[..] {
            "-h" => {
                short_help();
                return;
            }
            "--help" => {
                full_help();
                return;
            }
            "-v" | "--verbose" => verbose = true,
            "-r" | "--recursive=false" => recurse = false,
            "-o" | "--output" => {
                if let Some(output_path) = args.next() {
                    output = Some(output_path);
                } else {
                    panic!("No value specified for --output");
                }
            }
            _ => {
                if arg.starts_with('-') || arg.starts_with("--") {
                    println!("Unknown flag {}", arg);
                } else {
                    inputs.push_back(arg);
                }
            }
        }
    }

    let search_query = inputs.pop_front().expect("unable to get search term");

    if inputs.len() > 0 {
        println!("Only one search term allowed. Extra inputs will be ignored.");
    }

    let search_queue = VecDeque::<PathBuf>::new();
    let searched = HashSet::<PathBuf>::new();

    let output_file = if let Some(output) = output {
        Some(File::create(output).unwrap())
    } else {
        None
    };

    // start at home directory then go to root?
    if let Some(home_dir) = home_dir() {
    } else {
    }
}

fn short_help() {
    println!("Usage: {}", CORRECT_FORMAT);
    println!("Try 'wheres --help' for more information.");
}

fn full_help() {
    println!("help...");
}
