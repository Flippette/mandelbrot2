use clap::Parser;
use eyre::{eyre, Result};
use image::RgbImage;
use mandelbrot2::{config::Config, render::render};
use std::time::Instant;

fn main() -> Result<()> {
    let config = Config::try_parse()?;

    let frame_data = match render(&config) {
        Ok(data) => data,
        Err(e) => panic!("{e:?}"),
    };

    let timer = Instant::now();

    println!("[info] saving render buffer to image...");

    let image = RgbImage::from_vec(config.image_size.0, config.image_size.1, frame_data)
        .ok_or_else(|| eyre!("failed to create internal image!"))?;
    image.save(config.output)?;

    println!(
        "[info] saving finished in {}ms.",
        timer.elapsed().as_millis()
    );

    Ok(())
}
