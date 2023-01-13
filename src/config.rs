use clap::Parser;
use eyre::{ensure, Result};
use std::{error::Error, path::PathBuf, str::FromStr};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    // image size in pixels
    #[arg(
        short = 'i',
        long,
        value_parser = parse_comma_pair::<u64>
    )]
    pub image_size: (u32, u32),

    // viewport size in units
    #[arg(
        short = 'v',
        long,
        value_parser = parse_comma_pair::<f64>
    )]
    pub viewport_size: (f64, f64),

    // viewport displacement in units
    #[arg(
        short = 'd',
        long,
        value_parser = parse_comma_pair::<f64>
    )]
    pub viewport_displacement: (f64, f64),

    // maximum trace steps
    #[arg(short = 's', long, default_value_t = 255)]
    pub max_steps: u8,

    // output image path
    #[arg(short = 'o', long, default_value = "image.jpeg")]
    pub output: PathBuf,

    // LUT path
    #[arg(short = 'l', long, default_value = "default.lut")]
    pub lut: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            image_size: (256, 256),
            viewport_size: (1.0, 1.0),
            viewport_displacement: (0.0, 0.0),
            max_steps: 255,
            output: PathBuf::from_str("image.jpeg").unwrap(),
            lut: PathBuf::from_str("default.lut").unwrap(),
        }
    }
}

fn parse_comma_pair<T>(s: &str) -> Result<(T, T)>
where
    T: FromStr,
    T::Err: Error + Send + Sync + 'static,
{
    let tokens = s.split(',').collect::<Vec<_>>();
    ensure!(
        tokens.len() == 2,
        "comma-separated arguments must come in a pair!"
    );

    Ok((tokens[0].parse()?, tokens[1].parse()?))
}
