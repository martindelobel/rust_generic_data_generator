mod delta_lake;
mod json_reader;
mod models;
mod parquet_handler;
use crate::delta_lake::delta_lake::*;
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
    use tokio::runtime::Runtime;

    let mut rt = Runtime::new().unwrap();

    let path = args_clap.output;

    let result = rt.block_on(create_table(&path));

    match result {
        Ok(table) => {
            println!("Table created successfully: {:?}", table);
        }
        Err(error) => {
            eprintln!("Error creating table: {:?}", error);
        }
    }

    // let parquet_handler = ParquetHandler {
    //     path: args_clap.output,
    // };

    // parquet_handler.writeParquet();
}
