use clap::Parser;
use std::fs;

fn read_file(path: &String) -> String {
    return fs::read_to_string(path).expect("File not found");
}

fn count_bytes(text: &String) -> usize {
    return text.len();
}

fn count_new_lines(text: &String) -> usize {
    let mut count: usize = 0;
    for char in text.chars() {
        if char == '\n' {
            count += 1;
        }
    }
    return count;
}

fn count_words(text: &String) -> usize {
    return text.split_whitespace().count();
}

#[derive(Debug, Parser)]
struct Args {
    #[arg(short = 'c', long)]
    bytes: bool,

    #[arg(short, long)]
    lines: bool,

    #[arg(short, long)]
    words: bool,

    path: Option<String>,
}

fn main() {
    let args = Args::parse();

    if let Some(path) = args.path {
        let contents = read_file(&path);

        if args.bytes {
            let count = count_bytes(&contents);
            println!("{} {}", count, path);
        }

        if args.lines {
            let count = count_new_lines(&contents);
            println!("{} {}", count, path);
        }

        if args.words {
            let count = count_words(&contents);
            println!("{} {}", count, path);
        }
    } else {
        println!("No path given")
    }
}
