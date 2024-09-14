use clap::{builder::Str, Parser};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args = Cli::parse();
    let result = std::fs::read_to_string(&args.path);
    let content = match result{
        Ok(file_content) => {file_content},
        Err(error) => { return Err(error.into()); }
    };

    for line in content.lines(){
        if line.contains(&args.pattern){
            println!("{}",line)
        }
    }
    return Ok(());
}
