use clap::Parser;
pub mod models {

    #[derive(clap::Parser, Debug)]
    pub struct Args {
        pub format: String,
        pub size: String,
        pub schema: String,
        pub output: String,
    }

    #[derive(Debug, serde::Deserialize)]
    pub struct Schema {
        fields: Vec<Field>,
    }

    #[derive(Debug, serde::Deserialize)]
    struct Field {
        field_name: String,
        field_type: String,
    }
}
