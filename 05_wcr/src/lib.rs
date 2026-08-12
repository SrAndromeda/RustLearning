use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::ops::AddAssign;

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    #[arg(help = "Input file(s)", default_value = "-")]
    files: Vec<String>,

    #[arg(short = 'l', long = "lines", help = "Show line count")]
    lines: bool,

    #[arg(
        short = 'c',
        long = "bytes",
        help = "Show byte count",
        group = "exclusive"
    )]
    bytes: bool,

    #[arg(short = 'w', long = "words", help = "Show word count")]
    words: bool,

    #[arg(
        short = 'm',
        long = "chars",
        help = "Show character count",
        group = "exclusive"
    )]
    characters: bool,
}

pub fn get_args() -> Result<Args> {
    let mut args = Args::parse();
    // Iterate through all and check if all of them is false
    if [args.lines, args.words, args.bytes, args.characters]
        .iter()
        .all(|v| !v)
    {
        args.lines = true;
        args.bytes = true;
        args.words = true;
    }
    Ok(args)
}

pub fn run(args: Args) -> Result<()> {
    let mut total: FileInfo = FileInfo::new();
    for filename in &args.files {
        let file = open(&filename);
        match file {
            Err(err) => eprint!("{filename}: {}", err.to_string()),
            Ok(reader) => {
                let data = count(reader).unwrap_or_else(|err| {
                    eprintln!("{filename}: {}", err.to_string());
                    FileInfo::new()
                });
                println!(
                    "{}",
                    format_fileinfo(
                        &data,
                        filename,
                        args.lines,
                        args.words,
                        args.bytes,
                        args.characters
                    )
                );
                total += data;
            }
        }
    }
    if args.files.len() > 1 {
        println!(
            "{}",
            format_fileinfo(
                &total,
                "total",
                args.lines,
                args.words,
                args.bytes,
                args.characters
            )
        );
    }
    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

impl FileInfo {
    fn new() -> FileInfo {
        FileInfo {
            num_lines: 0,
            num_words: 0,
            num_bytes: 0,
            num_chars: 0,
        }
    }
}

impl AddAssign for FileInfo {
    fn add_assign(&mut self, rhs: Self) {
        self.num_lines += rhs.num_lines;
        self.num_words += rhs.num_words;
        self.num_bytes += rhs.num_bytes;
        self.num_chars += rhs.num_chars;
    }
}

pub fn count(mut file: impl BufRead) -> Result<FileInfo> {
    let mut info = FileInfo::new();

    let mut buf: String = String::new();
    let mut eof: bool = false;
    while !eof {
        let readed = file.read_line(&mut buf)?; // Lee en unn buffer string el contenido hasta el final de linea, manteniendo si es de Windows o Linux
        if readed > 0 {
            info.num_lines += 1;
            info.num_words += buf.split_whitespace().count();
            info.num_bytes += readed;
            info.num_chars += buf.chars().count();
            buf.clear();
        } else {
            eof = true;
        }
    }

    Ok(info)
}

fn format_fileinfo(
    data: &FileInfo,
    filename: &str,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
) -> String {
    let mut string = String::new();
    if lines {
        string += &format!("{:>8}", data.num_lines);
    }
    if words {
        string += &format!("{:>8}", data.num_words);
    }
    if bytes {
        string += &format!("{:>8}", data.num_bytes);
    }
    if chars {
        string += &format!("{:>8}", data.num_chars);
    }
    if !filename.eq("-") {
        string += &format!(" {}", filename);
    }
    return string;
}

#[cfg(test)]
mod tests {
    use super::{FileInfo, count};
    use std::io::Cursor;
    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }
}
