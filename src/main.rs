use clap::Parser;
use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
    process::Command,
};

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
    match Path::new(&file_name).extension().unwrap().to_str().unwrap() {
        "pdf" => (),
        _ => {
            println!("Wrong extension");
            return;
        }
    };
    match Command::new("pdftotext")
        .args(["-layout", &file_name])
        .output()
    {
        Ok(_) => (),
        Err(e) => {
            println!("Error while command was executing: {e}");
            return;
        }
    };

    let mut new_file = PathBuf::from(file_name);
    new_file.set_extension("txt");

    let data = match read_to_string(new_file) {
        Ok(d) => d,
        Err(e) => {
            println!("Error while getting data from text file {e}");
            return;
        }
    };
    println!("{data}");
}
