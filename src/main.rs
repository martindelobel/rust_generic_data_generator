mod json_reader;
mod models;
use crate::json_reader::json_reader::*;
use clap::Parser;
use models::models::*;
use serde_json::error::Error;
fn main() {
    let args_raw = std::env::args().collect::<Vec<String>>();
    println!("args: {:?}", args_raw);

    let args_clap = Args::parse();

    println!("Args Clap: {:?}", args_clap);
    println!("Format: {}", args_clap.format);
    let args_object: Result<Schema, Error> = read_json_file(args_clap.schema);
    match args_object {
        Ok(schema) => {
            println!("Schema: {:?}", schema);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
