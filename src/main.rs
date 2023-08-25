mod delta_lake;
mod json_reader;
mod models;
mod parquet_handler;
mod parquet_writer;
mod delta_writer;
use crate::json_reader::json_reader::*;
use clap::Parser;
use models::models::*;
use polars::df;
use serde_json::error::Error;
use polars::prelude::NamedFrom;
use crate::parquet_writer::parquet_writer::write_in_parquet;
use crate::delta_writer::delta_writer::write_in_delta;

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
    use tokio::runtime::Runtime;
    let rt = Runtime::new().unwrap();

    let path = args_clap.output;

    let df = df!(
        "foo" => &[1, 2, 3],
        "bar" => &[None, Some("bak"), Some("baz")],
    )
    .unwrap();

    rt.block_on(write_in_delta(&path, df))
}

