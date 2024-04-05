use std::{error::Error, path::PathBuf};

use clap::Parser;
use image::{io::Reader as ImageReader, GenericImageView, Pixel, Rgba};
use itertools::Itertools;

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

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let img = ImageReader::open(cli.input)?.decode()?;
    let img = img.resize(
        cli.max_width.unwrap_or(img.width()),
        cli.max_height.unwrap_or(img.height()),
        image::imageops::FilterType::CatmullRom,
    );

    let groups = img
        .pixels()
        .map(|(x, y, pixel)| pixel_to_emoji(x, y, pixel))
        .group_by(|(_x, y, _emoji)| *y);
    let emoji_grid = groups
        .into_iter()
        .map(|(_line_number, line)| line.map(|pixel| pixel.2).collect::<String>())
        .join("\n");

    println!("{emoji_grid}");

    Ok(())
}

fn pixel_to_emoji(x: u32, y: u32, pixel: Rgba<u8>) -> (u32, u32, char) {
    const EMOJIS: &[(char, Rgba<u8>)] = &[
        ('🟦', Rgba([0x5d, 0xad, 0xec, 0])),
        ('🟪', Rgba([0xaa, 0x8e, 0xd6, 0])),
        ('🟧', Rgba([0xff, 0xac, 0x33, 0])),
        ('🟫', Rgba([0x7c, 0x53, 0x3e, 0])),
        ('🟥', Rgba([0xbe, 0x19, 0x31, 0])),
        ('🟨', Rgba([0xfd, 0xcb, 0x58, 0])),
        ('🟩', Rgba([0x78, 0xb1, 0x59, 0])),
        ('⬜', Rgba([0xe6, 0xe7, 0xe8, 0])),
        ('⬛', Rgba([0x29, 0x2f, 0x33, 0])),
        ('▪', Rgba([0, 0, 0, 255])),
    ];

    let best_emoji = EMOJIS
        .iter()
        .min_by_key(|(_, rgba)| {
            rgba.channels()
                .iter()
                .zip(pixel.channels())
                .map(|(&reference, &sample)| {
                    let r = reference as f64;
                    let s = sample as f64;

                    r * r - s * s
                })
                .sum::<f64>()
                .sqrt() as u32
        })
        .unwrap()
        .0;

    (x, y, best_emoji)
}
