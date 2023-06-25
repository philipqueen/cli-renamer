#![allow(unused)]
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};
use clap::Parser;
use tempfile::NamedTempFile;

#[derive(Parser)]
struct Cli {
    /// String pattern we're searching for
    username: String,
    /// Path of file we're searching
    path: std::path::PathBuf,
}

/// not a bad idea to add a test to this
fn main() -> Result<(), Error> {
    let args = Cli::parse();
    println!("username to replace: {}, file: {}", args.username, args.path.display());

    let path = args.path;
    let path_clone = path.clone();
    
    let mut temp_file = NamedTempFile::new()?;

    let content = File::open(path);

    let content= match content {
        Ok(content) => content,
        Err(e) => {panic!("error opening file: {}", e)}
    };

    let buffered = BufReader::new(content);

    println!("Searching for usernames to replace");
    for line in buffered.lines() {
        let line = line.unwrap();
        /// I'd like to do something fun here, like replace the username with a random fake name from a list
        writeln!(temp_file, "{}", line.replace(&args.username, "username"))?;
    }

    println!("Saving file with replaced usernames");
    temp_file.persist(path_clone)?;

    Ok(())

}
