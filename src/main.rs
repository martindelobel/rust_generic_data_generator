use clap::Parser;

fn main() {
    let args_raw = std::env::args().collect::<Vec<String>>();
    println!("args: {:?}", args_raw);

    let args_clap = Args::parse();

    println!("Args Clap: {:?}", args_clap);
}

#[derive(Parser, Debug)]
struct Args {
    format: String,
    size: String,
    schema: String,
    output: String,
}

// enum Format {
//     Parquet,
//     Delta,
// }

// struct Output {
//     path: String,
//     file_system: String,
// }
