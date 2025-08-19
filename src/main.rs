use std::{
    collections::{HashSet, VecDeque},
    env::{args, consts::OS, home_dir},
    fs::File,
    io::Write,
    path::PathBuf,
};

const CORRECT_FORMAT: &'static str = "wheres [options]... <filename>";

fn main() {
    let mut output: Option<String> = None;
    let mut recurse = true;
    let mut verbose = false;
    let writer = std::io::stdout();

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

    let mut search_queue = VecDeque::<PathBuf>::new();
    let mut searched = HashSet::<PathBuf>::new();

    let output_file = if let Some(output) = output {
        Some(File::create(output).unwrap())
    } else {
        None
    };

    if let Some(home_dir) = home_dir() {
        search_queue.push_front(home_dir);
    } else {
        match OS {
            "windows" => {
                // yeah i'm not checking for other drives on windows bruh
                search_queue.push_front(PathBuf::from("c:/"));
            }
            "linux" => {
                search_queue.push_front(PathBuf::from("/"));
            }
            _ => {
                println!("Unsupported OS detected. Wheres only supports Linux and W*ndows");
                return;
            }
        }
    }

    while !search_queue.is_empty() {
        let next = search_queue.pop_front().unwrap();

        for item in next.read_dir().unwrap() {
            let item_path = item.unwrap().path();

            if recurse == true && item_path.is_dir() && !searched.contains(&item_path) {
                search_queue.push_front(item_path);
            } else if item_path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .contains(search_query.as_str())
            {
                writeln!(&writer, "{:?}", item_path.to_str().unwrap()).unwrap();
            }
        }

        searched.insert(next);
    }
}

fn short_help() {
    println!("Usage: {}", CORRECT_FORMAT);
    println!("Try 'wheres --help' for more information.");
}

fn full_help() {
    println!("help...");
}
