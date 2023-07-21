mod delta_lake;
mod json_reader;
mod models;
mod parquet_handler;
mod parquet_writer;
use crate::json_reader::json_reader::*;
use clap::Parser;
use models::models::*;
use polars::df;
use serde_json::error::Error;
use polars::prelude::NamedFrom;
use crate::parquet_writer::parquet_writer::write_in_parquet;

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
/* 
    let result = rt.block_on(create_table(&path));

    match result {
        Ok(table) => {
            println!("Table created successfully: {:?}", table);
        }
        Err(error) => {
            eprintln!("Error creating table: {:?}", error);
        }
    } */

    let df = df!(
        "foo" => &[1, 2, 3],
        "bar" => &[None, Some("bak"), Some("baz")],
    )
    .unwrap();

    rt.block_on(write_in_parquet(&path, df))
}

/*
mod delta_lake;
mod json_reader;
mod models;
mod parquet_handler;
use crate::delta_lake::delta_lake::*;
use crate::json_reader::json_reader::*;
use clap::Parser;
use models::models::*;
use serde_json::error::Error;
use deltalake::writer::{DeltaWriter, RecordBatchWriter};
use deltalake::{action::SaveMode, DeltaOps, SchemaDataType, SchemaField};
use deltalake::*;
use crate::parquet::{
    basic::{Compression, ZstdLevel},
    file::properties::WriterProperties,
};
use std::sync::Arc;
use crate::arrow::{
    array::{Int32Array, StringArray},
    datatypes::{DataType, Field, Schema as ArrowSchema},
    record_batch::RecordBatch,
};

fn get_table_batches() -> RecordBatch {
    let schema = Arc::new(ArrowSchema::new(vec![
        Field::new("board_id", DataType::Int32, false),
        Field::new("board_name", DataType::Utf8, false),
    ]));

    let str_values = StringArray::from(vec!["A", "B", "A", "B", "A", "A", "A", "B", "B", "A", "A"]);
    let int_values = Int32Array::from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);

    RecordBatch::try_new(schema, vec![Arc::new(int_values), Arc::new(str_values)]).unwrap()
}

async fn create_initialized_table(table_path: &String) -> DeltaTable {
    DeltaOps::try_from_uri(table_path)
        .await
        .unwrap()
        .create()
        .with_column(
            "board_id",
            SchemaDataType::primitive(String::from("integer")),
            false,
            Default::default(),
        )
        .with_column(
            "board_name",
            SchemaDataType::primitive(String::from("string")),
            false,
            Default::default(),
        )
        .await
        .unwrap()
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    let args_raw = std::env::args().collect::<Vec<String>>();
    println!("args: {:?}", args_raw);

    let args_clap = Args::parse();

    println!("Args Clap: {:?}", args_clap);
    println!("Format: {}", args_clap.format);
    let args_object: Result<DataSchema, Error> = read_json_file(args_clap.schema);

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


    let maybe_table = deltalake::open_table(&path).await;

    let mut table = match maybe_table {
        Ok(table) => table,
        Err(DeltaTableError::NotATable(_)) => {
            println!("It doesn't look like our delta table has been created");
            create_initialized_table(&path).await
        }
        Err(err) => Err(err).unwrap(),
    };


    let writer_properties = WriterProperties::builder()
    .set_compression(Compression::ZSTD(ZstdLevel::try_new(3).unwrap()))
    .build();

    let mut writer = RecordBatchWriter::for_table(&table)
    .expect("Failed to make RecordBatchWriter");

    let batch = get_table_batches();

    let result = writer.write(batch).await;


    let version = writer
    .flush_and_commit(&mut table)
    .await
    .expect("Failed to flush write");
    println!("{} version written", version);


    // let parquet_handler = ParquetHandler {
    //     path: args_clap.output,
    // };

    // parquet_handler.writeParquet();
    Ok(())
}
 */
