use clap::Parser;
use std::path::PathBuf;
use std::fs;

fn main() {
    let args = CliArgs::parse();

    println!("Parsed arguments: {:?}", args);
    println!("Input file path: {:?}", args.input_path);
    println!("Output file path: {:?}", args.output_path);

    let md_content = fs::read_to_string(&args.input_path)
        .expect("Failed to read the input file.");
    
    println!("\n--- File content ---\n{}", md_content);
}


#[derive(Parser, Debug)]
struct CliArgs {
    input_path: PathBuf,
    output_path: PathBuf,
}
