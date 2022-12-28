use clap::Parser;
use eyre::{eyre, Result};
use image::{ImageBuffer, Rgb};
use mandelbrot2::*;

fn main() -> Result<()> {
    let config = config::Config::try_parse()?;

    let frame_data = render::render(&config)?;
    let img_buffer = ImageBuffer::<Rgb<u8>, _>::from_vec(
        config.image_size.0 as u32,
        config.image_size.1 as u32,
        frame_data,
    )
    .ok_or_else(|| eyre!("malformed render buffer!"))?;

    img_buffer.save(config.output)?;

    Ok(())
}
