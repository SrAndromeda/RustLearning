use anyhow::Result;
use clap::Parser;

use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Parser, Debug)]
#[command(version, about)]
// Rust version of "cat"
pub struct Args {
    #[arg(
        short = 'n',
        long = "number",
        help = "Number lines",
        group = "numbered"
    )]
    number_lines: bool,

    #[arg(
        short = 'b',
        long = "number-nonblank",
        help = "Number nonblank lines",
        group = "numbered"
    )]
    number_nonblank: bool,

    #[arg(help = "Input file(s) [default: -]", default_value = "-")]
    files: Vec<String>,
}

pub fn get_args() -> Result<Args> {
    let args = Args::parse();
    Ok(args)
}

pub fn run(args: Args) -> Result<()> {
    for filename in args.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(reader) => read_from_reader(reader, args.number_lines, args.number_nonblank)?,
        }
    }
    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn read_from_reader(
    reader: Box<dyn BufRead>,
    number_lines: bool,
    number_nonblank: bool,
) -> Result<()> {
    let mut nline = 1;
    for line in reader.lines() {
        let this_line = line?;
        if number_lines || (number_nonblank && !this_line.is_empty()) {
            println!("{number:>6}\t{text}", number = nline, text = this_line);
            nline = nline + 1;
        } else {
            println!("{}", this_line);
        }
    }
    Ok(())
}
