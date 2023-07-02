use deltalake::operations::create::CreateBuilder;
use deltalake::{DeltaTable, DeltaTableError, SchemaDataType};
use std::error::Error;

// Creates an empty delta lake table with 2 columns
pub async fn create_table(path: &str) -> Result<DeltaTable, DeltaTableError> {
    let create_table_result = CreateBuilder::new()
        .with_location(path)
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
        .await;

    match create_table_result {
        Ok(table) => Ok(table),
        Err(error) => Err(error),
    }
}
