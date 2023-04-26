use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

use clap::Parser;
use ut99_strings::create_string_extractor;

#[derive(Parser, Debug)]
#[command(author, about, long_about = None, arg_required_else_help = true)]
struct Args {
    #[arg(help = "File path to a UT99 DEMO")]
    file: PathBuf,

    #[arg(short, long, default_value_t = 6,
            value_parser = clap::value_parser!(u8).range(1..),
            help = "Minimum string length")]
    length: u8,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let file = File::open(args.file)?;
    let buf_reader = BufReader::new(file);

    let string_extractor = create_string_extractor(buf_reader.bytes().map(|r| r.unwrap()));
    for value in string_extractor {
        if value.len() >= args.length as usize {
            println!("{value}");
        }
    }

    Ok(())
}
