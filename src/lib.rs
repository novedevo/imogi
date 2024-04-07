use image::{DynamicImage, GenericImageView, Pixel, Rgba};
use itertools::Itertools;

pub fn image_to_emoji(
    img: DynamicImage,
    max_width: Option<u32>,
    max_height: Option<u32>,
) -> String {
    let img = img.resize(
        max_width.unwrap_or(img.width()),
        max_height.unwrap_or(img.height()),
        image::imageops::FilterType::CatmullRom,
    );

    let groups = img
        .pixels()
        .map(|(x, y, pixel)| pixel_to_emoji(x, y, pixel))
        .group_by(|(_x, y, _emoji)| *y);

    groups
        .into_iter()
        .map(|(_line_number, line)| line.map(|pixel| pixel.2).collect::<String>())
        .join("\n")
}

pub fn pixel_to_emoji(x: u32, y: u32, pixel: Rgba<u8>) -> (u32, u32, char) {
    if pixel.channels()[3] < 127 {
        return (x, y, '▪');
    }
    let hsv_pixel = rgba_to_hsv(pixel);
    const EMOJIS: &[(char, Rgba<u8>)] = &[
        ('🟦', Rgba([0x5d, 0xad, 0xec, 255])),
        ('🟪', Rgba([0xaa, 0x8e, 0xd6, 255])),
        ('🟧', Rgba([0xff, 0xac, 0x33, 255])),
        ('🟫', Rgba([0x7c, 0x53, 0x3e, 255])),
        ('🟥', Rgba([0xbe, 0x19, 0x31, 255])),
        ('🟨', Rgba([0xfd, 0xcb, 0x58, 255])),
        ('🟩', Rgba([0x78, 0xb1, 0x59, 255])),
        ('⬜', Rgba([0xe6, 0xe7, 0xe8, 255])),
        ('⬛', Rgba([0x29, 0x2f, 0x33, 255])),
    ];

    let best_emoji = EMOJIS
        .iter()
        .min_by_key(|(_, rgba)| {
            rgba_to_hsv(*rgba)
                .iter()
                .zip(hsv_pixel)
                .map(|(&reference, sample)| {
                    let r = reference;
                    let s = sample;

                    (r - s) * (r - s)
                })
                .sum::<f64>()
                .sqrt() as u32
        })
        .unwrap()
        .0;

    (x, y, best_emoji)
}

fn rgba_to_hsv(input: Rgba<u8>) -> [f64; 3] {
    let rgb = &input.0[0..3];
    let [r, g, b] = rgb else { unreachable!() };
    let (rf, gf, bf) = (*r as f64, *g as f64, *b as f64);
    let xmax = rgb.iter().max().unwrap();
    let v = xmax;
    let vf = *v as f64;
    let xmin = rgb.iter().min().unwrap();
    let c = xmax - xmin;
    let cf = c as f64;
    // let l = (xmax + xmin) as f64 / 2.0;
    let h = if c == 0 {
        0.0
    } else {
        60.0 * if v == r {
            ((gf - bf) / cf) % 6.0
        } else if v == g {
            (bf - rf) / cf + 2.0
        } else if v == b {
            (rf - gf) / cf + 4.0
        } else {
            unreachable!()
        }
    };

    let sv = if *v == 0 { 0.0 } else { cf / vf };
    [h, sv, vf]
}
