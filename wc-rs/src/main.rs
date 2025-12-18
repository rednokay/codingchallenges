use clap::Parser;
use std::fs;
use std::io;
use std::io::Read;

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

#[derive(Debug, Parser, Clone)]
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

fn cli(args: &Args, contents: String, path: Option<&String>) {
    let count: usize;
    if args.bytes {
        count = count_bytes(&contents);
    } else if args.lines {
        count = count_new_lines(&contents);
    } else if args.words {
        count = count_words(&contents);
    } else if args.chars {
        count = count_chars(&contents);
    } else {
        let byte_count = count_bytes(&contents);
        let line_count = count_new_lines(&contents);
        let word_count = count_words(&contents);

        if path.is_none() {
            println!("{}   {}  {}", line_count, word_count, byte_count);
        } else {
            println!(
                "{}   {}  {} {}",
                line_count,
                word_count,
                byte_count,
                path.unwrap()
            );
        }
        return;
    }

    if path.is_none() {
        println!("{}", count);
    } else {
        println!("{} {}", count, path.unwrap());
    }
}

fn main() {
    let args = Args::parse();

    if let Some(path) = &args.path {
        let contents = read_file(path);

        cli(&args, contents, Some(path));
    } else {
        let mut contents = String::new();
        io::stdin()
            .read_to_string(&mut contents)
            .expect("Error processing std in");

        cli(&args, contents, None);
    }
}
