use std::path::PathBuf;

use clap::Parser;
use image::{io::Reader as ImageReader, ImageError};
use imoji::image_to_emoji;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Image file to be emojified
    #[arg(short, long, value_name = "FILE")]
    input: PathBuf,

    /// Restrict the output emoji grid to a certain width
    max_width: Option<u32>,
    /// Restrict the output emoji grid to a certain width
    max_height: Option<u32>,
}

fn main() -> Result<(), ImageError> {
    let cli = Cli::parse();
    let img = ImageReader::open(cli.input)?.decode()?;
    let emoji_grid = image_to_emoji(img, cli.max_width, cli.max_height);
    println!("{emoji_grid}");
    Ok(())
}
