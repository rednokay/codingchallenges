use clap::Parser;
use std::fs;

fn read_file(path: &String) -> String {
    return fs::read_to_string(path).expect("File not found");
}

fn count_bytes(text: &String) -> usize {
    return text.len();
}

#[derive(Debug, Parser)]
struct Args {
    #[arg(short = 'c', long)]
    bytes: bool,

    #[arg(short, long)]
    lines: bool,

    path: Option<String>,
}

fn cli(args: Args) {
    if args.bytes {
        if let Some(path) = args.path {
            let contents = read_file(&path);
            let count = count_bytes(&contents);
            println!("{} {}", count, path);
        }
    }
    if args.lines {}
}

fn main() {
    let args = Args::parse();

    if args.path == None {
        println!("No path given");
    } else {
        cli(args);
    }
}
