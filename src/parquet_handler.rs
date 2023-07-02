pub mod parquet_handler {
    #[derive(Debug)]
    pub struct ParquetHandler {
        pub path: String,
    }
    impl ParquetHandler {
        pub fn writeParquet(&self) {
            use arrow_array::RecordBatch;
            use arrow_array::{ArrayRef, Int32Array};
            use parquet::arrow::arrow_writer::ArrowWriter;
            use parquet::file::properties::WriterProperties;
            use std::fs::File;
            use std::sync::Arc;
            let ids = Int32Array::from(vec![1, 2, 3, 4]);
            let vals = Int32Array::from(vec![5, 6, 7, 8]);
            let batch = RecordBatch::try_from_iter(vec![
                ("id", Arc::new(ids) as ArrayRef),
                ("val", Arc::new(vals) as ArrayRef),
            ])
            .unwrap();

            let file = File::create(self.path.to_owned()).unwrap();

            let mut writer = ArrowWriter::try_new(file, batch.schema(), None).unwrap();

            writer.write(&batch).expect("Writing batch");

            writer.close().unwrap();
        }
    }
}
