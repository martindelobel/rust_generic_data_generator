use clap::Parser;

fn main() {
    let my_args = {
        Format::Parquet;
        16;
        "schema_path.yaml";
        "s3a//prd-dct-dlk-gold/";
    };

    println!(my_args);
}

#[derive(Parser, Debug, Clone)]
struct Args {
    format: Format,
    size: String,
    schema: String,
    output: Output
}

enum Format {
    Parquet,
    Delta
}

struct Output {
    path: String,
    file_system: String
}