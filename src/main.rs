use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    file_name: String,

    /// Number of times to greet
    #[arg(short, long)]
    command: String,
}

fn main() {
    let args = Args::parse();
}
