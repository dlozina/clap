#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use clap::Parser;
use napi::bindgen_prelude::{Error as NapiError, Result};


/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The path to the file to read
    // path: std::path::PathBuf,
    path: String,
    /// The pattern to look for
    pattern: String,
}

#[napi]
pub fn sum(a: f64, b: f64) -> f64 { a + b }

#[napi]
async fn run(path: String, pattern: String) -> Result<()> {
    // This will not work -> Node.js is parsing the commands
    //let args = Cli::parse();

    // let content = std::fs::read_to_string(&args.path)
    //     .with_context(|| format!("could not read file `{}`", args.path.display()))?;
    println!("This is your path: {}", path);
    println!("This is your pattern: {}", pattern);

    //find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}


