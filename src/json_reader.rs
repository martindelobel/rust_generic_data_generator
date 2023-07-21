pub mod json_reader {
    use crate::models::models::Schema;

    pub fn read_json_file(file_path: String) -> Result<Schema, serde_json::Error> {
        let file_content = std::fs::read_to_string(file_path).unwrap();
        let generic_object = serde_json::from_str(&file_content);
        generic_object
    }
}
