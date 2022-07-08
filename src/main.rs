use std::io::BufRead;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    path: Option<String>,
}

fn run(content: String) -> anyhow::Result<()> {
    let split = content.split_ascii_whitespace();
    for token in split {
        println!("{}", token);
    }
    Ok(())
}

fn run_prompt() -> anyhow::Result<()> {
    let stdin = std::io::stdin();
    let mut iter = stdin.lock();

    loop {
        let mut str = String::new();
        let read = iter.read_line(&mut str)?;
        if read == 0 {
            break;
        }
        run(str)?;
    }

    Ok(())
}

fn run_file(path: String) -> anyhow::Result<()> {
    let content = std::fs::read_to_string(path)?;
    run(content)
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.path {
        Some(path) => {
            // read from file
            run_file(path)
        }
        None => {
            // read from stdin
            run_prompt()
        }
    }
}
