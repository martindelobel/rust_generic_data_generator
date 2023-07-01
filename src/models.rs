pub mod models {
    #[derive(Debug, serde::Deserialize)]
    pub struct Schema {
        fields: Vec<Field>,
    }

    impl Schema {
        pub fn get_fields(&self) -> &Vec<Field> {
            &self.fields
        }
    }

    #[derive(Debug, serde::Deserialize)]
    struct Field {
        fieldName: String,
        fieldType: String,
    }
}
