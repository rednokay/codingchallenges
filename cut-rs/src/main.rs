use std::io;
use std::io::{Read, Write};

use clap::Parser;

#[derive(Parser)]
struct Args {
    path: Option<String>,

    #[arg(short, long)]
    field: Option<String>,

    #[arg(short, long)]
    delim: Option<char>,
}

fn read_file(path: &String) -> anyhow::Result<String> {
    Ok(std::fs::read_to_string(path)?)
}

fn read_stdin() -> anyhow::Result<String> {
    let mut contents = String::new();
    std::io::stdin().read_to_string(&mut contents)?;
    Ok(contents)
}

fn get_field(contents: &String, num: &Vec<usize>, delim: char) -> anyhow::Result<()> {
    let mut stdout = io::stdout();
    for line in contents.lines() {
        for (idx, n) in num.iter().enumerate() {
            if let Some(f) = line.split(delim).nth(n - 1) {
                if idx == 0 {
                    write!(stdout, "{}", &f)?;
                } else {
                    write!(stdout, "\t{}", &f)?;
                }
            }
        }
        write!(stdout, "\n")?;
    }

    Ok(())
}

fn cli(args: &Args) -> anyhow::Result<()> {
    let contents = match &args.path {
        Some(p) => read_file(p)?,
        None => read_stdin()?,
    };

    let delim = match args.delim {
        Some(c) => c,
        None => '\t',
    };

    if let Some(field) = &args.field {
        let field: Vec<usize> = field.split(',').map(|s| s.parse().unwrap()).collect();

        get_field(&contents, &field, delim)?;
    }
    Ok(())
}

fn main() {
    let args = Args::parse();

    cli(&args).expect("Error using CLI")
}
