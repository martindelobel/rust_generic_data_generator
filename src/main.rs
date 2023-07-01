mod json_reader;
mod models;

use std::fs::read_to_string;

use clap::Parser;

fn main() {
    let args_raw = std::env::args().collect::<Vec<String>>();
    println!("args: {:?}", args_raw);

    let args_clap = Args::parse();

    println!("Args Clap: {:?}", args_clap);
    println!("Format: {}", args_clap.format);
    use crate::json_reader::json_reader::read_json_file;
    let args_object = read_json_file(args_clap.schema);
    println!("{:?}", args_object);
}

#[derive(Parser, Debug)]
struct Args {
    format: String,
    size: String,
    schema: String,
    output: String,
}

// enum Format {
//     Parquet,
//     Delta,
// }
// #[derive(Parser, Debug)]
// struct Output {
//     path: String,
//     file_system: String,
// }
