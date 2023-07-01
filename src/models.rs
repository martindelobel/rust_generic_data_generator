pub mod models {
    #[derive(Debug, serde::Deserialize)]
    pub struct Schema {
        fields: Vec<Field>,
    }

    #[derive(Debug, serde::Deserialize)]
    struct Field {
        fieldName: String,
        fieldType: String,
    }
}
