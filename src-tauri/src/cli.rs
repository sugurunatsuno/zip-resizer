use std::path::PathBuf;
use clap::Parser;
use zip_resizer_lib::{process_zip, ResizeOptions};

/// Zip image resizer CLI
#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Input zip file
    #[arg(short, long)]
    input: PathBuf,
    /// Output zip file
    #[arg(short, long)]
    output: PathBuf,
    /// Maximum width of images
    #[arg(long)]
    max_width: Option<u32>,
    /// Maximum height of images
    #[arg(long)]
    max_height: Option<u32>,
    /// JPEG quality (1-100)
    #[arg(long, default_value_t = 80)]
    quality: u8,
}

fn main() {
    let args = Args::parse();
    let opts = match ResizeOptions::new(args.max_width, args.max_height, args.quality) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("Invalid options: {e}");
            std::process::exit(1);
        }
    };
    if let Err(e) = process_zip(&args.input, &args.output, &opts) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
