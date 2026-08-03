use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
//Rust version of 'head'
pub struct Args {
    #[arg(
        short = 'n',
        long,
        value_name = "LINES",
        help = "Print count lines of each of the specified files ",
        default_value = "10",
        value_parser = parse_positive_int
    )]
    lines: usize,

    #[arg(
        short = 'c',
        long,
        help = "Print bytes of each of the specified files",
        conflicts_with("lines"),
        value_parser = parse_positive_int
    )]
    bytes: Option<usize>,

    #[arg(help = "Files to read", default_value = "-")]
    files: Vec<String>,
}

pub fn get_args() -> Result<Args> {
    let args = Args::parse();
    Ok(args)
}

pub fn run(args: Args) -> Result<()> {
    let number_of_files = args.files.len();
    for (file_number, filename) in args.files.iter().enumerate() {
        if number_of_files > 1 {
            println!("==> {} <==", filename)
        }
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(reader) => {
                print_file(reader, args.lines, args.bytes)?;
            }
        }
        if number_of_files > file_number + 1 {
            println!();
        }
    }
    Ok(())
}

fn print_file(
    reader: Box<dyn BufRead>,
    lines_to_print: usize,
    bytes_to_print: Option<usize>,
) -> Result<()> {
    if bytes_to_print.is_none() {
        // for (nline, line_text) in reader.read_until(byte, buf).enumerate() {
        //     if nline >= lines_to_print {
        //         break;
        //     };
        //     println!("{}", line_text.unwrap());
        // }
        // Ok(())
        print_file_lines(reader, lines_to_print)
    } else {
        print_file_bytes(reader, bytes_to_print)
    }
}

fn print_file_lines(mut reader: Box<dyn BufRead>, lines_to_print: usize) -> Result<()> {
    let mut buf: String = String::new();
    let mut nlines = 0;
    let mut eof: bool = false;
    while lines_to_print > nlines && !eof {
        let readed = reader.read_line(&mut buf)?; // Lee en unn buffer string el contenido hasta el final de linea, manteniendo si es de Windows o Linux
        nlines += 1;
        if readed > 0 {
            print!("{}", buf);
            buf.clear();
        } else {
            eof = true;
        }
    }
    Ok(())
}

fn print_file_bytes(mut reader: Box<dyn BufRead>, bytes_to_print: Option<usize>) -> Result<()> {
    let size = bytes_to_print.unwrap();
    let mut buff: Vec<u8> = vec![0; size];
    let bytes_read = reader.read(&mut buff);
    match bytes_read {
        Ok(bytes_read) => {
            print!("{}", String::from_utf8_lossy(&buff[..bytes_read])); // Lee bytes pero en caso de algo que no pueda decodificar lo escupe aún asi
            Ok(())
        }
        Err(e) => Err(anyhow::anyhow!(e.to_string())),
    }
}

fn parse_positive_int(val: &str) -> Result<usize> {
    match val.parse::<usize>() {
        Ok(num) if num > 0 => Ok(num),
        _ => Err(anyhow::anyhow!("invalid digit found in string")),
    }
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
