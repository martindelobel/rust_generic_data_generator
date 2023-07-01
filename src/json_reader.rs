pub mod json_reader {

    #[derive(Debug, serde::Deserialize)]
    pub struct Schema {
        fields: Vec<Field>,
    }

    #[derive(Debug, serde::Deserialize)]
    struct Field {
        fieldName: String,
        fieldType: String,
    }

    pub fn read_json_file(file_path: String) -> Schema {
        let file_content = std::fs::read_to_string(file_path).unwrap();
        let generic_object: Schema =
            serde_json::from_str(&file_content).expect("JSON file is not properly formatted.");
        generic_object
    }
}
