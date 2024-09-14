use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct CustomError(String);

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for CustomError {}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args = Cli::parse();
    let result = std::fs::read_to_string(&args.path);
    let content = match result {
        Ok(file_content) => file_content,
        Err(error) => return Err(CustomError(format!("Error reading `{}`: {}", args.path.display(), error)).into()),
    };

    for line in content.lines(){
        if line.contains(&args.pattern){
            println!("{}",line)
        }
    }
    return Ok(());
}
