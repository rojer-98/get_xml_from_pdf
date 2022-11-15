use clap::Parser;
use std::{io::ErrorKind, path::{Path, PathBuf}, process::Command};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    file_name: String,
}

fn main() {
    let args = Args::parse();
    let file_name = args.file_name;
    let ext = Path::new(&file_name).extension().unwrap().to_str().unwrap();
    match ext {
        "pdf" => (),
        _ => {
            println!("Wrong extension");
            return;
        }
    };

    let output = match Command::new("pdftotext")
        .args(["-layout", &file_name])
        .output()
    {
        Ok(d) => (),
        Err(e) => {
            println!("Error while command was executing: {e}");
            return;
        }
    };

    let mut new_file = PathBuf::from(file_name);
    new_file.set_extension("txt");
    let new_file = new_file.to_str().unwrap().to_string();
    println!("{new_file}");
}
