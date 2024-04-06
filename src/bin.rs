use std::path::PathBuf;

use clap::Parser;
use image::{io::Reader as ImageReader, ImageError};
use imoji::image_to_emoji;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Image file to be emojified
    input_file: PathBuf,

    /// Restrict the output emoji grid to a certain width
    #[arg(long, name = "width")]
    max_width: Option<u32>,
    /// Restrict the output emoji grid to a certain height
    #[arg(long, name = "height")]
    max_height: Option<u32>,
    /// output for monospace (default true)
    #[arg(long, short)]
    monospace: Option<bool>,
}

fn main() -> Result<(), ImageError> {
    let cli = Cli::parse();
    let img = ImageReader::open(cli.input_file)?.decode()?;
    let mut emoji_grid = image_to_emoji(img, cli.max_width, cli.max_height);
    if let Some(true) | None = cli.monospace {
        emoji_grid = emoji_grid.replace('▪', "  ")
    }
    println!("{emoji_grid}");
    Ok(())
}
