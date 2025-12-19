use clap::Parser;
use std::{
    collections::HashSet,
    fs,
    io::{self, Read},
};

#[derive(Debug, Parser)]
struct Args {
    path: Option<String>,

    #[arg(short, long)]
    out: Option<String>,

    #[arg(short, long)]
    count: bool,

    #[arg(short = 'd', long)]
    repeated: bool,

    #[arg(short = 'u', long = "true")]
    true_unique: bool,
}

fn read_file(path: &String) -> anyhow::Result<String> {
    Ok(fs::read_to_string(path)?)
}

fn write_file(path: &String, contents: &String) -> anyhow::Result<()> {
    fs::write(path, contents)?;
    Ok(())
}

fn read_stdin() -> anyhow::Result<String> {
    let mut contents = String::new();
    io::stdin().read_to_string(&mut contents)?;
    Ok(contents)
}

fn fetch_input(args: &Args) -> anyhow::Result<String> {
    Ok(match &args.path {
        Some(p) => read_file(p)?,
        None => read_stdin()?,
    })
}

fn uniq(input: &String, args: &Args) -> anyhow::Result<()> {
    let mut uniq_content: String = String::new();
    let mut true_uniq_content: String = String::new();
    let mut repeated_content: String = String::new();
    let mut seen: HashSet<String> = HashSet::new();

    for line in input.lines() {
        let is_added = seen.insert(line.to_string());
        if is_added {
            uniq_content.push_str(line);
            uniq_content.push('\n');

            true_uniq_content.push_str(line);
            true_uniq_content.push('\n');
        } else {
            repeated_content.push_str(line);
            repeated_content.push('\n');

            if args.true_unique {
                if let Some(first) = true_uniq_content.find(&line) {
                    let length = line.chars().count();
                    let _ = true_uniq_content.drain(first..first + length + 1);
                }
            }
        }
    }

    let print_string;
    if args.repeated {
        print_string = &repeated_content;
    } else if args.true_unique {
        print_string = &true_uniq_content;
    } else {
        print_string = &uniq_content;
    }

    if args.count {
        for (idx, line) in print_string.lines().enumerate() {
            println!("{} {}", idx + 1, &line);
        }
    } else {
        print!("{}", &print_string);
    }

    if let Some(path) = &args.out {
        uniq_content.truncate(uniq_content.len() - 1);
        write_file(path, &uniq_content)?;
    }

    Ok(())
}

fn cli(args: &Args) -> anyhow::Result<()> {
    let input = fetch_input(&args)?;

    uniq(&input, &args)?;

    Ok(())
}

fn main() {
    let args = Args::parse();

    cli(&args).expect("Error runnin CLI");
}
