pub mod parquet_writer {
    use polars::prelude::{DataFrame, ParquetWriter};


    pub async fn write_in_parquet(path: &str, mut df: DataFrame) {
        let mut file = std::fs::File::create(path).unwrap();
        ParquetWriter::new(&mut file).finish(&mut df).unwrap();
    }
}