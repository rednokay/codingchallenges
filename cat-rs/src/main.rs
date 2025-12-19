use clap::Parser;
use std::io::Read;

#[derive(Debug, Parser)]
struct Args {
    paths: Option<Vec<String>>,

    #[arg(short, long)]
    lines: Option<usize>,

    #[arg(short, long)]
    nums: bool,

    #[arg(short = 'b', long)]
    skip: bool,
}

fn read_file(path: &String) -> std::io::Result<String> {
    Ok(std::fs::read_to_string(path)?)
}

fn read_stdin() -> std::io::Result<String> {
    let mut contents = String::new();
    std::io::stdin().read_to_string(&mut contents)?;
    Ok(contents)
}

fn print_lines(contents: &String, count: Option<usize>, nums: bool, skip: bool) {
    let count = match count {
        Some(c) => c,
        None => usize::MAX,
    };

    let mut idx = 0;
    for line in contents.lines() {
        if line.is_empty() & skip {
            println!("");
            continue;
        }

        if nums | skip {
            println!("{}  {}", idx + 1, line);

            if idx == count - 1 {
                return;
            }
            idx += 1;
        } else {
            println!("{}", line);
        }
    }
}

fn cli(args: &Args) -> std::io::Result<()> {
    let mut contents: Vec<String> = Vec::new();

    match &args.paths {
        Some(p) => {
            for path in p {
                contents.push(read_file(path)?);
            }
        }
        None => contents.push(read_stdin()?),
    };

    for ref content in contents {
        print_lines(content, args.lines, args.nums, args.skip);
    }

    Ok(())
}

fn main() {
    let args = Args::parse();

    cli(&args).expect("Error running CLI");
}
