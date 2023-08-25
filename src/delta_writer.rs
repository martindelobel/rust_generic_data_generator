pub mod delta_writer {
    use deltalake::{operations::create::{CreateBuilder, self}, SchemaDataType};
    use polars::prelude::{DataFrame, DataType};

    
    pub async fn write_in_delta(path: &str, df: DataFrame) {
        let mut create_builder = CreateBuilder::new()
            .with_location(path);

        let columns_type = df.dtypes();
        let columns_name = df.get_column_names();


        for n in 1..columns_name.len() {
            create_builder = create_builder.with_column(columns_name[n],
                 convert_data_type(columns_type[n].clone()),
                 true, Default::default())
        }

        create_builder.await;


    }

    fn convert_data_type(column_type: DataType) -> SchemaDataType {
        return SchemaDataType::primitive(String::from("integer"));
    }
}