use clap::Parser;
use std::path::{PathBuf, Path};
use anyhow::{Ok, Result, bail};

// cli parameters


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
/// description of compiler here
pub struct Args {
    /// Input source file
    #[arg(short, long, value_parser = validate_input)]
    pub input: PathBuf,
    
    /// Directory to ouput compiled project
    #[arg(short, long, value_parser = validate_output)]
    pub output: Option<PathBuf>,
}

// cli function helpers

fn validate_input(s: &str) -> Result<PathBuf> {
    let path = PathBuf::from(s);
    
    if path.exists() {
        if path.is_file() {
            Ok(path)
        } else {
            bail!("Input Path '{}' doesn't lead to a file", s)
        }
    } else {
        bail!("Output Path '{}' does not exist", s)
    }

}

fn validate_output(s: &str) -> Result<PathBuf> {
    let path = PathBuf::from(s);

    if path.exists() {
        if !path.is_file() {
            Ok(path)
        } else {
            bail!("Output Path '{}' leads to a file", s)
        }
    } else {
        bail!("Output Path '{}' does not exist", s)
    }

}

fn get_default_output (input: &Path) -> Result<PathBuf> {
    let mut path:PathBuf = input.to_path_buf();
    path.pop();
    Ok(path)

}

    // main CLI interface functions

pub fn cli_parse() -> Result<(PathBuf, PathBuf)>{
    let args = Args::parse();
    
    let input = args.input;
    let output = args.output.unwrap_or(get_default_output(&input)?);
    Ok((input, output))
}