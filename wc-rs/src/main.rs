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

fn count_chars(text: &String) -> usize {
    return text.chars().count();
}

#[derive(Debug, Parser)]
struct Args {
    #[arg(short = 'c', long)]
    bytes: bool,

    #[arg(short, long)]
    lines: bool,

    #[arg(short, long)]
    words: bool,

    #[arg(short = 'm', long)]
    chars: bool,

    path: Option<String>,
}

fn main() {
    let args = Args::parse();

    if let Some(path) = args.path {
        let contents = read_file(&path);

        if args.bytes {
            let count = count_bytes(&contents);
            println!("{} {}", count, path);
        } else if args.lines {
            let count = count_new_lines(&contents);
            println!("{} {}", count, path);
        } else if args.words {
            let count = count_words(&contents);
            println!("{} {}", count, path);
        } else if args.chars {
            let count = count_chars(&contents);
            println!("{} {}", count, path);
        } else {
            let byte_count = count_bytes(&contents);
            let line_count = count_new_lines(&contents);
            let word_count = count_words(&contents);

            println!("{}   {}  {} {}", line_count, word_count, byte_count, path)
        }
    } else {
        println!("No path given")
    }
}
