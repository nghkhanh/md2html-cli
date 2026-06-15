use clap::Parser;
use std::path::PathBuf;
use std::fs;
use std::error::Error;
mod parser;


fn main() -> Result<(), Box<dyn Error>> {
    let args = CliArgs::parse();

    let md_content = fs::read_to_string(&args.input_path)?;

    let html_output = parser::parse(&md_content);

    fs::write(&args.output_path, html_output)?;

    println!("Successfully converted {:?} to {:?}.", args.input_path, args.output_path);

    Ok(())
}


// fn parse_md(content: &str) -> String {
//     content.to_string()
// }


#[derive(Parser, Debug)]
struct CliArgs {
    input_path: PathBuf,
    output_path: PathBuf,
}
